#[cfg(feature = "color")]
pub use color_mod::Color;

#[cfg(feature = "color")]
mod color_mod {
    use super::{to_grayscale, GrayscaleMode};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl Color {
        pub fn white() -> Self {
            Self {
                r: 255,
                g: 255,
                b: 255,
            }
        }

        pub fn grayscale(&self, grayscale_mode: GrayscaleMode) -> u8 {
            to_grayscale(self.r, self.g, self.b, grayscale_mode)
        }
    }

    impl From<[u8; 4]> for Color {
        fn from(value: [u8; 4]) -> Self {
            Self {
                r: value[0],
                g: value[1],
                b: value[2],
            }
        }
    }

    #[cfg(feature = "crossterm")]
    impl From<Color> for crossterm::style::Color {
        fn from(value: Color) -> Self {
            crossterm::style::Color::Rgb {
                r: value.r,
                g: value.g,
                b: value.b,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GrayscaleMode {
    Luminosity,
}

pub fn to_grayscale(r: u8, g: u8, b: u8, grayscale_mode: GrayscaleMode) -> u8 {
    match grayscale_mode {
        GrayscaleMode::Luminosity => {
            (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32).round() as u8
        }
    }
}
