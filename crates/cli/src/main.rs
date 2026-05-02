use std::time::Instant;

use ::clap::Parser;
use anyhow::{Context, Error, Result};
use lib::{Image, Steps};
use log::{error, info};

// use plotters::prelude::*;

use crate::clap::Cli;

mod clap;
mod interactive;

fn main() -> Result<()> {
    let time = Instant::now();
    // TODO mkdir exports
    simple_logger::init().context("Failed to initialize Simple Logger")?;

    // get filenames from arguments
    let cli = Cli::parse();
    let steps = match &cli.steps_file {
        Some(val) => Some(Steps::from_file(val).context("Failed to load steps")?),
        None => None,
    };
    let image = match &cli.image_file {
        Some(val) => Some(Image::from_file(val).context("Failed to load file")?),
        None => None,
    };

    // apply
    if let Some(interactive) = cli.interactive
        && interactive
    {
        crate::interactive::run(steps, image)?;
    } else {
        if steps.is_none() || image.is_none() {
            error!("You have to specify image file and steps file when running non-interactively");
            return Err(Error::msg("Invalid input arguments"));
        }

        // chart(&image, "before.png")?;
        image.unwrap().steps(&steps.unwrap());
        // chart(&image, "after.png")?;
    }

    info!("Program finished in {}s", time.elapsed().as_secs_f32());
    Ok(())
}

// fn chart(image: &Image, name: &str) -> Result<()> {
//     info!("Starting charts");
//
//     let mut x: Vec<i32> = vec![];
//     let mut r: Vec<i32> = vec![];
//     let mut g: Vec<i32> = vec![];
//     let mut b: Vec<i32> = vec![];
//
//     for (i, pixel) in image.pixels().unwrap().enumerate() {
//         x.push(i as i32);
//         r.push(pixel.0[0].into());
//         g.push(pixel.0[1].into());
//         b.push(pixel.0[2].into());
//     }
//
//     let root = BitMapBackend::new(name, (2000, 1000)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root).build_cartesian_2d(0..x.len(), 0..255)?;
//
//     info!("About to insert data");
//
//     chart.draw_series(LineSeries::new(
//         r.iter().enumerate().map(|(x, y)| (x, *y)),
//         &RED,
//     ))?;
//     info!("R done");
//
//     chart.draw_series(LineSeries::new(
//         g.iter().enumerate().map(|(x, y)| (x, *y)),
//         &GREEN,
//     ))?;
//     info!("G done");
//
//     chart.draw_series(LineSeries::new(
//         b.iter().enumerate().map(|(x, y)| (x, *y)),
//         &BLUE,
//     ))?;
//     info!("B done");
//
//     root.present()?;
//
//     info!("Charts done");
//
//     Ok(())
// }
