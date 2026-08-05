use slint::{Rgba8Pixel, SharedPixelBuffer};

pub fn image_to_slint(image: Option<&lib::Image>) -> slint::Image {
    if let Some(image) = image {
        let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(image.width(), image.height());
        let pixels = pixel_buffer.make_mut_slice();

        for (i, chunk) in image.pixels().chunks_exact(4).enumerate() {
            pixels[i] = Rgba8Pixel {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            }
        }

        slint::Image::from_rgba8(pixel_buffer)
    } else {
        slint::Image::default()
    }
}
