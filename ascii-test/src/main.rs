#[cfg(not(feature = "terminal"))]
fn main() {}

#[cfg(feature = "test")]
mod test_values;
#[cfg(feature = "terminal")]
use ascii_izer::put_in_console;

#[cfg(feature = "test")]
#[cfg(test)]
mod test {
    use crate::test_values::{
        create_correct_ascii, create_correct_ascii_small, create_correct_vec, create_correct_strings,
    };
    use ascii_izer::{to_ascii, to_chars, to_gray_vector, ASCIIGenerator};

    #[test]
    fn to_bits_test() {
        // We cheat and use strings here because giant vectors aren't good for "cargo check"
        let correct = create_correct_vec();
        let image = format!(
            "{:#?}",
            to_gray_vector("../cuddlyferris.png").expect("could not process image")
        );
        assert_eq!(correct, image);
    }

    #[test]
    fn to_chars_test() {
        let bits = to_gray_vector("../cuddlyferris.png").expect("could not process image");

        let correct = create_correct_ascii();
        let result = to_chars(bits);
        assert_eq!(correct, result);
    }

    #[test]
    fn to_ascii_test() {
        let correct = create_correct_ascii_small();
        let result = to_ascii("../cuddlyferris.png", (230, 172)).expect("could not process image");
        assert_eq!(correct, result);
    }

    #[test]
    fn generator_string_test() {
        let correct: Vec<String> = create_correct_strings();
        let mut generator = ASCIIGenerator::default();
        generator.set_image("../cuddlyferris.png");
        assert_eq!(correct, generator.build().expect("Failed to process image"));
    }
}

#[cfg(feature = "terminal")]
fn main() {
    let _ = put_in_console("../cuddlyferris.png");
}
