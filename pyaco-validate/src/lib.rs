use anyhow::Result;
use clap::Clap;
use futures::{stream, StreamExt};
use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, SearcherBuilder};
use log::{error, info};
use pyaco_core::InputType;
use regex::Regex;
use std::{borrow::Borrow, collections::HashSet, fs::File, path::Path, process::exit, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clap, Debug)]
pub struct Options {
    /// CSS file path or URL used for code verification
    #[clap(short, long)]
    pub css_input: String,

    /// Glob pointing to the files to validate
    #[clap(short, long)]
    pub input_glob: String,

    /// Classes matcher regex, must include a capture containing all the classes
    #[clap(long, default_value = r#"class="([^"]+)""#)]
    pub capture_regex: String,

    /// Classes splitter regex, will split the string captured with the `capture_regex` argument and split it into classes
    #[clap(long, default_value = r#"\s+"#)]
    pub split_regex: String,

    /// How many files can be read concurrently at most, setting this value to a big number might break depending on your system
    #[clap(long, default_value = "128")]
    pub max_opened_files: usize,
}

pub async fn run(options: Options) -> Result<()> {
    let capture_regex = Arc::new(RegexMatcher::new(options.capture_regex.as_str())?);

    let split_regex = Arc::new(Regex::new(options.split_regex.as_str())?);

    info!(
        "Validating {} against {}",
        options.input_glob, options.css_input
    );

    let css_input = InputType::from_path(options.css_input);

    // The classes contained in the provided css file/URL
    let accepted_classes = css_input.extract_classes()?;

    let glob = glob(options.input_glob.as_str())?;

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
        .buffer_unordered(options.max_opened_files);

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
        "{} classes used in total in the provided files, {} whitelisted classes",
        found_classes.len(),
        accepted_classes.len()
    );

    if !unknown_classes.is_empty() {
        for class in unknown_classes {
            eprintln!("Unknown class found {}", class);
        }

        exit(1);
    }

    info!("Used classes are all valid");

    Ok(())
}

pub async fn extra_classes_from_path<C, S>(
    file: File,
    capture_regex: C,
    split_regex: S,
) -> Result<HashSet<String>>
where
    C: Borrow<RegexMatcher>,
    S: Borrow<Regex>,
{
    // Classes found in the visited file
    let mut found_classes = HashSet::new();

    // TODO: We may be able to reuse the same searcher for all the files?
    let mut searcher = SearcherBuilder::new().multi_line(true).build();

    let capture_regex = capture_regex.borrow();

    searcher.search_file(
        capture_regex,
        &file,
        UTF8(|_, line| {
            let mut captures = capture_regex.new_captures()?;

            // Search for the captures pattern...
            if capture_regex.captures(line.as_bytes(), &mut captures)? {
                if let Some(m) = captures.get(1) {
                    let classes = &line[m];

                    // ... and then split the captured classes
                    for class in split_regex.borrow().split(classes) {
                        if !class.is_empty() {
                            found_classes.insert(class.to_string());
                        }
                    }
                }
            }

            Ok(true)
        }),
    )?;

    Ok(found_classes)
}

pub async fn open_file<P: AsRef<Path>>(path: P) -> Option<File> {
    match File::open(path.as_ref()) {
        Ok(file) => Some(file),
        Err(error) => {
            if let Some(24) = error.raw_os_error() {
                eprintln!("Too many files opened [os error 24], please use the --max-opened-files options to lower the amount of opened files");

                exit(2);
            }

            error!("An error occured when trying to open {:?}", path.as_ref());

            None
        }
    }
}
