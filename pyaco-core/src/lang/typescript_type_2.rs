use std::collections::HashSet;

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "typescript_type_2.txt")]
pub struct TypescriptType2<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for TypescriptType2<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}
