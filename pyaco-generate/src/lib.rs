#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{convert::TryInto, path::Path, time::Duration};

use clap::Parser as ClapParser;
use notify::{ReadDirectoryChangesWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent, Debouncer};
use pyaco_core::{
    resolve_path, Elm, InputType, Lang, LangTemplate, Purescript, Rescript, RescriptType,
    Rescripti, Typescript, TypescriptType1, TypescriptType2,
};
use serde::Deserialize;
use tokio::{
    fs::create_dir_all,
    runtime::Handle,
    sync::mpsc::{self, Receiver},
};
use tracing::{enabled, info, warn, Level};

pub use crate::errors::*;

mod errors;

#[derive(ClapParser, Debug, Deserialize)]
pub struct Options {
    /// CSS file path and/or URL to parse and generate code from
    #[clap(short, long)]
    pub input: String,

    /// Directory for generated code
    #[clap(short, long, default_value = "./")]
    pub output_directory: String,

    /// Filename (without extension) used for the generated code
    #[clap(short = 'f', long)]
    pub output_filename: String,

    /// Language used in generated code (elm|purescript|rescript|typescript|typescript-type-1|typescript-type-2)
    #[clap(short, long)]
    pub lang: Lang,

    /// Watch for changes in the provided css file and regenarate the code (doesn't work with URL)
    #[clap(short, long)]
    pub watch: bool,

    /// Watch debounce duration (in ms), if files are validated twice after saving the css file, you should try to increase this value
    #[clap(long, default_value = "10")]
    pub watch_debounce_duration: u64,
}

#[allow(clippy::missing_errors_doc)]
pub async fn run(options: Options) -> Result<()> {
    let input = options.input.as_str().try_into()?;

    if enabled!(Level::INFO) || enabled!(Level::WARN) {
        match input {
            InputType::Path(ref path) => info!("Extracting from file {:?}", path),
            InputType::Url(ref url) => {
                info!("Extracting from URL {}", url);

                if options.watch {
                    warn!("You provided an URL as the css input, watch mode will not be activated");
                }
            }
        }
    };

    info!("Creating directory {} if needed", options.output_directory);

    create_dir_all(options.output_directory.as_str()).await?;

    // Always run at least once, even in watch mode
    run_once(
        &input,
        &options.lang,
        options.output_directory.as_str(),
        options.output_filename.as_str(),
    )
    .await?;

    if options.watch {
        if let InputType::Path(ref path) = input {
            run_watch(
                path,
                &options.lang,
                options.output_directory.as_str(),
                options.output_filename.as_str(),
                options.watch_debounce_duration,
            )
            .await?;
        }
    }

    Ok(())
}

async fn run_once(
    input: &InputType,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
) -> Result<()> {
    let classes = input.extract_classes().await?;

    match lang {
        Lang::Elm => {
            let template = Elm::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "elm"))
                .await?;
        }
        Lang::Purescript => {
            let template = Purescript::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "purs"))
                .await?;
        }
        Lang::Rescript => {
            let template = Rescript::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "res"))
                .await?;

            let template = Rescripti::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "resi"))
                .await?;
        }
        Lang::RescriptType => {
            let template = RescriptType::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "res"))
                .await?;
        }
        Lang::Typescript => {
            let template = Typescript::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "ts"))
                .await?;
        }
        Lang::TypescriptType1 => {
            let template = TypescriptType1::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "ts"))
                .await?;
        }
        Lang::TypescriptType2 => {
            let template = TypescriptType2::new(output_directory, output_filename, &classes)?;

            template
                .write_to_file(resolve_path(output_directory, output_filename, "ts"))
                .await?;
        }
    }

    Ok(())
}

async fn run_watch(
    path: &Path,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
    watch_debounce_duration: u64,
) -> Result<()> {
    let (mut debouncer, mut rx) =
        async_debounced_watcher(Duration::from_millis(watch_debounce_duration))?;
    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    while let Some(event) = rx.recv().await {
        match event {
            Ok(events) if !events.is_empty() => {
                run_once(
                    &InputType::Path(path.to_owned()),
                    lang,
                    output_directory,
                    output_filename,
                )
                .await?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
fn async_debounced_watcher(
    timeout: Duration,
) -> Result<(
    Debouncer<ReadDirectoryChangesWatcher>,
    Receiver<std::result::Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
)> {
    let (tx, rx) = mpsc::channel(1);

    let debouncer = new_debouncer(timeout, None, move |res| {
        Handle::current().block_on(async {
            tx.send(res).await.unwrap();
        });
    })?;

    Ok((debouncer, rx))
}
