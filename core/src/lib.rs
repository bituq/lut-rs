pub mod cube;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

fn clamp_and_scale(value: u8, lut_size: usize) -> u32 {
    ((value as f32 / 255.0 * (lut_size as f32 - 1.0)).round() as u32).min(lut_size as u32 - 1)
}

pub fn apply_lut(image: &DynamicImage, lut: &[[u8; 3]], lut_size: usize) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut output_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in output_image.enumerate_pixels_mut() {
        let input_pixel = image.get_pixel(x, y);
        
        let indices: Vec<u32> = input_pixel
            .0
            .iter()
            .map(|&channel| clamp_and_scale(channel, lut_size))
            .collect();

        let index: usize = (indices[0]
            + indices[1] * lut_size as u32
            + indices[2] * lut_size as u32 * lut_size as u32)
            as usize;

        let lut_value = lut[index];
        *pixel = Rgb([lut_value[0], lut_value[1], lut_value[2]]);
    }

    DynamicImage::ImageRgb8(output_image)
}
