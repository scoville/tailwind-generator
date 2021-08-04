use askama::Template;
use std::collections::HashSet;

#[derive(Template)]
#[template(path = "typescript_type_1.txt")]
pub struct TypescriptType1Template {
    pub classes: HashSet<String>,
}
