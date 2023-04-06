use std::collections::HashSet;

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "rescript_type.txt")]
pub struct RescriptType<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for RescriptType<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}
