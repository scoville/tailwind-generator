#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{
    collections::HashSet,
    convert::TryInto,
    hash,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use clap::Parser as ClapParser;
use compact_str::CompactString;
use futures::{stream, StreamExt};
use glob::glob;
use grep_matcher::{Captures, Matcher};
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, SearcherBuilder};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};
use pyaco_core::InputType;
use regex::Regex;
use serde::Deserialize;
use tokio::{fs::File, runtime::Handle, sync::mpsc};
use tracing::{error, info};

pub use crate::errors::*;
use crate::sink::{console::Console as ConsoleSink, SearchFileEvent, Sink};

mod errors;
mod sink;

#[derive(ClapParser, Debug, Deserialize)]
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

    /// Watch for changes in the provided css file and files and revalidate the code (doesn't work with URL)
    #[clap(short, long)]
    pub watch: bool,

    /// Watch debounce duration (in ms), if files are validated twice after saving a file, you should try to increase this value
    #[clap(long, default_value = "10")]
    pub watch_debounce_duration: u64,

    /// Prevents any output
    #[clap(short, long)]
    pub quiet: bool,
}

#[allow(clippy::missing_errors_doc)]
pub async fn run(options: Options) -> Result<()> {
    let capture_regex = Arc::new(RegexMatcher::new(options.capture_regex.as_str())?);

    let split_regex = Arc::new(Regex::new(options.split_regex.as_str())?);

    info!(
        "Validating {} against {}",
        options.input_glob, options.css_input
    );

    let css_input = options.css_input.as_str().try_into()?;

    let glob_pattern = glob::Pattern::new(options.input_glob.as_str())?;

    run_once(
        glob(glob_pattern.as_str())?.filter_map(std::result::Result::ok),
        &css_input,
        Arc::clone(&capture_regex),
        Arc::clone(&split_regex),
        options.max_opened_files,
        options.quiet,
    )
    .await?;

    if options.watch {
        let (mut debouncer, mut rx) =
            async_debounced_watcher(Duration::from_millis(options.watch_debounce_duration))?;
        if let InputType::Path(ref css_input_path) = css_input {
            debouncer
                .watcher()
                .watch(css_input_path, RecursiveMode::NonRecursive)?;
        }
        for filepath in (glob(glob_pattern.as_str())?).flatten() {
            debouncer
                .watcher()
                .watch(filepath.as_path(), RecursiveMode::NonRecursive)?;
        }

        while let Some(event) = rx.recv().await {
            match event {
                Ok(events) if !events.is_empty() => {
                    run_once(
                        glob(glob_pattern.as_str())?.filter_map(std::result::Result::ok),
                        &css_input,
                        Arc::clone(&capture_regex),
                        Arc::clone(&split_regex),
                        options.max_opened_files,
                        options.quiet,
                    )
                    .await?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

async fn run_once(
    paths: impl IntoIterator<Item = PathBuf>,
    css_input: &InputType,
    capture_regex: Arc<RegexMatcher>,
    split_regex: Arc<Regex>,
    max_opened_files: usize,
    quiet: bool,
) -> Result<()> {
    // The classes contained in the provided css file/URL
    let allowed_classes = Arc::new(css_input.extract_classes().await?);

    info!("{} allowed classes", allowed_classes.len());

    let sink = ConsoleSink::new(quiet);

    stream::iter(paths)
        .map(|path| {
            let capture_regex = Arc::clone(&capture_regex);
            let split_regex = Arc::clone(&split_regex);
            let allowed_classes = Arc::clone(&allowed_classes);
            let mut sink = sink.clone();

            tokio::spawn(async move {
                report_unknown_classes(
                    &allowed_classes,
                    dunce::canonicalize(path)?,
                    Arc::as_ref(&capture_regex),
                    Arc::as_ref(&split_regex),
                    &mut sink,
                )
                .await
            })
        })
        .buffer_unordered(max_opened_files)
        .map(|res| match res {
            Ok(Ok(_)) => {}
            Ok(Err(err)) => error!("task error: {err}"),
            Err(err) => error!("join error: {err}"),
        })
        .collect::<()>()
        .await;

    let valid = sink.done().await;

    if !valid {
        return Err(Error::InvalidClasses);
    }

    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub async fn report_unknown_classes<R, S>(
    allowed_classes: &HashSet<CompactString, R>,
    path: PathBuf,
    capture_regex: &RegexMatcher,
    split_regex: &Regex,
    sink: &mut S,
) -> Result<()>
where
    R: hash::BuildHasher + Default,
    S: Sink<Event = SearchFileEvent> + Send,
{
    // TODO: We may be able to reuse the same searcher for all the files?
    let mut searcher = SearcherBuilder::new().multi_line(true).build();
    let Ok(file) = open_file(&path).await?.try_into_std() else {
        unreachable!("file couldn't be converted to std file");
    };

    searcher.search_file(
        capture_regex,
        &file,
        UTF8(|line_number, line| {
            let mut captures = capture_regex.new_captures()?;

            // Search for the captures pattern...
            if capture_regex.captures(line.as_bytes(), &mut captures)? {
                if let Some(m) = captures.get(1) {
                    let classes = &line[m];

                    // ... and then split the captured classes
                    for class in split_regex.split(classes) {
                        if !class.is_empty() && !allowed_classes.contains(class) {
                            sink.send((line_number, path.clone(), class.into()));
                        }
                    }
                }
            }

            Ok(true)
        }),
    )?;

    Ok(())
}

/// ## Errors
///
/// Fails if the file can't be accessed
pub async fn open_file(path: impl AsRef<Path>) -> Result<File> {
    File::open(path).await.map_err(|err| {
        if let Some(24) = err.raw_os_error() {
            error!(
                "Too many files opened [os error 24], please use the
                --max-opened-files options to lower the amount of opened files"
            );
        }

        err.into()
    })
}

#[allow(clippy::type_complexity)]
fn async_debounced_watcher(
    timeout: Duration,
) -> Result<(
    Debouncer<ReadDirectoryChangesWatcher>,
    mpsc::Receiver<std::result::Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
)> {
    let (tx, rx) = mpsc::channel(1);

    let debouncer = new_debouncer(timeout, None, move |res| {
        Handle::current().block_on(async {
            tx.send(res).await.unwrap();
        });
    })?;

    Ok((debouncer, rx))
}
