use anyhow::Result;
use clap::{crate_version, Clap};
use log::{debug, info, warn};
use notify::event::{DataChange, ModifyKind};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc::channel;
use style_generator_core::{
    extract_classes_from_file, extract_classes_from_url, resolve_path, write_code_to_file,
    ElmTemplate, Lang, PurescriptTemplate, RescriptTemplate, RescriptTypeTemplate,
    RescriptiTemplate, TypescriptTemplate, TypescriptType1Template, TypescriptType2Template,
};
use url::Url;

enum InputType {
    Path(PathBuf),
    Url(Url),
}

#[derive(Clap, Debug)]
#[clap(name = "style-generator", version = crate_version!())]
struct Opts {
    /// CSS file path or URL to parse and generate code from
    #[clap(short, long)]
    input: String,

    /// Directory for generated code
    #[clap(short, long, default_value = "./")]
    output_directory: String,

    /// Filename (without extension) used for the generated code
    #[clap(short = 'f', long)]
    output_filename: String,

    /// Language used in generated code (elm|purescript|rescript|typescript|typescript-type-1|typescript-type-2)"
    #[clap(short, long)]
    lang: Lang,

    /// Watch for changes in the provided css file and regenarate the code (doesn't work with URL)
    #[clap(short, long)]
    watch: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let Opts {
        input,
        lang,
        output_directory,
        output_filename,
        watch,
    } = Opts::parse();

    let input = match Url::parse(input.as_str()) {
        Err(_) => {
            info!("Extracting from file {}", input);

            InputType::Path(PathBuf::from(input))
        }
        Ok(url) => {
            info!("Extracting from URL {}", url);

            if watch {
                warn!("You provided an URL as the css input, watch mode will not be activated")
            }

            InputType::Url(url)
        }
    };

    info!("Creating directory {} if needed", output_directory);

    create_dir_all(output_directory.as_str())?;

    // Always run at least once, even in watch mode
    run(
        &input,
        &lang,
        output_directory.as_str(),
        output_filename.as_str(),
    )?;

    if watch {
        if let InputType::Path(ref path) = input {
            run_watch(
                path,
                &lang,
                output_directory.as_str(),
                output_filename.as_str(),
            )?
        }
    }

    Ok(())
}

fn run(
    input: &InputType,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
) -> Result<()> {
    let classes = match input {
        InputType::Path(path) => extract_classes_from_file(path)?,
        InputType::Url(url) => extract_classes_from_url(url)?,
    };

    match lang {
        Lang::Elm => {
            let template = ElmTemplate::new(output_directory, output_filename, classes)?;

            write_code_to_file(
                template,
                resolve_path(output_directory, output_filename, "elm")?,
            )?;
        }
        Lang::Purescript => {
            let template = PurescriptTemplate::new(output_directory, output_filename, classes)?;

            write_code_to_file(
                template,
                resolve_path(output_directory, output_filename, "purs")?,
            )?;
        }
        Lang::Rescript => {
            write_code_to_file(
                RescriptTemplate { classes: &classes },
                resolve_path(output_directory, output_filename, "res")?,
            )?;

            write_code_to_file(
                RescriptiTemplate { classes: &classes },
                resolve_path(output_directory, output_filename, "resi")?,
            )?;
        }
        Lang::RescriptType => {
            write_code_to_file(
                RescriptTypeTemplate { classes },
                resolve_path(output_directory, output_filename, "res")?,
            )?;
        }
        Lang::Typescript => {
            write_code_to_file(
                TypescriptTemplate { classes },
                resolve_path(output_directory, output_filename, "ts")?,
            )?;
        }
        Lang::TypescriptType1 => {
            write_code_to_file(
                TypescriptType1Template { classes },
                resolve_path(output_directory, output_filename, "ts")?,
            )?;
        }
        Lang::TypescriptType2 => {
            write_code_to_file(
                TypescriptType2Template { classes },
                resolve_path(output_directory, output_filename, "ts")?,
            )?;
        }
    }

    Ok(())
}

fn run_watch(
    path: &Path,
    lang: &Lang,
    output_directory: &str,
    output_filename: &str,
) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(move |result| tx.send(result).unwrap())?;

    watcher.watch(path, RecursiveMode::NonRecursive)?;

    for result in rx {
        match result {
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                ..
            }) => run(
                &InputType::Path(path.to_owned()),
                lang,
                output_directory,
                output_filename,
            )?,
            Ok(Event {
                kind: EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::From)),
                ..
            }) => {
                warn!("File {:?} was removed, exiting", path);
                process::exit(2)
            }
            Ok(Event {
                kind: event_kind, ..
            }) => debug!("Unhandled event kind: {:?}", event_kind),
            Err(error) => warn!("Watch error: {}", error),
        }
    }

    Ok(())
}
