#![cfg_attr(docsrs, feature(doc_cfg))]

//! > **An image to ASCII art library for rust**
//! ## Features
//! * Color \[default\]: Processes the image color alongside the pixels during
//!   ASCII generation.
//! * Crossterm: Provides a simple method to put the ASCII into the console via
//!   crossterm. This does not output color
//! 
//! ## Usage
//! ### Simple
//! The functions [to_ascii_lines] and [image_into_lines] are the simplest way to
//! convert an image into ASCII. Keep in mind, however, the dimensions taken by
//! these functions are the exact level that the input image will be resized to,
//! aspect ration is not respected.
//! 
//! ### Advanced
//! The [ASCIIGenerator] struct can be used to have more control over the exact
//! parameters used.

use std::path::Path;

#[cfg(feature = "color")]
use color::Color;
use color::GrayscaleMode;
use image::{DynamicImage, GenericImageView, ImageReader};
#[cfg(feature = "crossterm")]
pub use terminal::put_in_console;
pub use {error::ASCIIError, generator::ASCIIGenerator};

mod color;
mod error;
mod generator;
mod terminal;

#[derive(Debug, Clone)]
pub struct Line {
    chars: Vec<char>,
    #[cfg(feature = "color")]
    colors: Vec<Color>,
}

impl Line {
    pub fn new(size: usize) -> Self {
        Line {
            chars: Vec::with_capacity(size),
            #[cfg(feature = "color")]
            colors: vec![Color::white(); size],
        }
    }

    pub fn add_char(&mut self, char: char) {
        self.chars.push(char);
    }

    pub fn add_color(&mut self, color: Color) {
        self.colors.push(color);
    }

    pub fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    #[cfg(feature = "color")]
    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }
}

pub fn to_ascii_lines<P: AsRef<Path>>(
    path: P,
    grayscale_mode: GrayscaleMode,
    #[cfg(feature = "color")] with_color: bool,
) -> Result<Vec<Line>, ASCIIError> {
    let image = ImageReader::open(path)?.decode()?;
    image_into_lines(
        &image,
        grayscale_mode,
        #[cfg(feature = "color")]
        with_color,
    )
}

pub fn image_into_lines(
    image: &DynamicImage,
    grayscale_mode: GrayscaleMode,
    #[cfg(feature = "color")] with_color: bool,
) -> Result<Vec<Line>, ASCIIError> {
    let mut lines: Vec<Line> =
        vec![Line::new(image.dimensions().0 as usize); image.dimensions().1 as usize];
    for (_, y, color) in image.pixels() {
        let color = Color::from(color.0);
        let gray = color.grayscale(grayscale_mode);

        lines
            .get_mut(y as usize)
            .unwrap()
            .add_char(char_from_gray(gray));
        if cfg!(feature = "color") && with_color {
            lines.get_mut(y as usize).unwrap().add_color(color);
        }
    }

    Ok(lines)
}

fn char_from_gray(gray: u8) -> char {
    if gray > 230 {
        ' '
    } else if gray >= 200 {
        '.'
    } else if gray >= 180 {
        '*'
    } else if gray >= 160 {
        ':'
    } else if gray >= 130 {
        'o'
    } else if gray >= 100 {
        '&'
    } else if gray >= 70 {
        '8'
    } else if gray >= 50 {
        '#'
    } else {
        '@'
    }
}
