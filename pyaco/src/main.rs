#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use clap::Parser as ClapParser;
use pyaco_generate::{run as generate, Options as GenerateOptions};
use pyaco_validate::{run as validate, Options as ValidateOptions};

#[derive(ClapParser, Debug)]
#[clap(name = "pyaco", version)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(ClapParser, Debug)]
pub enum Command {
    /// Generate code from a css input
    Generate(GenerateOptions),
    /// Validate code against a css input
    Validate(ValidateOptions),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let options: Options = Options::parse();

    match options.command {
        Command::Generate(options) => generate(options).await?,
        Command::Validate(options) => validate(options).await?,
    };

    Ok(())
}
