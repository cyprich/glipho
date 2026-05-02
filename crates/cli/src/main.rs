use ::clap::Parser;
use anyhow::{Context, Result};
use lib::{Image, Layer};

use crate::clap::Cli;

mod clap;
mod interactive;

fn main() -> Result<()> {
    // TODO mkdir exports
    simple_logger::init().context("Failed to initialize Simple Logger")?;

    let cli = Cli::parse();
    if let Some(val) = &cli.interactive
        && *val
    {
        crate::interactive::run()?;
        return Ok(());
    } else {
        println!("noninteractive");
    }

    // load image
    let img = Image::from_file("sample.jpg")?;

    img.clone()
        .layer(&Layer::WrapBrightness(-20))
        .save("step1.jpg")?
        .layer(&Layer::Invert)
        .save("step2.jpg")?;

    img.clone().layer(&Layer::ReverseBits).save("pokus.jpg")?;

    // img.clone()
    //     .layer(&Layer::Brightness(-50))
    //     .save("bright.jpg")?;

    // img.clone()
    //     .layer(&Layer::WrapBrightness(-50))
    //     .save("wrap.jpg")?;

    Ok(())
}
