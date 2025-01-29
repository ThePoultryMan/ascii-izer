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

#[cfg(test)]
mod test {
    use crate::to_bits;
    use crate::other::create_correct_vec;

    #[test]
    fn to_bits_test() {
        // We cheat and use strings here because giant vectors aren't good for "cargo check"
        let correct = create_correct_vec();
        let image = format!("{:#?}", to_bits("./cuddlyferris.png").expect("could not process image"));
        assert_eq!(correct, image);
    }
}
