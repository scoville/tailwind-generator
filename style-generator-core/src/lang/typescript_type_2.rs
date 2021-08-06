use askama::Template;
use std::collections::HashSet;

#[derive(Template)]
#[template(path = "typescript_type_2.txt")]
pub struct TypescriptType2Template {
    pub classes: HashSet<String>,
}
