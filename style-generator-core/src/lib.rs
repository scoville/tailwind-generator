use anyhow::{anyhow, Result};
use askama::Template;
use cssparser::{Parser, ParserInput, RuleListParser};
use log::info;
use std::fmt::Display;
use std::io::{Read, Write};
use std::{fs::File, path::Path};

use crate::classes_parser::ClassesParser;

pub use lang::*;

mod classes_parser;
mod lang;
mod utils;

pub fn extract_classes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut file = File::open(path)?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    extract_classes_from_text(file_content)
}

pub fn extract_classes_from_url<U: AsRef<str>>(url: U) -> Result<Vec<String>> {
    let css_text = ureq::get(url.as_ref()).call()?.into_string()?;

    extract_classes_from_text(css_text)
}

fn extract_classes_from_text<R: AsRef<str>>(css_text: R) -> Result<Vec<String>> {
    let mut parser_input = ParserInput::new(css_text.as_ref());

    let mut parser = Parser::new(&mut parser_input);

    let mut classes = vec![];

    let rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, ClassesParser);

    for class in rule_list_parser.into_iter().flatten() {
        classes.push(class);
    }

    classes.sort();

    classes.dedup();

    if classes.is_empty() {
        return Err(anyhow!("no css classes found, are you sure the provided css source contains any classes and is valid?"));
    }

    Ok(classes)
}

pub fn write_code_to_file<P: AsRef<Path> + Display>(
    template: impl Template,
    path: P,
) -> Result<()> {
    info!("Writing code into {}", path);

    let code = template.render()?;

    let mut output = File::create(path)?;

    output.write_all(code.as_bytes())?;

    Ok(())
}

pub fn resolve_path<P: AsRef<Path> + Display>(
    directory: P,
    filename: String,
    extension: &str,
) -> Result<String> {
    let output_path = Path::new(&directory.to_string()).join(filename);

    let output_path = output_path
        .to_str()
        .ok_or_else(|| anyhow!("path is invalid"))?;

    Ok(format!("{}.{}", output_path, extension))
}
