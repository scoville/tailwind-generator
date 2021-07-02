use std::str::FromStr;

pub mod elm;
pub mod purescript;
pub mod rescript;
pub mod typescript;

#[derive(Debug)]
pub enum Lang {
    Elm,
    Purescript,
    Rescript,
    Typescript,
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "elm" => Ok(Lang::Elm),
            "purescript" => Ok(Lang::Purescript),
            "rescript" => Ok(Lang::Rescript),
            "typescript" => Ok(Lang::Typescript),
            unknown_lang => Err(format!(
                "\"{}\" is not a valid lang, should be one of (elm|purescript|rescript|typescript)",
                unknown_lang
            )),
        }
    }
}
