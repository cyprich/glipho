use std::fs;

use ::clap::Parser;
use anyhow::{Context, Error, Result};
use lib::{Image, Layer, Steps};

use crate::clap::Cli;

mod clap;
mod interactive;

fn main() -> Result<()> {
    // TODO mkdir exports
    simple_logger::init().context("Failed to initialize Simple Logger")?;

    let cli = Cli::parse();
    dbg!(&cli);

    let steps = match &cli.steps_file {
        Some(val) => {
            let text = fs::read_to_string(val);
            match text {
                Ok(val) => Steps::from_toml_string(&val).ok(),
                Err(val) => {
                    eprintln!("Failed to load file {}", val);
                    None
                }
            }
        }
        None => None,
    };

    if let Some(interactive) = cli.interactive
        && interactive
    {
        crate::interactive::run(steps)?;
    } else {
        if cli.steps_file.is_none() || cli.image_file.is_none() {
            eprintln!(
                "You have to specify image file and steps file when running non-interactively"
            );
            return Err(Error::msg("Invalid input arguments"));
        }

        let mut image =
            Image::from_file(&cli.image_file.unwrap()).context("Failed to load file")?;
        let steps = Steps::from_file(&cli.steps_file.unwrap()).context("Failed to load steps")?;

        image.steps(&steps);
    }

    Ok(())
}
