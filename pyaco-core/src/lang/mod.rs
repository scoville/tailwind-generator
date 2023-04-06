use std::{
    borrow::Cow, collections::HashSet, fmt::Display, path::Component, path::Path, str::FromStr,
};

use askama::Template;
use async_trait::async_trait;
use compact_str::CompactString;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::Result;

pub use super::elm::Elm;
pub use super::purescript::Purescript;
pub use super::rescript::Rescript;
pub use super::rescript::Rescripti;
pub use super::rescript_type::RescriptType;
pub use super::typescript::Typescript;
pub use super::typescript_type_1::TypescriptType1;
pub use super::typescript_type_2::TypescriptType2;

pub mod elm;
pub mod purescript;
pub mod rescript;
pub mod rescript_type;
pub mod typescript;
pub mod typescript_type_1;
pub mod typescript_type_2;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
                "\"{unknown_lang}\" is not a valid lang, should be one of (elm|purescript|rescript|rescript-type|typescript|typescript-type-1|typescript-type-2)"
            )),
        }
    }
}

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait LangTemplate<'a>: Template + Sized {
    /// ## Errors
    ///
    /// A template creation typically fails when the directory/filenames are not present or can't be accessed
    fn new(
        output_directory: &'a str,
        output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self>;

    async fn write_to_file(&self, path: impl AsRef<Path> + Display + Send) -> Result<()> {
        info!("Writing code into {}", path);

        let code = self.render()?;

        let mut output = File::create(path).await?;

        output.write_all(code.as_bytes()).await?;

        Ok(())
    }
}

/// Used by Elm and PureScript to generate their module name based on the directory and the filename
pub(crate) fn generate_module_name<'a>(
    output_directory: &'a str,
    output_filename: &'a str,
) -> Result<Cow<'a, str>> {
    let path = Path::new(output_directory);

    let base = path
        .components()
        .try_fold("".into(), |acc, component| -> Result<Cow<'a, str>> {
            if let Component::Normal(part) = component {
                let part = part.to_string_lossy();

                if acc.is_empty() {
                    return Ok(part);
                }

                return Ok(format!("{acc}.{part}").into());
            }

            Ok(acc)
        })?;

    if base.is_empty() {
        return Ok(output_filename.into());
    }

    Ok(format!("{base}.{output_filename}").into())
}
