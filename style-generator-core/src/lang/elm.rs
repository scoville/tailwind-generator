use anyhow::Result;
use askama::Template;

use super::utils::generate_module_name;

#[derive(Template)]
#[template(path = "elm.txt")]
pub struct ElmTemplate {
    pub module_name: String,
    pub classes: Vec<String>,
}

impl ElmTemplate {
    pub fn new(
        output_directory: &str,
        output_filename: &str,
        classes: Vec<String>,
    ) -> Result<Self> {
        let module_name = generate_module_name(output_directory, output_filename)?;

        Ok(ElmTemplate {
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
        Ok(escape_class_name(class.to_string()).to_case(Case::Camel))
    }
}
