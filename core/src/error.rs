use std::{error::Error, fmt::Display, io};

use image::ImageError;

#[derive(Debug)]
pub enum ASCIIError {
    ImageError(ImageError),
    IOError(io::Error),
}

impl Error for ASCIIError {}

impl Display for ASCIIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASCIIError::ImageError(error) => {
                write!(f, "{error}")
            }
            ASCIIError::IOError(error) => {
                write!(f, "{error}")
            }
        }
    }
}

impl From<ImageError> for ASCIIError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}

impl From<io::Error> for ASCIIError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
