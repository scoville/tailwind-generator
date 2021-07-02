use askama::Template;

#[derive(Template)]
#[template(path = "rust.txt")]
pub struct RustTemplate {
    pub classes: Vec<String>,
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class.to_string()).to_case(Case::UpperSnake))
    }
}
