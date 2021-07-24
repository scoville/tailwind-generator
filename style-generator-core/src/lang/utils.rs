use anyhow::Result;
use std::borrow::Cow;
use std::path::Component;
use std::path::Path;

/// Used by Elm and PureScript to generate their module name based on the directory and the filename
pub fn generate_module_name<'a>(
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
