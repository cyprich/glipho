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
        // running in script
        if let Some(mut i) = input
            && let Some(o) = output
            && let Some(e) = effects
            && !e.is_empty()
        {
            i.effects(&e);

            if fs::exists(&o).unwrap_or(true) {
                warn!("File '{}' already exists and will be overwritten", &o);
            }

            if let Err(e) = i.save(o) {
                error!("Error while saving image: '{}'", e)
            }
        } else {
            error!(
                "You have to specify input, output and at least one effect when running non-interactively"
            );
            return Err(Error::msg("Invalid input arguments"));
        }
    }

    Ok(())
}
