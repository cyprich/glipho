use std::{
    cmp::{max, min},
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result};
use image::{RgbImage, RgbaImage};
use log::{debug, info};

use crate::{Effect, Effects};

#[derive(Debug, Clone, PartialEq)]
enum ImageFormat {
    Jpg,
    Png,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u8>, // RGBA8 = R, G, B, A, R, G, B, A, ...
    path: PathBuf,
    format: ImageFormat,
}

impl Image {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let time = Instant::now();

        debug!("Reading '{}'", path.to_str().unwrap_or("image"));
        let img = image::open(path).context("Failed to load image")?;

        debug!("Converting to RGBA8");
        let img = img.into_rgba8();

        debug!(
            "Loaded '{}' in {}s",
            path.to_str().unwrap_or("image"),
            time.elapsed().as_secs_f32()
        );

        let format = if let Some(val) = path.to_string_lossy().rsplit('.').next() {
            match val.to_lowercase().as_str() {
                "jpg" | "jpeg" => ImageFormat::Jpg,
                "png" => ImageFormat::Png,
                _ => ImageFormat::Other,
            }
        } else {
            ImageFormat::Other
        };

        Ok(Self {
            width: img.width(),
            height: img.height(),
            pixels: img.into_raw(),
            path: path.into(),
            format,
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<&Self> {
        let time = Instant::now();

        // TODO: remove the exports?
        if fs::exists(&path).unwrap_or(false) {
            fs::remove_file(&path).context("Failed to remove old image")?;
        }

        match self.format {
            ImageFormat::Jpg => {
                let mut pixels = Vec::with_capacity((self.width * self.height * 3) as usize);
                self.pixels.chunks_exact(4).for_each(|p| {
                    pixels.push(p[0]);
                    pixels.push(p[1]);
                    pixels.push(p[2]);
                });

                let img = RgbImage::from_raw(self.width, self.height, pixels)
                    .context("Failed to construct RGB Image")?;
                img.save(&path).context("Failed to save an Image")?;
            }
            _ => {
                let img = RgbaImage::from_raw(self.width, self.height, self.pixels.clone())
                    .context("Failed to construct RGBA Image")?;
                img.save(&path).context("Failed to save an Image")?;
            }
        }

        info!(
            "Saved '{}' in {}s",
            path.as_ref().to_str().unwrap_or("image"),
            time.elapsed().as_secs_f32()
        );

        Ok(self)
    }

    pub fn effect(&mut self, layer: &Effect) -> &mut Self {
        let time = Instant::now();
        let result = match layer {
            Effect::Brightness(val) => match val >= &0i16 {
                true => self.apply_closure(|x| *x = x.saturating_add(*val as u8)),
                false => self.apply_closure(|x| *x = x.saturating_sub(*val as u8)),
            },
            Effect::WrapBrightness(val) => match val >= &0i16 {
                true => self.apply_closure(|x| *x = x.wrapping_add(*val as u8)),
                false => self.apply_closure(|x| *x = x.wrapping_sub(*val as u8)),
            },
            Effect::Invert => self.apply_closure(|x| *x = 255 - *x),
            Effect::ReverseBits => self.apply_closure(|x| *x = x.reverse_bits()),
            Effect::Min(val) => self.apply_closure(|x| *x = max(*x, *val)),
            Effect::Max(val) => self.apply_closure(|x| *x = min(*x, *val)),
        };

        info!(
            "Applied layer '{}' in {}s",
            layer,
            time.elapsed().as_secs_f32()
        );

        result
    }

    pub fn effects(&mut self, effects: &Effects) -> &mut Self {
        let time = Instant::now();
        for l in &effects.inner {
            self.effect(l);
        }
        info!(
            "Applied {} layers in {}s",
            &effects.inner.len(),
            time.elapsed().as_secs_f32()
        );
        self
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn apply_closure<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut u8),
    {
        for p in self.pixels.chunks_exact_mut(4) {
            f(&mut p[0]);
            f(&mut p[1]);
            f(&mut p[2]);
        }

        self
    }
}
