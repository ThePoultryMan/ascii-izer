use std::path::Path;

use image::{imageops::FilterType, ImageReader};

use crate::{color::GrayscaleMode, image_into_lines, ASCIIError, Line};

/// A struct used to control the specifics of how an image is converted into
/// ASCII.
///
/// _Note: Color functions are only available with the "color" feature._
#[derive(Default)]
pub struct ASCIIGenerator {
    resize_mode: ResizeMode,
    #[cfg(feature = "color")]
    color: bool,
}

/// The type of resizing that the image will undergo.
#[derive(Default)]
pub enum ResizeMode {
    /// No resizing will occur.
    #[default]
    None,
    /// The image is scaled to the maximum size that fits within the provided
    /// size. Aspect ratio is preserved.
    Normal((u32, u32)),
    /// The image is resized exactly to dimensions provided. Aspect ration is
    /// _not_ preserved.
    Exact((u32, u32)),
    /// The image is scaled to fit within the larger bound, then cropped to fit
    /// within the smaller. Preserves aspect ratio.
    Fill((u32, u32)),
}

impl ASCIIGenerator {
    /// Sets the resize mode used when the ASCII is generated.
    ///
    /// See [ResizeMode]
    pub fn set_resize_mod(&mut self, resize_mode: ResizeMode) -> &mut Self {
        self.resize_mode = resize_mode;
        self
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    #[cfg(feature = "color")]
    /// Sets whether or not to generate the art with color.
    pub fn set_color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    /// Using the current settings, generates image into ASCII.
    ///
    /// Throws an [ASCIIError](crate::error::ASCIIError) if the image at the
    /// provided path cannot be found or read.
    pub fn generate<P: AsRef<Path>>(&self, image_path: P) -> Result<Vec<Line>, ASCIIError> {
        let mut image = ImageReader::open(image_path)?.decode()?;

        match self.resize_mode {
            ResizeMode::None => {}
            ResizeMode::Normal((width, height)) => {
                image = image.resize(width, height, FilterType::Lanczos3)
            }
            ResizeMode::Exact((width, height)) => {
                image = image.resize_exact(width, height, FilterType::Lanczos3)
            }
            ResizeMode::Fill((width, height)) => {
                image = image.resize_to_fill(width, height, FilterType::Lanczos3)
            }
        }
        image_into_lines(
            &image,
            GrayscaleMode::Luminosity,
            #[cfg(feature = "color")]
            self.color,
        )
    }
}
