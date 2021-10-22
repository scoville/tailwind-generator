use anyhow::{anyhow, Error, Result};
use cssparser::{Parser, ParserInput, RuleListParser};
use log::{error, info};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, path::Path};
use url::Url;

use crate::classes_parser::ClassesParser;

pub use lang::*;

mod classes_parser;
mod lang;
mod utils;

#[derive(Debug)]
pub enum InputType {
    Path(PathBuf),
    Url(Url),
}

impl InputType {
    pub fn extract_classes(&self) -> Result<HashSet<String>> {
        match self {
            Self::Path(path) => extract_classes_from_file(path),
            Self::Url(url) => extract_classes_from_url(url),
        }
    }
}

impl TryFrom<&str> for InputType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match Url::parse(value) {
            Err(_) => {
                let filepath = std::fs::canonicalize(value)?;

                Ok(InputType::Path(filepath))
            }
            Ok(url) => Ok(InputType::Url(url)),
        }
    }
}

pub fn extract_classes_from_file<P>(path: P) -> Result<HashSet<String>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    extract_classes_from_text(file_content)
}

pub fn extract_classes_from_url<U>(url: U) -> Result<HashSet<String>>
where
    U: AsRef<str>,
{
    let css_text = ureq::get(url.as_ref()).call()?.into_string()?;

    extract_classes_from_text(css_text)
}

fn extract_classes_from_text<C>(css_text: C) -> Result<HashSet<String>>
where
    C: AsRef<str>,
{
    let mut parser_input = ParserInput::new(css_text.as_ref());

    let mut parser = Parser::new(&mut parser_input);

    let rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, ClassesParser);

    let out_classes =
        rule_list_parser
            .into_iter()
            .fold(HashSet::new(), |mut classes, classes_results| {
                match classes_results {
                    Ok(None) => (),
                    Ok(Some(new_classes)) => classes.extend(new_classes),
                    Err(error) => error!("An error occured while parsing the css: {:?}", error),
                };

                classes
            });

    if out_classes.is_empty() {
        return Err(anyhow!("no css classes found, are you sure the provided css source contains at least one class and is valid?"));
    }

    info!("{} classes found", out_classes.len());

    Ok(out_classes)
}

pub fn resolve_path<D, P>(directory: D, filename: P, extension: &str) -> Result<String>
where
    D: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let output_path = Path::new(&directory).join(filename);

    let output_path = output_path.to_string_lossy();

    Ok(format!("{}.{}", output_path, extension))
}
