use anyhow::Result;
use clap::Parser as ClapParser;
use log::{info, log_enabled, warn, Level};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use pyaco_core::{
    resolve_path, ElmTemplate, InputType, Lang, LangTemplate, PurescriptTemplate, RescriptTemplate,
    RescriptTypeTemplate, RescriptiTemplate, TypescriptTemplate, TypescriptType1Template,
    TypescriptType2Template,
};
use std::convert::TryInto;
use std::fs::create_dir_all;
use std::path::Path;
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(ClapParser, Debug)]
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

pub fn run(options: Options) -> Result<()> {
    let input = options.input.as_str().try_into()?;

    if log_enabled!(Level::Info) || log_enabled!(Level::Warn) {
        match input {
            InputType::Path(ref path) => info!("Extracting from file {:?}", path),
            InputType::Url(ref url) => {
                info!("Extracting from URL {}", url);

                if options.watch {
                    warn!("You provided an URL as the css input, watch mode will not be activated")
                }
            }
        }
    };

    info!("Creating directory {} if needed", options.output_directory);

    create_dir_all(options.output_directory.as_str())?;

    // Always run at least once, even in watch mode
    run_once(
        &input,
        &options.lang,
        options.output_directory.as_str(),
        options.output_filename.as_str(),
    )?;

    if options.watch {
        if let InputType::Path(ref path) = input {
            run_watch(
                path,
                &options.lang,
                options.output_directory.as_str(),
                options.output_filename.as_str(),
                options.watch_debounce_duration,
            )?
        }
    }

    Ok(())
}

fn run_once(
    input: &InputType,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
) -> Result<()> {
    let classes = input.extract_classes()?;

    match lang {
        Lang::Elm => {
            let template = ElmTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "elm")?)?;
        }
        Lang::Purescript => {
            let template = PurescriptTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "purs")?)?;
        }
        Lang::Rescript => {
            let template = RescriptTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "res")?)?;

            let template = RescriptiTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "resi")?)?;
        }
        Lang::RescriptType => {
            let template = RescriptTypeTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "res")?)?;
        }
        Lang::Typescript => {
            let template = TypescriptTemplate::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "ts")?)?;
        }
        Lang::TypescriptType1 => {
            let template =
                TypescriptType1Template::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "ts")?)?;
        }
        Lang::TypescriptType2 => {
            let template =
                TypescriptType2Template::new(output_directory, output_filename, &classes)?;

            template.write_to_file(resolve_path(output_directory, output_filename, "ts")?)?;
        }
    }

    Ok(())
}

fn run_watch(
    path: &Path,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
    watch_debounce_duration: u64,
) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(watch_debounce_duration))?;

    watcher.watch(path, RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => run_once(
                &InputType::Path(path.to_owned()),
                lang,
                output_directory,
                output_filename,
            )?,
            Ok(DebouncedEvent::Remove(_)) => {
                warn!("File {:?} was removed, exiting", path);

                process::exit(2)
            }
            Ok(_) | Err(_) => (),
        }
    }
}
