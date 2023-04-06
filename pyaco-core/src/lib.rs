#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{
    collections::HashSet,
    convert::TryFrom,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use compact_str::CompactString;
use cssparser::{Parser, ParserInput, RuleListParser};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{debug_span, error, info};
use url::Url;

use crate::classes_parser::ClassesParser;

pub use crate::errors::*;
pub use crate::lang::*;

mod classes_parser;
mod errors;
mod lang;
mod utils;

#[derive(Debug)]
pub enum InputType {
    Path(PathBuf),
    Url(Url),
}

impl InputType {
    /// ## Errors
    ///
    /// See `extract_classes_from_file` and `extract_classes_from_file` for more
    pub async fn extract_classes(&self) -> Result<HashSet<CompactString>> {
        match self {
            Self::Path(path) => extract_classes_from_file(path).await,
            Self::Url(url) => extract_classes_from_url(url),
        }
    }
}

impl TryFrom<&str> for InputType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match Url::parse(value) {
            Err(_) => {
                let filepath = dunce::canonicalize(value)?;

                Ok(InputType::Path(filepath))
            }
            Ok(url) => Ok(InputType::Url(url)),
        }
    }
}

/// ## Errors
///
/// Fails if the file can't be read or if the css can't be parsed
pub async fn extract_classes_from_file(path: impl AsRef<Path>) -> Result<HashSet<CompactString>> {
    let mut file = File::open(path).await?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).await?;

    extract_classes_from_text(file_content)
}

/// ## Errors
///
/// Fails if the request to the url fails or if the css can't be parsed
pub fn extract_classes_from_url(url: impl AsRef<str>) -> Result<HashSet<CompactString>> {
    let css_text = ureq::get(url.as_ref())
        .call()
        .map_err(Box::new)?
        .into_string()?;

    extract_classes_from_text(css_text)
}

fn extract_classes_from_text(css_text: impl AsRef<str>) -> Result<HashSet<CompactString>> {
    debug_span!("extract_classes_from_text");

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
        return Err(Error::NoCssClassesFound);
    }

    info!("{} classes found", out_classes.len());

    Ok(out_classes)
}

pub fn resolve_path(
    directory: impl AsRef<OsStr>,
    filename: impl AsRef<Path>,
    extension: &str,
) -> String {
    let output_path = Path::new(&directory).join(filename);

    let output_path = output_path.to_string_lossy();

    format!("{output_path}.{extension}")
}
