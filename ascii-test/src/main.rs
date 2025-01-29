#[cfg(not(feature = "test"))]
fn main() {}

#[cfg(feature = "test")]
mod test_values;

#[cfg(feature = "test")]
#[cfg(test)]
mod test {
    use ascii_izer::{to_bits, to_chars};
    use crate::test_values::create_correct_vec;

    #[test]
    fn to_bits_test() {
        // We cheat and use strings here because giant vectors aren't good for "cargo check"
        let correct = create_correct_vec();
        let image = format!(
            "{:#?}",
            to_bits("../cuddlyferris.png").expect("could not process image")
        );
        assert_eq!(correct, image);
    }

    #[test]
    fn to_chars_test() {
        let bits = to_bits("../cuddlyferris.png").expect("could not process image");

        let correct: Vec<char> = Vec::new();
        let result = to_chars(bits);
        assert_eq!(correct, result);
    }
}

#[cfg(feature = "test")]
fn main() {}
