use anyhow::{anyhow, Result};
use askama::Template;
use cssparser::{Parser, ParserInput, RuleListParser};
use log::info;
use std::fmt::Display;
use std::io::{Read, Write};
use std::{fs::File, path::Path};

use crate::classes_parser::ClassesParser;

macro_rules! replace_first_char {
    ($escaped_class_name:ident, $($char:literal => $replace_with:expr),*) => (
        match $escaped_class_name.chars().nth(0) {
            $(Some($char) => $escaped_class_name.replace_range(..1, $replace_with),)+
            _ => (),
        }
    )
}

pub fn escape_class_name(class: String) -> String {
    let mut escaped_class_name = class;

    replace_first_char!(escaped_class_name,
        '-' => "neg-",
        '0' => "zero-",
        '1' => "one-",
        '2' => "two-",
        '3' => "three-",
        '4' => "four-",
        '5' => "five-",
        '6' => "six-",
        '7' => "seven-",
        '8' => "eight-",
        '9' => "nine-"
    );

    escaped_class_name
        .replace(":-", "-neg-")
        .replace("/", "-over-")
        .replace(":", "-")
        .replace(".", "-dot-")
}

pub fn extract_classes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut file = File::open(path)?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    let mut parser_input = ParserInput::new(file_content.as_str());

    let mut parser = Parser::new(&mut parser_input);

    let mut classes = vec![];

    let rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, ClassesParser);

    for class in rule_list_parser.into_iter().flatten() {
        classes.extend(vec![class]);
    }

    classes.sort();

    classes.dedup();

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
        .ok_or_else(|| anyhow!("Path is invalid"))?;

    Ok(format!("{}.{}", output_path, extension))
}
