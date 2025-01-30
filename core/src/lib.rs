use std::path::Path;

use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageReader};
pub use error::AsciiError;
#[cfg(feature = "crossterm")]
pub use terminal::put_in_console;

mod error;
mod terminal;

pub fn to_gray_vector<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AsciiError> {
    let mut buffer = Vec::new();
    let image = ImageReader::open(path)?.decode()?;

    for (_, _, color) in image.grayscale().pixels() {
        buffer.push(*color.0.first().expect("color is missing."));
    }

    Ok(buffer)
}

pub fn to_gray_vector_from_image(image: &DynamicImage) -> Result<Vec<u8>, AsciiError> {
    let mut buffer = Vec::new();

    for (_, _, color) in image.grayscale().pixels() {
        buffer.push(*color.0.first().expect("color is missing"));
    }

    Ok(buffer)
}

pub fn to_chars(gray_vector: Vec<u8>) -> Vec<char> {
    let mut chars = Vec::with_capacity(gray_vector.len());

    for level in gray_vector {
        if level > 230 {
            chars.push(' ');
        } else if level >= 200 {
            chars.push('.');
        } else if level >= 180 {
            chars.push('*');
        } else if level >= 160 {
            chars.push(':');
        } else if level >= 130 {
            chars.push('o');
        } else if level >= 100 {
            chars.push('&');
        } else if level >= 70 {
            chars.push('8');
        } else if level >= 50 {
            chars.push('#');
        } else {
            chars.push('@');
        }
    }

    chars
}

pub fn to_strings(gray_vector: Vec<u8>, division: usize) -> Vec<String> {
    let mut strings = Vec::with_capacity(gray_vector.len() / division);
    strings.push(String::with_capacity(division));
    let mut place = 0;
    let mut index = 0;

    for level in gray_vector {
        let string: &mut String = strings.get_mut(index).unwrap();
        if level > 230 {
            string.push(' ');
        } else if level >= 200 {
            string.push('.');
        } else if level >= 180 {
            string.push('*');
        } else if level >= 160 {
            string.push(':');
        } else if level >= 130 {
            string.push('o');
        } else if level >= 100 {
            string.push('&');
        } else if level >= 70 {
            string.push('8');
        } else if level >= 50 {
            string.push('#');
        } else {
            string.push('@');
        }

        place += 1;
        if place == division - 1 {
            strings.push(String::with_capacity(division));
            index += 1;
        }
    }

    strings
}

pub fn to_ascii<P: AsRef<Path>>(
    image_path: P,
    dimensions: (u32, u32),
) -> Result<Vec<char>, AsciiError> {
    let mut image = ImageReader::open(image_path)?.decode()?;
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let gray_vector = to_gray_vector_from_image(&image)?;
    let ascii_result = to_chars(gray_vector);

    Ok(ascii_result)
}

pub fn to_ascii_string<P: AsRef<Path>>(image_path: P, dimensions: (u32, u32)) -> Result<Vec<String>, AsciiError> {
    let mut image = ImageReader::open(image_path)?.decode()?;
    image = image.resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3);

    let gray_vector = to_gray_vector_from_image(&image)?;
    let ascii_result = to_strings(gray_vector.clone(), dimensions.0 as usize);

    Ok(ascii_result)
}
