use askama::Template;

#[derive(Template)]
#[template(path = "typescript_type_2.txt")]
pub struct TypescriptType2Template {
    pub classes: Vec<String>,
}
