use std::path::Path;

pub use error::AsciiError;
use image::{GenericImageView, ImageReader};

mod error;
mod other;

fn to_bits<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AsciiError> {
    let mut buffer = Vec::new();
    let image = ImageReader::open(path)?.decode()?;

    for (_, _, color) in image.grayscale().pixels() {
        buffer.push(*color.0.get(0).expect("color is missing."));
    }

    Ok(buffer)
}

fn to_chars(bits: Vec<u8>) -> Vec<char> {
    let mut chars = Vec::with_capacity(bits.len());

    for level in bits {
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

#[cfg(test)]
mod test {
    use crate::{to_bits, to_chars};
    use crate::other::create_correct_vec;

    #[test]
    fn to_bits_test() {
        // We cheat and use strings here because giant vectors aren't good for "cargo check"
        let correct = create_correct_vec();
        let image = format!("{:#?}", to_bits("./cuddlyferris.png").expect("could not process image"));
        assert_eq!(correct, image);
    }

    #[test]
    fn to_chars_test() {
        let bits = to_bits("./cuddlyferris.png").expect("could not process image");

        let correct: Vec<char> = Vec::new();
        let result = to_chars(bits);
        assert_eq!(correct, result);
    }
}
