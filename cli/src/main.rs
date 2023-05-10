use std::fs;
use clap::ArgAction;
use clap::Command;
use clap::{Arg};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use lut_rs::apply_lut;
use lut_rs::cube::parse_cube_file;

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
