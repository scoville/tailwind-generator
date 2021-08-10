use anyhow::Result;
use askama::Template;
use std::collections::HashSet;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "typescript_type_2.txt")]
pub struct TypescriptType2Template<'a> {
    classes: &'a HashSet<String>,
}

impl<'a> LangTemplate<'a> for TypescriptType2Template<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<String>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}
