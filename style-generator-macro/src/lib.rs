#[macro_use]
extern crate lazy_static;

use anyhow::{anyhow, Result};
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::{env, fs::File, io::Read, path::PathBuf};
use style_generator_core::{extract_classes_from_file, extract_classes_from_url};
use syn::{parse_macro_input, LitStr};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum InputConfig {
    Simple(String),
    Path { path: String },
    Url { url: String },
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    input: InputConfig,
}

#[derive(Debug, Deserialize)]
struct Config {
    general: GeneralConfig,
}

static CONFIG_FILE_NAME: &str = "style-generator.toml";

fn read_config() -> Result<Config> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let filename = root.join(CONFIG_FILE_NAME);

    if !filename.exists() {
        return Err(anyhow!(
            "couldn't find style-generator.toml configuration file but it is required",
        ));
    }

    let mut file = File::open(filename)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let config = toml::from_str(content.as_str())?;

    Ok(config)
}

lazy_static! {
    static ref CONFIG: Config = read_config().expect("couldn't read config file");
    static ref ACCEPTED_CLASSES: Vec<String> = match CONFIG.general.input.clone() {
        InputConfig::Simple(path) | InputConfig::Path { path } => extract_classes_from_file(path),
        InputConfig::Url { url } => extract_classes_from_url(url),
    }
    .expect("css could not be loaded");
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input);

    let input_value = input.value();

    let classes = input_value.split_whitespace().collect::<Vec<&str>>();

    let mut out_classes = Vec::new();

    // Validate class names
    for class in classes {
        if out_classes.contains(&class) {
            panic!("Class already in class names list: {}", class)
        }

        if !ACCEPTED_CLASSES.contains(&class.to_string()) {
            panic!("Invalid class name: {}", class)
        }

        out_classes.push(class);
    }

    let joined_classes = out_classes.join(" ");

    let trimmed_classed = joined_classes.trim();

    let expanded = quote! {
        #trimmed_classed
    };

    expanded.into()
}