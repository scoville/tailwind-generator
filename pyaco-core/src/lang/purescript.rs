use std::{borrow::Cow, collections::HashSet};

use askama::Template;
use compact_str::CompactString;

use crate::{LangTemplate, Result};

use super::generate_module_name;

#[derive(Template)]
#[template(path = "purescript.txt")]
pub struct Purescript<'a> {
    classes: &'a HashSet<CompactString>,
    module_name: Cow<'a, str>,
}

impl<'a> LangTemplate<'a> for Purescript<'a> {
    fn new(
        output_directory: &'a str,
        output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        let module_name = generate_module_name(output_directory, output_filename)?;

        Ok(Purescript {
            classes,
            module_name,
        })
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
