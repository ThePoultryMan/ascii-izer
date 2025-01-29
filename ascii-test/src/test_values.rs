#![cfg(test)]

pub fn create_correct_vec() -> String {
    let correct: Vec<u8> = include!("../correct_result.rs");
    format!("{:#?}", correct)
}

pub fn create_correct_ascii() -> Vec<char> {
    include!("../correct_ascii.rs")
}

pub fn create_correct_ascii_small() -> Vec<char> {
    include!("../correct_ascii_small.rs")
}
