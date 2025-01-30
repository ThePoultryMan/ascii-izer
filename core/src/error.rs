use std::{error::Error, fmt::Display, io};

use image::ImageError;

#[derive(Debug)]
pub enum AsciiError {
    ImageError(ImageError),
    IOError(io::Error),
    NoImage,
}

impl Error for AsciiError {}

impl Display for AsciiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsciiError::ImageError(error) => {
                write!(f, "{error}")
            }
            AsciiError::IOError(error) => {
                write!(f, "{error}")
            },
            AsciiError::NoImage => {
                write!(f, "You must provide an image to generate from")
            }
        }
    }
}

impl From<ImageError> for AsciiError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<io::Error> for AsciiError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
