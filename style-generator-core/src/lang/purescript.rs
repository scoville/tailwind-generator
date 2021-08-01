use anyhow::Result;
use askama::Template;
use std::borrow::Cow;

use super::utils::generate_module_name;

#[derive(Template)]
#[template(path = "purescript.txt")]
pub struct PurescriptTemplate<'a> {
    pub classes: Vec<String>,
    pub module_name: Cow<'a, str>,
}

impl<'a> PurescriptTemplate<'a> {
    pub fn new(
        output_directory: &'a str,
        output_filename: &'a str,
        classes: Vec<String>,
    ) -> Result<Self> {
        let module_name = generate_module_name(output_directory, output_filename)?;

        Ok(PurescriptTemplate {
            classes,
            module_name,
        })
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
