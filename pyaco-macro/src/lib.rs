#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{collections::HashSet, env, path::PathBuf, process};

use anyhow::{bail, Result};
use compact_str::CompactString;
use once_cell::sync::OnceCell;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_call_site_warning, proc_macro_error};
use pyaco_core::{extract_classes_from_file, extract_classes_from_url};
use quote::quote;
use serde::Deserialize;
use syn::{parse_macro_input, LitStr};
use tokio::{fs::File, io::AsyncReadExt, runtime::Runtime};
use tracing::error;

#[derive(Debug, Deserialize)]
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

static CONFIG_FILE_NAME: &str = "pyaco.toml";
static CONFIG: OnceCell<Config> = OnceCell::new();
static ACCEPTED_CLASSES: OnceCell<HashSet<CompactString>> = OnceCell::new();

async fn read_config() -> Result<Config> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let filename = root.join(CONFIG_FILE_NAME);

    if !filename.exists() {
        bail!("couldn't find required pyaco.toml configuration file",);
    }

    let mut file = File::open(filename).await?;

    let mut content = String::new();

    file.read_to_string(&mut content).await?;

    let config = toml::from_str(content.as_str())?;

    Ok(config)
}

#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input);

    let input_value = input.value();

    let classes = input_value.split_whitespace().collect::<Vec<&str>>();

    let mut out_classes = String::new();

    // Validate class names
    for class in classes {
        if out_classes.contains(class) {
            emit_call_site_warning!("Class already in class names list: {}", class);
            continue;
        }

        let accepted_classes = ACCEPTED_CLASSES.get_or_init(init_accepted_classes);
        if !accepted_classes.contains(class) {
            abort_call_site!("Invalid class name: {}", class)
        }

        if out_classes.is_empty() {
            out_classes.push_str(class);
        } else {
            out_classes.push_str(format!(" {class}").as_str());
        }
    }

    let expanded = quote! {
        #out_classes
    };

    expanded.into()
}

fn init_accepted_classes() -> HashSet<CompactString> {
    let config = CONFIG.get_or_init(|| {
        let rt = match Runtime::new() {
            Ok(rt) => rt,
            Err(err) => {
                error!("couldn't create the tokio runtime: {err}");
                process::exit(1);
            }
        };

        match rt.block_on(read_config()) {
            Ok(config) => config,
            Err(err) => {
                error!("couldn't read config file: {err}");
                process::exit(1);
            }
        }
    });
    match &config.general.input {
        InputConfig::Simple(path) | InputConfig::Path { path } => {
            let rt = match Runtime::new() {
                Ok(rt) => rt,
                Err(err) => {
                    error!("couldn't create the tokio runtime: {err}");
                    process::exit(1);
                }
            };

            rt.block_on(extract_classes_from_file(path))
        }
        InputConfig::Url { url } => extract_classes_from_url(url),
    }
    .expect("css could not be loaded")
}
