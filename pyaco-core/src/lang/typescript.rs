use std::collections::HashSet;

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "typescript.txt")]
pub struct Typescript<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for Typescript<'a> {
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
        let name = escape_class_name(class).to_case(Case::Camel);

        // TODO: Escape more keywords
        Ok(match name.as_str() {
            "static" => String::from("static_"),
            _ => name,
        })
    }
}
