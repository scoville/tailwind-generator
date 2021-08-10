use anyhow::Result;
use askama::Template;
use std::{borrow::Cow, collections::HashSet};

use crate::LangTemplate;

use super::generate_module_name;

#[derive(Template)]
#[template(path = "purescript.txt")]
pub struct PurescriptTemplate<'a> {
    classes: &'a HashSet<String>,
    module_name: Cow<'a, str>,
}

impl<'a> LangTemplate<'a> for PurescriptTemplate<'a> {
    fn new(
        output_directory: &'a str,
        output_filename: &'a str,
        classes: &'a HashSet<String>,
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
