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

pub fn create_correct_strings() -> Vec<String> {
    include!("../correct_strings.rs").into_iter().map(|string| string.to_owned()).collect()
}
