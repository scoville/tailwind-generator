use anyhow::Result;
use clap::{crate_version, Clap};
use pyaco_generate::{run as generate, Options as GenerateOptions};
use pyaco_validate::{run as validate, Options as ValidateOptions};

#[derive(Clap, Debug)]
#[clap(name = "pyaco", version = crate_version!())]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
    /// Generate code from a css input
    Generate(GenerateOptions),
    /// Validate code against a css input
    Validate(ValidateOptions),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let options: Options = Options::parse();

    match options.command {
        Command::Generate(options) => generate(options),
        Command::Validate(options) => validate(options).await,
    }?;

    Ok(())
}
