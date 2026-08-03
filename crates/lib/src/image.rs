use std::{
    cmp::{max, min},
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result};
use image::{ImageReader, RgbImage, RgbaImage};
use log::{debug, info, trace};
use rayon::{
    iter::ParallelIterator,
    slice::{ParallelSlice, ParallelSliceMut},
};

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
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path_name = path.as_ref().to_string_lossy();
        let path = path.as_ref();
        let time = Instant::now();

        debug!("Reading '{}'", path_name);
        let reader = ImageReader::open(path).context("Failed to read image path")?;
        trace!("Reader done in {}", &time.elapsed().as_secs_f32());
        let reader_format = reader
            .format()
            .context("Failed to determine image format")?;
        trace!("Format done in {}", &time.elapsed().as_secs_f32());

        debug!("Decoding");
        let img = reader.decode().context("Failed to decode image")?;
        trace!("Decode done in {}", &time.elapsed().as_secs_f32());

        debug!("Converting to RGBA8");
        let img = img.into_rgba8();
        trace!("Convert done in {}", &time.elapsed().as_secs_f32());

        // let img = image::open(path)?;
        // let img = img.into_rgba8();

        debug!(
            "Loaded '{}' in {}s",
            path_name,
            time.elapsed().as_secs_f32()
        );

        let format = match reader_format {
            image::ImageFormat::Png => ImageFormat::Png,
            image::ImageFormat::Jpeg => ImageFormat::Jpg,
            _ => ImageFormat::Other,
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

        if fs::exists(&path).unwrap_or(false) {
            fs::remove_file(&path).context("Failed to remove old image")?;
        }

        match self.format {
            ImageFormat::Jpg => {
                let pixels = Vec::with_capacity((self.width * self.height * 3) as usize);

                self.pixels
                    .par_chunks_exact(4)
                    .map(|p| [p[0], p[1], p[2]])
                    .flatten()
                    .collect::<Vec<u8>>();

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
                false => self.apply_closure(|x| *x = x.saturating_sub((val * -1) as u8)),
            },
            Effect::WrapBrightness(val) => match val >= &0i16 {
                true => self.apply_closure(|x| *x = x.wrapping_add(*val as u8)),
                false => self.apply_closure(|x| *x = x.wrapping_sub((val * -1) as u8)),
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
            &effects.len(),
            time.elapsed().as_secs_f32()
        );
        self
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn apply_closure<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&mut u8) + Send + Sync,
    {
        self.pixels.par_chunks_exact_mut(4).for_each(|p| {
            f(&mut p[0]);
            f(&mut p[1]);
            f(&mut p[2]);
        });

        self
    }
}
