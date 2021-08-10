use anyhow::{anyhow, Result};
use clap::{crate_version, Clap};
use futures::{stream, StreamExt};
use glob::glob;
use grep_regex::RegexMatcher;
use log::{debug, info, warn};
use notify::event::{DataChange, ModifyKind};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pyaco_core::InputType;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::sync::{mpsc::channel, Arc};
use tokio::sync::Mutex;

use crate::lib::{extra_classes_from_path, open_file};

mod lib;

#[derive(Clap, Debug)]
#[clap(name = "pyaco-validate", version = crate_version!())]
struct Options {
    /// CSS file path or URL used for code verification
    #[clap(short, long)]
    css_input: String,

    /// Glob pointing to the files to validate
    #[clap(short, long)]
    input_glob: String,

    /// Classes matcher regex, must include a capture containing all the classes
    #[clap(long, default_value = r#"class="([^"]+)""#)]
    capture_regex: String,

    /// Classes splitter regex, will split the string captured with the `capture_regex` argument and split it into classes
    #[clap(long, default_value = r#"\s+"#)]
    split_regex: String,

    /// How many files can be read concurrently at most, setting this value to a big number might break depending on your system
    #[clap(long, default_value = "128")]
    max_opened_files: usize,

    /// Watch for changes in the provided css file and the directory then revalidate the code (doesn't work with URL)
    #[clap(short, long)]
    watch: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Options {
        css_input,
        input_glob,
        capture_regex,
        split_regex,
        max_opened_files,
        watch,
    } = Options::parse();

    let capture_regex = Arc::new(RegexMatcher::new(capture_regex.as_str())?);

    let split_regex = Arc::new(Regex::new(split_regex.as_str())?);

    info!("Validating {} against {}", input_glob, css_input);

    let css_input = InputType::from_path(css_input);

    // The classes contained in the provided css file/URL
    let accepted_classes = css_input.extract_classes()?;

    // Always run at least once, even in watch mode
    let valid = run(
        input_glob.as_str(),
        &accepted_classes,
        capture_regex.clone(),
        split_regex.clone(),
        max_opened_files,
    )
    .await;

    if let Some(watch_dir) = watch {
        run_watch(
            &css_input,
            watch_dir.as_str(),
            input_glob.as_str(),
            &accepted_classes,
            capture_regex,
            split_regex,
            max_opened_files,
        )
        .await?
    }

    valid?;

    Ok(())
}

async fn run(
    input_glob: &str,
    accepted_classes: &HashSet<String>,
    capture_regex: Arc<RegexMatcher>,
    split_regex: Arc<Regex>,
    max_opened_files: usize,
) -> Result<()> {
    let glob = glob(input_glob)?;
    // Open and extract classes from files concurrently
    let jobs = stream::iter(glob)
        .filter_map(|path| async move {
            match path {
                Ok(path) => open_file(path).await,
                Err(_) => None,
            }
        })
        .map(|file| {
            let split_regex = split_regex.clone();
            let capture_regex = capture_regex.clone();
            tokio::spawn(extra_classes_from_path(file, capture_regex, split_regex))
        })
        .buffer_unordered(max_opened_files);
    let found_classes = Mutex::new(HashSet::new());
    // Insert the classes captured into the `found_classes` set
    jobs.for_each(|job| async {
        let mut found_classes = found_classes.lock().await;
        if let Ok(Ok(classes)) = job {
            for class in classes {
                found_classes.insert(class);
            }
        }
    })
    .await;
    let found_classes = found_classes.lock().await;
    // Diff between whitelisted classes found the provided css and the classes found in the files
    let unknown_classes = found_classes
        .difference(&accepted_classes)
        .collect::<HashSet<&String>>();
    info!(
        "{} classes used in total throughout the project, {} classes are whitelisted",
        found_classes.len(),
        accepted_classes.len()
    );
    if !unknown_classes.is_empty() {
        for class in unknown_classes {
            eprintln!("Unknown class found {}", class);
        }
        return Err(anyhow!("Unknown classes found"));
    }
    info!("Used classes are all valid");
    Ok(())
}

async fn run_watch(
    css_input: &InputType,
    watch_dir: &str,
    glob: &str,
    accepted_classes: &HashSet<String>,
    capture_regex: Arc<RegexMatcher>,
    split_regex: Arc<Regex>,
    max_opened_files: usize,
) -> Result<()> {
    let path = Path::new(watch_dir);
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(move |result| {
        if tx.send(result).is_err() {
            debug!("Couldn't send event message to watcher")
        }
    })?;
    if let InputType::Path(ref path) = css_input {
        watcher.watch(path, RecursiveMode::NonRecursive)?;
    }
    watcher.watch(path, RecursiveMode::Recursive)?;
    for result in rx {
        match result {
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                ..
            }) => {
                let _ = run(
                    glob,
                    accepted_classes,
                    capture_regex.clone(),
                    split_regex.clone(),
                    max_opened_files,
                )
                .await;
            }
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::From)),
                ..
            }) => {
                return Err(anyhow!("File {:?} was removed, exiting", path));
            }
            Ok(Event {
                kind: event_kind, ..
            }) => debug!("Unhandled event kind: {:?}", event_kind),
            Err(error) => warn!("Watch error: {}", error),
        }
    }
    Ok(())
}
