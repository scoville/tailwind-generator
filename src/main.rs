use anyhow::Result;
use clap::Clap;
use lang::Lang;
use log::info;
use std::fs::create_dir_all;

use crate::lang::{
    ElmTemplate, PurescriptTemplate, RescriptTemplate, RescriptiTemplate, RustTemplate,
    TypescriptTemplate,
};
use crate::utils::{extract_classes_from_file, resolve_path, write_code_to_file};

mod classes_parser;
mod lang;
mod utils;

#[derive(Clap, Debug)]
#[clap(name = "style-generator")]
struct Opts {
    /// CSS file to parse and generate code from
    #[clap(short, long)]
    input: String,

    /// Directory for generated code
    #[clap(short, long, default_value = "./")]
    output: String,

    /// Filename (without extension) used for the generated code
    #[clap(short = 'f', long, default_value = "Output")]
    output_filename: String,

    /// Language used in generated code (elm|purescript|rescript|typescript)"
    #[clap(short, long)]
    lang: Lang,
}

fn main() -> Result<()> {
    env_logger::init();

    let Opts {
        input,
        lang,
        output,
        output_filename,
    } = Opts::parse();

    info!("CSS will be read from {}", input);

    let classes = extract_classes_from_file(input)?;

    info!("{} classes found", classes.len());

    info!("Creating directory {} if needed", output);

    create_dir_all(output.clone())?;

    match lang {
        Lang::Elm => {
            write_code_to_file(
                ElmTemplate { classes },
                resolve_path(output, output_filename, "elm")?,
            )?;
        }
        Lang::Purescript => {
            write_code_to_file(
                PurescriptTemplate { classes },
                resolve_path(output, output_filename, "purs")?,
            )?;
        }
        Lang::Rescript => {
            write_code_to_file(
                RescriptTemplate {
                    classes: classes.clone(),
                },
                resolve_path(output.clone(), output_filename.clone(), "res")?,
            )?;

            write_code_to_file(
                RescriptiTemplate { classes },
                resolve_path(output, output_filename, "resi")?,
            )?;
        }
        Lang::Rust => {
            write_code_to_file(
                RustTemplate { classes },
                resolve_path(output, output_filename, "rs")?,
            )?;
        }
        Lang::Typescript => {
            write_code_to_file(
                TypescriptTemplate { classes },
                resolve_path(output, output_filename, "ts")?,
            )?;
        }
    }

    Ok(())
}
