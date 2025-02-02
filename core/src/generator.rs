use std::path::Path;

use image::{imageops::FilterType, ImageReader};

use crate::{color::GrayscaleMode, image_into_lines, ASCIIError, Line};

/// A struct used to control the specifics of how an image is converted into
/// ASCII.
///
/// _Note: Color functions are only available with the "color" feature._
#[derive(Default)]
pub struct ASCIIGenerator {
    #[cfg(feature = "color")]
    color: bool,
    dimensions: (u32, u32),
}

impl ASCIIGenerator {
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    #[cfg(feature = "color")]
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

    /// Using the current settings, generates image into ASCII.
    ///
    /// Throws an [ASCIIError](crate::error::ASCIIError) if the image at the
    /// provided path cannot be found or read.
    pub fn generate<P: AsRef<Path>>(&self, image_path: P) -> Result<Vec<Line>, ASCIIError> {
        let image = ImageReader::open(image_path)?.decode()?;

        if self.dimensions != (0, 0) {
            let _ = image.resize_exact(self.dimensions.0, self.dimensions.1, FilterType::Lanczos3);
        }
        Ok(image_into_lines(
            &image,
            GrayscaleMode::Luminosity,
            #[cfg(feature = "color")]
            self.color,
        )?)
    }
}
