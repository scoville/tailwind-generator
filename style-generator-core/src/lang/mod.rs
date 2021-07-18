use std::str::FromStr;

pub mod elm;
pub mod purescript;
pub mod rescript;
pub mod typescript;
pub mod typescript_type_1;
pub mod typescript_type_2;
mod utils;

pub use elm::ElmTemplate;
pub use purescript::PurescriptTemplate;
pub use rescript::RescriptTemplate;
pub use rescript::RescriptiTemplate;
pub use typescript::TypescriptTemplate;
pub use typescript_type_1::TypescriptType1Template;
pub use typescript_type_2::TypescriptType2Template;

#[derive(Debug)]
pub enum Lang {
    Elm,
    Purescript,
    Rescript,
    Typescript,
    TypescriptType1,
    TypescriptType2,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "elm" => Ok(Lang::Elm),
            "purescript" => Ok(Lang::Purescript),
            "rescript" => Ok(Lang::Rescript),
            "typescript" => Ok(Lang::Typescript),
            "typescript-type-1" => Ok(Lang::TypescriptType1),
            "typescript-type-2" => Ok(Lang::TypescriptType2),
            unknown_lang => Err(format!(
                "\"{}\" is not a valid lang, should be one of (elm|purescript|rescript|rust|typescript|typescript-type-1|typescript-type-2)",
                unknown_lang
            )),
        }
    }
}
