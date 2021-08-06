use anyhow::{anyhow, Result};
use askama::Template;
use cssparser::{Parser, ParserInput, RuleListParser};
use log::info;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::Display;
use std::io::{Read, Write};
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
    pub fn from_path<S: AsRef<str>>(input: S) -> Self {
        match Url::parse(input.as_ref()) {
            Err(_) => InputType::Path(PathBuf::from(input.as_ref())),
            Ok(url) => InputType::Url(url),
        }
    }

    pub fn extract_classes(&self) -> Result<HashSet<String>> {
        match self {
            Self::Path(path) => extract_classes_from_file(path),
            Self::Url(url) => extract_classes_from_url(url),
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

    let out_classes = rule_list_parser
        .into_iter()
        .flatten()
        .flatten()
        .collect::<HashSet<String>>();

    if out_classes.is_empty() {
        return Err(anyhow!("no css classes found, are you sure the provided css source contains at least one class and is valid?"));
    }

    info!("{} classes found", out_classes.len());

    Ok(out_classes)
}

pub fn write_code_to_file<P>(template: impl Template, path: P) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    info!("Writing code into {}", path);

    let code = template.render()?;

    let mut output = File::create(path)?;

    output.write_all(code.as_bytes())?;

    Ok(())
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
