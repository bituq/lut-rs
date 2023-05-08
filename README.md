# lut-rs

A command-line tool to apply LUTs (Look-Up Tables) from `.cube` files to images, written in Rust.

## Features

- Apply LUTs from `.cube` files to images
- Generate ReShade preset files (optional)

## Usage

1. Install Rust and Cargo if you haven't already: https://www.rust-lang.org/tools/install
2. Clone this repository: `git clone https://github.com/bituq/lut-rs.git`
3. Change into the cloned directory: `cd lut-rs`
4. Build the project: `cargo build --release`
5. Run the program with the required arguments:

```
./target/release/lut-rs --directory <DIRECTORY> --image <IMAGE> [--reshade-preset]
```

Replace `<DIRECTORY>` with the path to the directory containing `.cube` files, and `<IMAGE>` with the path to the input image file. Add the `--reshade-preset` flag if you want to generate ReShade preset files.

## Example

```
./target/release/lut-rs --directory ./cube_files --image ./input.png --reshade-preset
```

This will apply LUTs from all `.cube` files in the `./cube_files` directory to the `./lut.png` image and generate ReShade preset files for each entry.