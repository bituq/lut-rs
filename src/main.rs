use std::env;
use std::fs;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        return;
    }

    let dir_path = &args[1];

    let lut_image = match image::load_from_memory(include_bytes!("../lut.png")) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error opening LUT PNG file: {}", e);
            return;
        }
    };

    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("cube")) {
            let cube_file = match fs::read_to_string(&path) {
                Ok(cube) => cube,
                Err(e) => {
                    eprintln!("Error reading .cube file {}: {}", path.display(), e);
                    continue;
                }
            };

            let (lut, lut_size) = match parse_cube_file(&cube_file) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error parsing .cube file {}: {}", path.display(), e);
                    continue;
                }
            };

            let output_image = apply_lut(&lut_image, &lut, lut_size);
            let output_path = path.with_extension("png");
            if let Err(e) = output_image.save(&output_path) {
                eprintln!("Error saving output image {}: {}", output_path.display(), e);
            }
        }
    }
}

fn parse_cube_file(cube_file: &str) -> Result<(Vec<[u8; 3]>, usize), &'static str> {
    // Parse the .cube file and return a LUT as a vector of [u8; 3]
    // This is a simplified parser that assumes the .cube file has a valid format
    let mut lut = Vec::new();
    let mut lut_size: Option<usize> = None;

    for line in cube_file.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let values: Vec<&str> = line.split_whitespace().collect();

        if values.len() == 2 && values[0] == "LUT_3D_SIZE" {
            lut_size = Some(values[1].parse::<usize>().unwrap_or(0));
        } else if values.len() == 3 {
            let r = (values[0].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
            let g = (values[1].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
            let b = (values[2].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
            lut.push([r, g, b]);
        }
    }

    
    if let Some(size) = lut_size {
        if lut.len() != size.pow(3) {
            return Err("Invalid LUT size");
        }
    } else {
        return Err("LUT_3D_SIZE not found");
    }

    Ok((lut, lut_size.unwrap()))
}

fn clamp_and_scale(value: u8, lut_size: usize) -> u32 {
    ((value as f32 / 255.0 * (lut_size as f32 - 1.0)).round() as u32).min(lut_size as u32 - 1)
}

fn apply_lut(image: &DynamicImage, lut: &[[u8; 3]], lut_size: usize) -> DynamicImage {
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
