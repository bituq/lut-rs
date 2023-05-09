mod lib;

use std::fs;
use clap::ArgAction;
use clap::Command;
use clap::{Arg};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use lib::cube::parse_cube_file;
use crate::lib::cube;

fn main() {
    let matches = Command::new("LUT Image Processor")
        .version("1.0")
        .author("Dylan N <zealbus@outlook.com>")
        .about("Applies LUTs from .cube files to images")
        .arg(
            Arg::new("directory")
                .value_name("DIRECTORY")
                .help("Sets the directory containing .cube files")
                .required(true),
        )
        .arg(
            Arg::new("image")
                .value_name("IMAGE")
                .help("Sets the input image file")
                .required(true),
        )
        .arg(
            Arg::new("reshade-preset")
                .long("reshade-preset")
                .action(ArgAction::SetTrue)
                .help("Create a ReShade preset file"),
        )
        .get_matches();

    let dir_path = matches.get_one::<String>("directory").unwrap();
    let lut_image_path = matches.get_one::<String>("image").unwrap();
    let reshade_preset = matches.get_one::<bool>("reshade-preset").unwrap();

    let lut_image = match image::open(lut_image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error decoding PNG file: {}", e);
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

    for entry in entries.filter_map(Result::ok) {
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
                    eprintln!("Error parsing .cube file {}: {:?}", path.display(), e);
                    continue;
                }
            };

            let output_image = apply_lut(&lut_image, &lut, lut_size);
            let output_path = path.with_extension("png");
            if let Err(e) = output_image.save(&output_path) {
                eprintln!("Error saving output image {}: {}", output_path.display(), e);
            }
            if *reshade_preset {
                let ini_path = path.with_extension("ini");
                let lut_name = output_path.file_name().unwrap().to_str().unwrap();
                let ini_content = format!(
                    "PreprocessorDefinitions=fLUT_TextureName=\"{lut_name}\"\nTechniques=LUT@LUT.fx"
                );
                if let Err(e) = fs::write(ini_path, ini_content) {
                    eprintln!("Error writing ReShade preset file: {}", e);
                }
            }
        }
    }
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
