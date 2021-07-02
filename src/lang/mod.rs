use std::str::FromStr;

pub mod elm;
pub mod purescript;
pub mod rescript;
pub mod rust;
pub mod typescript;

pub use elm::ElmTemplate;
pub use purescript::PurescriptTemplate;
pub use rescript::RescriptTemplate;
pub use rescript::RescriptiTemplate;
pub use rust::RustTemplate;
pub use typescript::TypescriptTemplate;

#[derive(Debug)]
pub enum Lang {
    Elm,
    Purescript,
    Rescript,
    Rust,
    Typescript,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "elm" => Ok(Lang::Elm),
            "purescript" => Ok(Lang::Purescript),
            "rescript" => Ok(Lang::Rescript),
            "rust" => Ok(Lang::Rust),
            "typescript" => Ok(Lang::Typescript),
            unknown_lang => Err(format!(
                "\"{}\" is not a valid lang, should be one of (elm|purescript|rescript|rust|typescript)",
                unknown_lang
            )),
        }
    }
}
