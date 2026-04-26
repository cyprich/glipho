use crate::image::Image;

impl Image {
    pub(crate) fn add_wrapping(&mut self, val: u8) -> &mut Self {
        if let Some(pixels) = self.img.as_mut_rgb8() {
            for pixel in pixels.pixels_mut() {
                pixel[0] = pixel[0].wrapping_add(val);
                pixel[1] = pixel[1].wrapping_add(val);
                pixel[2] = pixel[2].wrapping_add(val);
            }
        }

        self
    }

    pub(crate) fn add_saturating(&mut self, val: u8) -> &mut Self {
        if let Some(pixels) = self.img.as_mut_rgb8() {
            for pixel in pixels.pixels_mut() {
                pixel[0] = pixel[0].saturating_add(val);
                pixel[1] = pixel[1].saturating_add(val);
                pixel[2] = pixel[2].saturating_add(val);
            }
        }

        self
    }

    pub(crate) fn invert(&mut self) -> &mut Self {
        if let Some(pixels) = self.img.as_mut_rgb8() {
            for pixel in pixels.pixels_mut() {
                pixel[0] = 255 - pixel[0];
                pixel[1] = 255 - pixel[1];
                pixel[2] = 255 - pixel[2];
            }
        }

        self
    }
}
