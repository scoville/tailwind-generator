use askama::Template;
use std::collections::HashSet;

#[derive(Template)]
#[template(path = "typescript.txt")]
pub struct TypescriptTemplate {
    pub classes: HashSet<String>,
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
