#[cfg(test)]
pub fn create_correct_vec() -> String {
    let correct: Vec<u8> = include!("../correct_result.rs");
    format!("{:#?}", correct)
}
