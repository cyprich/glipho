use std::time::Instant;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader, Rgb, buffer::PixelsMut};
use log::info;

use crate::{Layer, Step, Steps};

#[derive(Debug, Clone)]
pub struct Image {
    img: DynamicImage,
    save_subfolder: String,
}

impl Image {
    pub fn from_file(path: &str) -> Result<Self> {
        let time = Instant::now();
        let img = ImageReader::open(path).context("Failed to load image")?;
        let img = img.decode().context("Failed to decode image")?;

        info!("Loaded '{}' in {}s", path, time.elapsed().as_secs_f32());

        let save_subfolder = String::from("exports");

        Ok(Self {
            img,
            save_subfolder,
        })
    }

    pub fn save(&mut self, path: &str) -> Result<&mut Self> {
        let time = Instant::now();
        self.img.save(format!("{}/{}", self.save_subfolder, path))?;
        info!("Saved '{}' in {}s", path, time.elapsed().as_secs_f32());

        Ok(self)
    }

    pub fn layer(&mut self, layer: &Layer) -> &mut Self {
        let time = Instant::now();
        let result = match layer {
            Layer::Brightness(val) => match val >= &0i16 {
                true => self.apply_closure(|x| *x = x.saturating_add(*val as u8)),
                false => self.apply_closure(|x| *x = x.saturating_sub(*val as u8)),
            },
            Layer::WrapBrightness(val) => match val >= &0i16 {
                true => self.apply_closure(|x| *x = x.wrapping_add(*val as u8)),
                false => self.apply_closure(|x| *x = x.wrapping_sub(*val as u8)),
            },
            Layer::Invert => self.apply_closure(|x| *x = 255 - *x),
            Layer::ReverseBits => self.apply_closure(|x| *x = x.reverse_bits()),
        };

        info!(
            "Applied layer '{}' in {}s",
            layer,
            time.elapsed().as_secs_f32()
        );

        result
    }

    pub fn layers(&mut self, layers: &[Layer]) -> &mut Self {
        let time = Instant::now();
        for l in layers {
            self.layer(l);
        }
        info!(
            "Applied {} layers in {}s",
            layers.len(),
            time.elapsed().as_secs_f32()
        );
        self
    }

    pub fn step(&mut self, step: &Step) -> &mut Self {
        match step {
            Step::Layer(layer) => self.layer(layer),
            Step::Save(filename) => {
                if let Err(val) = self.save(filename) {
                    eprintln!("Failed to save image: {}", val);
                }
                self
            }
        }
    }

    pub fn steps(&mut self, steps: &Steps) -> &mut Self {
        for step in &steps.steps {
            self.step(step);
        }

        self
    }

    pub(crate) fn pixels_mut(&mut self) -> Option<PixelsMut<'_, Rgb<u8>>> {
        self.img.as_mut_rgb8().map(|x| x.pixels_mut())
    }

    pub fn pokus(&mut self) -> &mut Self {
        self.apply_closure(|x| *x = x.saturating_sub(50));
        self
    }

    pub(crate) fn apply_closure<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut u8),
    {
        if let Some(pixels) = self.pixels_mut() {
            for p in pixels {
                f(&mut p[0]);
                f(&mut p[1]);
                f(&mut p[2]);
            }
        }

        self
    }
}
