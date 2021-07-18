use anyhow::{anyhow, Result};
use convert_case::{Case, Casing};
use std::path::Component;
use std::path::Path;

/// Used by Elm and PureScript to generate their module name based on the directory and the filename
pub fn generate_module_name(output_directory: &str, output_filename: &str) -> Result<String> {
    let path = Path::new(output_directory);

    let base = path.components().into_iter().try_fold(
        String::new(),
        |acc, component| -> Result<String> {
            if let Component::Normal(part) = component {
                let part = part
                    .to_str()
                    .ok_or_else(|| anyhow!("the directory path contains non unicode characters"))?
                    .to_case(Case::Pascal);

                if acc.is_empty() {
                    return Ok(part);
                }

                return Ok(format!("{}.{}", acc, part));
            }

            Ok(acc)
        },
    )?;

    let last = output_filename.to_case(Case::Pascal);

    if base.is_empty() {
        return Ok(last);
    }

    Ok(format!("{}.{}", base, last))
}
