use anyhow::Result;
use askama::Template;
use std::collections::HashSet;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "rescript.txt")]
pub struct RescriptTemplate<'a> {
    classes: &'a HashSet<String>,
}

impl<'a> LangTemplate<'a> for RescriptTemplate<'a> {
    fn new(
        _output_directory: &'a str,
        _output_filename: &'a str,
        classes: &'a HashSet<String>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}

#[derive(Template)]
#[template(path = "rescripti.txt")]
pub struct RescriptiTemplate<'a> {
    classes: &'a HashSet<String>,
}

impl<'a> LangTemplate<'a> for RescriptiTemplate<'a> {
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
        Ok(escape_class_name(class).to_case(Case::Camel))
    }
}
