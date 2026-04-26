use anyhow::Result;
use lib::{Image, Layer};

fn main() -> Result<()> {
    // TODO mkdir exports

    let mut img2 = Image::from_file("sample.jpg")?;
    //
    // let jump = 2; // make every second image
    // for i in 1..=(255 / jump) {
    //     img2.layer(Layer::AddWrapping(jump))
    //         .save(&format!("exports/i-wrap-{}.jpg", jump * i))?;
    //     // img3.layer(Layer::AddSaturating(16))
    //     //     .save(&format!("exports/i-satu-{}.jpg", 16 * i))?;
    // }

    Image::from_file("sample.jpg")?
        .layer(Layer::Invert)
        .save("exports/inv.jpg");

    Image::from_file("sample.jpg")?
        .layer(Layer::AddWrapping(128))
        .save("exports/wra.jpg");

    Image::from_file("sample.jpg")?
        .layer(Layer::Invert)
        .layer(Layer::AddWrapping(128))
        .save("exports/invwra.jpg");

    Image::from_file("sample.jpg")?
        .layer(Layer::AddWrapping(128))
        .layer(Layer::Invert)
        .save("exports/wrainv.jpg");

    Ok(())
}
