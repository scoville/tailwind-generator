use askama::Template;

#[derive(Template)]
#[template(path = "rust.txt")]
pub struct RustTemplate {
    pub classes: Vec<String>,
}
