use askama::Template;

#[derive(Template)]
#[template(path = "rescript.txt")]
pub struct RescriptTemplate {
    pub classes: Vec<String>,
}

#[derive(Template)]
#[template(path = "rescripti.txt")]
pub struct RescriptiTemplate {
    pub classes: Vec<String>,
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class.to_string()).to_case(Case::Camel))
    }
}
