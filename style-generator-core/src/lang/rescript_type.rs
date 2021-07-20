use askama::Template;

#[derive(Template)]
#[template(path = "rescript_type.txt")]
pub struct RescriptTypeTemplate {
    pub classes: Vec<String>,
}
