use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageReader};

use crate::{to_ascii_from_image, AsciiError};

#[derive(Default)]
pub struct ASCIIGenerator {
    image_path: Option<PathBuf>,
    color: bool,
    dimensions: (u32, u32),
}

impl ASCIIGenerator {
    /// Sets the image that is used to generate art.
    pub fn set_image<P: AsRef<Path>>(&mut self, image: P) -> &mut Self {
        self.image_path = Some(image.as_ref().to_path_buf());
        self
    }

    /// Sets whether or not to generate the art with color.
    pub fn set_color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets the dimensions of the generator image. If left at the default, (0, 0),
    /// dimensions are set to match the image dimensions.
    pub fn set_dimensions(&mut self, dimensions: (u32, u32)) -> &mut Self {
        self.dimensions = dimensions;
        self
    }

    /// Generates the art into a [Vec\<String\>](Vec<String>).
    pub fn build(&self) -> Result<Vec<String>, AsciiError> {
        if let Some(image_path) = &self.image_path {
            let image = ImageReader::open(image_path)?.decode()?;
    
            let fixed_dimensions = if self.dimensions == (0, 0) {
                image.dimensions()
            } else {
                self.dimensions
            };
            Ok(to_ascii_from_image(&image, fixed_dimensions)?)
        } else {
            Err(AsciiError::NoImage)
        }
    }
}
