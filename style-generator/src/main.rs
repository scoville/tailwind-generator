use anyhow::Result;
use clap::Clap;
use log::info;
use std::fs::create_dir_all;
use style_generator_core::{
    extract_classes_from_file, extract_classes_from_url, resolve_path, write_code_to_file,
    ElmTemplate, Lang, PurescriptTemplate, RescriptTemplate, RescriptTypeTemplate,
    RescriptiTemplate, TypescriptTemplate, TypescriptType1Template, TypescriptType2Template,
};
use url::Url;

#[derive(Clap, Debug)]
#[clap(name = "style-generator")]
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

    /// Language used in generated code (elm|purescript|rescript|rust|typescript|typescript-type-1|typescript-type-2)"
    #[clap(short, long)]
    lang: Lang,
}

fn main() -> Result<()> {
    env_logger::init();

    let Opts {
        input,
        lang,
        output_directory,
        output_filename,
    } = Opts::parse();

    let classes = match Url::parse(input.as_str()) {
        Err(_) => {
            info!("Extracting from file {}", input);
            extract_classes_from_file(input)?
        }
        Ok(url) => {
            info!("Extracting from URL {}", url);
            extract_classes_from_url(url)?
        }
    };

    info!("{} classes found", classes.len());

    info!("Creating directory {} if needed", output_directory);

    create_dir_all(output_directory.clone())?;

    match lang {
        Lang::Elm => {
            let template =
                ElmTemplate::new(output_directory.as_str(), output_filename.as_str(), classes)?;

            write_code_to_file(
                template,
                resolve_path(output_directory, output_filename, "elm")?,
            )?;
        }
        Lang::Purescript => {
            let template = PurescriptTemplate::new(
                output_directory.as_str(),
                output_filename.as_str(),
                classes,
            )?;

            write_code_to_file(
                template,
                resolve_path(output_directory, output_filename, "purs")?,
            )?;
        }
        Lang::Rescript => {
            write_code_to_file(
                RescriptTemplate {
                    classes: classes.clone(),
                },
                resolve_path(output_directory.clone(), output_filename.clone(), "res")?,
            )?;

            write_code_to_file(
                RescriptiTemplate { classes },
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
