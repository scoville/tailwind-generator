use askama::Template;

#[derive(Template)]
#[template(path = "typescript_type_1.txt")]
pub struct TypescriptType1Template {
    pub classes: Vec<String>,
}
