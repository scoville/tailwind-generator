use anyhow::Result;
use askama::Template;
use log::info;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::str::FromStr;

pub use super::elm::ElmTemplate;
pub use super::purescript::PurescriptTemplate;
pub use super::rescript::RescriptTemplate;
pub use super::rescript::RescriptiTemplate;
pub use super::rescript_type::RescriptTypeTemplate;
pub use super::typescript::TypescriptTemplate;
pub use super::typescript_type_1::TypescriptType1Template;
pub use super::typescript_type_2::TypescriptType2Template;

pub mod elm;
pub mod purescript;
pub mod rescript;
pub mod rescript_type;
pub mod typescript;
pub mod typescript_type_1;
pub mod typescript_type_2;

#[derive(Debug)]
pub enum Lang {
    Elm,
    Purescript,
    Rescript,
    RescriptType,
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
            "rescript-type" => Ok(Lang::RescriptType),
            "typescript" => Ok(Lang::Typescript),
            "typescript-type-1" => Ok(Lang::TypescriptType1),
            "typescript-type-2" => Ok(Lang::TypescriptType2),
            unknown_lang => Err(format!(
                "\"{}\" is not a valid lang, should be one of (elm|purescript|rescript|rescript-type|typescript|typescript-type-1|typescript-type-2)",
                unknown_lang
            )),
        }
    }
}

pub trait LangTemplate<'a>: Template + Sized {
    fn new(
        output_directory: &'a str,
        output_filename: &'a str,
        classes: &'a HashSet<String>,
    ) -> Result<Self>;

    fn write_to_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path> + Display,
    {
        info!("Writing code into {}", path);

        let code = self.render()?;

        let mut output = File::create(path)?;

        output.write_all(code.as_bytes())?;

        Ok(())
    }
}

/// Used by Elm and PureScript to generate their module name based on the directory and the filename
pub(crate) fn generate_module_name<'a>(
    output_directory: &'a str,
    output_filename: &'a str,
) -> Result<Cow<'a, str>> {
    let path = Path::new(output_directory);

    let base = path.components().into_iter().try_fold(
        "".into(),
        |acc, component| -> Result<Cow<'a, str>> {
            if let Component::Normal(part) = component {
                let part = part.to_string_lossy();

                if acc.is_empty() {
                    return Ok(part);
                }

                return Ok(format!("{}.{}", acc, part).into());
            }

            Ok(acc)
        },
    )?;

    if base.is_empty() {
        return Ok(output_filename.into());
    }

    Ok(format!("{}.{}", base, output_filename).into())
}
