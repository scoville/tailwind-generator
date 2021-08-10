use anyhow::Result;
use askama::Template;
use std::collections::HashSet;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "rescript_type.txt")]
pub struct RescriptTypeTemplate<'a> {
    classes: &'a HashSet<String>,
}

impl<'a> LangTemplate<'a> for RescriptTypeTemplate<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<String>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}
