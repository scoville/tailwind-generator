use askama::Template;

#[derive(Template)]
#[template(path = "rescript.txt")]
pub struct RescriptTemplate<'a> {
    pub classes: &'a [String],
}

#[derive(Template)]
#[template(path = "rescripti.txt")]
pub struct RescriptiTemplate<'a> {
    pub classes: &'a [String],
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class).to_case(Case::Camel))
    }
}
