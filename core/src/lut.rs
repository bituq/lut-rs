use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

pub struct Lut {
    lut: Vec<[u8; 3]>,
    lut_size: usize,
}

impl Lut {
    pub fn new(lut: Vec<[u8; 3]>) -> Self {
		let lut_size = (lut.len() as f64).cbrt().round() as usize;
		
        Lut { lut, lut_size }
    }

    fn clamp_and_scale(&self, value: u8) -> u32 {
        ((value as f32 / 255.0 * (self.lut_size as f32 - 1.0)).round() as u32).min(self.lut_size as u32 - 1)
    }

    pub fn apply_to_color(&self, color: [u8; 3]) -> [u8; 3] {
        let indices: Vec<u32> = color
            .iter()
            .map(|&channel| self.clamp_and_scale(channel))
            .collect();

        let index: usize = (indices[0]
            + indices[1] * self.lut_size as u32
            + indices[2] * self.lut_size as u32 * self.lut_size as u32)
            as usize;

        self.lut[index]
    }
}

pub fn apply_lut(image: &DynamicImage, lut: &Lut) -> Result<DynamicImage, &'static str> {
    let (width, height) = image.dimensions();
    let mut output_image = ImageBuffer::new(width, height);

    output_image
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let input_pixel = image.get_pixel(x, y);

            // Extract the RGB channels from the Rgba pixel
            let input_rgb = [input_pixel[0], input_pixel[1], input_pixel[2]];

            let lut_value = lut.apply_to_color(input_rgb);
            *pixel = Rgb(lut_value);
        });

    Ok(DynamicImage::ImageRgb8(output_image))
}