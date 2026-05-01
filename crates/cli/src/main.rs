use anyhow::{Context, Result};
use lib::{Image, Layer};

fn main() -> Result<()> {
    // TODO mkdir exports
    simple_logger::init().context("Failed to initialize Simple Logger")?;

    // load image
    let img = Image::from_file("sample.jpg")?;

    img.clone()
        .layer(&Layer::WrapBrightness(-20))
        .save("step1.jpg")?
        .layer(&Layer::Invert)
        .save("step2.jpg")?;

    // img.clone()
    //     .layer(&Layer::Brightness(-50))
    //     .save("bright.jpg")?;

    // img.clone()
    //     .layer(&Layer::WrapBrightness(-50))
    //     .save("wrap.jpg")?;

    Ok(())
}
