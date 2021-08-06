use askama::Template;
use std::collections::HashSet;

#[derive(Template)]
#[template(path = "rescript_type.txt")]
pub struct RescriptTypeTemplate {
    pub classes: HashSet<String>,
}
