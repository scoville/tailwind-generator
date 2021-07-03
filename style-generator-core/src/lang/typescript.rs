use askama::Template;

#[derive(Template)]
#[template(path = "typescript.txt")]
pub struct TypescriptTemplate {
    pub classes: Vec<String>,
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    pub fn name(class: &str) -> Result<String> {
        let name = escape_class_name(class.to_string()).to_case(Case::Camel);

        // TODO: Escape more keywords
        Ok(match name.as_str() {
            "static" => "static_".to_string(),
            name => name.to_string(),
        })
    }
}
