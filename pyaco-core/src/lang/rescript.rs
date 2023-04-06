use std::collections::HashSet;

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "rescript.txt")]
pub struct Rescript<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for Rescript<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}

#[derive(Template)]
#[template(path = "rescripti.txt")]
pub struct Rescripti<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for Rescripti<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    #[allow(clippy::unnecessary_wraps)]
    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class).to_case(Case::Camel))
    }
}
