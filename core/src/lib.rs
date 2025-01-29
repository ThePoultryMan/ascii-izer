use std::path::Path;

pub use error::AsciiError;
use image::{GenericImageView, ImageReader};

mod error;

pub fn to_gray_vector<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AsciiError> {
    let mut buffer = Vec::new();
    let image = ImageReader::open(path)?.decode()?;

    for (_, _, color) in image.grayscale().pixels() {
        buffer.push(*color.0.get(0).expect("color is missing."));
    }

    Ok(buffer)
}

pub fn to_chars(gray_vector: Vec<u8>) -> Vec<char> {
    let mut chars = Vec::with_capacity(gray_vector.len());

    for level in gray_vector {
        if level > 230 {
            chars.push(' ');
        }
        else if level >= 200 {
            chars.push('.');
        }
        else if level >= 180 {
            chars.push('*');
        }
        else if level >= 160 {
            chars.push(':');
        }
        else if level >= 130 {
            chars.push('o');
        }
        else if level >= 100 {
            chars.push('&');
        }
        else if level >= 70 {
            chars.push('8');
        }
        else if level >= 50 {
            chars.push('#');
        }
        else {
            chars.push('@');
        }
    }

    chars
}
