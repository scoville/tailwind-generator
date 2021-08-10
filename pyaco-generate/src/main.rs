use anyhow::Result;
use clap::{crate_version, Clap};
use log::{debug, info, log_enabled, warn, Level};
use notify::event::{DataChange, ModifyKind};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pyaco_core::{
    resolve_path, ElmTemplate, InputType, Lang, LangTemplate, PurescriptTemplate, RescriptTemplate,
    RescriptTypeTemplate, RescriptiTemplate, TypescriptTemplate, TypescriptType1Template,
    TypescriptType2Template,
};
use std::fs::create_dir_all;
use std::path::Path;
use std::process;
use std::sync::mpsc::channel;

#[derive(Clap, Debug)]
#[clap(name = "pyaco-generate", version = crate_version!())]
struct Options {
    /// CSS file path and/or URL to parse and generate code from
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

    let Options {
        input,
        lang,
        output_directory,
        output_filename,
        watch,
    } = Options::parse();

    let input = InputType::from_path(input);

    if log_enabled!(Level::Info) || log_enabled!(Level::Warn) {
        match input {
            InputType::Path(ref path) => info!("Extracting from file {:?}", path),
            InputType::Url(ref url) => {
                info!("Extracting from URL {}", url);

                if watch {
                    warn!("You provided an URL as the css input, watch mode will not be activated")
                }
            }
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
) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(move |result| {
        if tx.send(result).is_err() {
            debug!("Couldn't send event message to watcher")
        }
    })?;

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
