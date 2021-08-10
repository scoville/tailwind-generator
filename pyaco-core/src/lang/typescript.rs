use anyhow::Result;
use askama::Template;
use std::collections::HashSet;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "typescript.txt")]
pub struct TypescriptTemplate<'a> {
    classes: &'a HashSet<String>,
}

impl<'a> LangTemplate<'a> for TypescriptTemplate<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<String>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    pub fn name(class: &str) -> Result<String> {
        let name = escape_class_name(class).to_case(Case::Camel);

        // TODO: Escape more keywords
        Ok(match name.as_str() {
            "static" => String::from("static_"),
            _ => name,
        })
    }
}
