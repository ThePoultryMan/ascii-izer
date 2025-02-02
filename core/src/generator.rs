use std::path::{Path, PathBuf};

use image::{imageops::FilterType, ImageReader};

use crate::{color::GrayscaleMode, image_into_lines, ASCIIError, Line};

/// A struct used to control the specifics of how an image is converted into
/// ASCII.
///
/// _Note: Color functions are only available with the "color" feature._
#[derive(Default)]
pub struct ASCIIGenerator {
    image_path: Option<PathBuf>,
    #[cfg(feature = "color")]
    color: bool,
    dimensions: (u32, u32),
}

impl ASCIIGenerator {
    /// Sets the image that is used to generate art.
    pub fn set_image<P: AsRef<Path>>(&mut self, image: P) -> &mut Self {
        self.image_path = Some(image.as_ref().to_path_buf());
        self
    }

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

    /// Generates the art into a [Vec\<String\>](Vec<String>).
    pub fn build(&self) -> Result<Vec<Line>, AsciiError> {
        if let Some(image_path) = &self.image_path {
            let image = ImageReader::open(image_path)?.decode()?;

            if self.dimensions != (0, 0) {
                let _ =
                    image.resize_exact(self.dimensions.0, self.dimensions.1, FilterType::Lanczos3);
            }
            Ok(image_into_lines(
                &image,
                GrayscaleMode::Luminosity,
                #[cfg(feature = "color")]
                self.color,
            )?)
        } else {
            Err(AsciiError::NoImage)
        }
    }
}
