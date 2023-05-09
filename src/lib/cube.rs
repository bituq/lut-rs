use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

#[derive(Debug)]
pub enum CubeFileError {
	InvalidLutSize,
	Lut3dSizeNotFound,
	ParseFloatError(ParseFloatError),
	ParseIntError(ParseIntError),
}

impl From<ParseFloatError> for CubeFileError {
    fn from(err: ParseFloatError) -> CubeFileError {
        CubeFileError::ParseFloatError(err)
    }
}

impl From<ParseIntError> for CubeFileError {
	fn from(err: ParseIntError) -> CubeFileError {
		CubeFileError::ParseIntError(err)
	}
}

pub fn parse_cube_file(cube_file: &str) -> Result<(Vec<[u8; 3]>, usize), CubeFileError> {
    let mut lut = Vec::new();
    let mut lut_size: Option<usize> = None;

    for line in cube_file.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let values: Vec<&str> = line.split_whitespace().collect();

        if values.len() == 2 && values[0] == "LUT_3D_SIZE" {
            lut_size = Some(values[1].parse::<usize>().map_err(CubeFileError::from)?);
        } else if values.len() == 3 {
            let r = (f32::from_str(values[0])? * 255.0).round() as u8;
            let g = (f32::from_str(values[1])? * 255.0).round() as u8;
            let b = (f32::from_str(values[2])? * 255.0).round() as u8;
            lut.push([r, g, b]);
        }
    }

    match lut_size {
        Some(size) if lut.len() == size.pow(3) => Ok((lut, size)),
        Some(_) => Err(CubeFileError::InvalidLutSize),
        None => Err(CubeFileError::Lut3dSizeNotFound),
    }
}