mod layer_implementations;

use std::time::Instant;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader};

use crate::Layer;

#[derive(Debug, Clone)]
pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn from_file(path: &str) -> Result<Self> {
        // let x = std::fs::read(path);
        let time = Instant::now();
        let img = ImageReader::open(path).context("Failed to load image")?;
        let img = img.decode().context("Failed to decode image")?;

        Ok(Self { img })
    }

    pub fn save(&self, path: &str) -> Result<()> {
        self.img.save(path)?;
        Ok(())
    }

    pub fn layer(&mut self, layer: Layer) -> &mut Self {
        match layer {
            Layer::AddSaturating(val) => self.add_saturating(val),
            Layer::AddWrapping(val) => self.add_wrapping(val),
            Layer::Invert => self.invert(),
        }
    }
}
