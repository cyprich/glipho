use std::fs;

use ::clap::Parser;
use anyhow::{Context, Error, Result};
use lib::{Effects, Image};
use log::{error, warn};

use crate::{arguments::Arguments, manual::Manual};

mod action;
mod arguments;
mod manual;
mod screen;

fn main() -> Result<()> {
    // TODO: make logger prettier
    simple_logger::init().context("Failed to initialize Simple Logger")?;

    // parse arguments
    let args = Arguments::parse();
    let input = match &args.input {
        Some(val) => Some(Image::open(val).context("Failed to load file")?),
        None => None,
    };
    let output = args.output.clone();
    let effects = match &args.effects {
        Some(val) => Some(Effects::load(val).context("Failed to load steps")?),
        None => None,
    };

    // apply
    if let Some(manual) = args.manual
        && manual
    {
        // running manually
        let manual = Manual::new(input, output, effects);
        manual.run()?;
    } else {
        // TODO: if let some
        // running in script
        if effects.is_none() || input.is_none() || output.is_none() {
            error!(
                "You have to specify image input and output and effects file when running non-interactively"
            );
            return Err(Error::msg("Invalid input arguments"));
        }

        let mut input = input.unwrap();
        let output = output.unwrap();
        let effects = effects.unwrap();

        input.effects(&effects);

        if fs::exists(&output).unwrap_or(true) {
            warn!("File '{}' already exists and will be overwritten", &output);
        }

        if let Err(e) = input.save(output) {
            error!("Error while saving image: '{}'", e)
        }
    }

    Ok(())
}
