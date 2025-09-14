use pyo3::prelude::*;

#[pyfunction]
pub fn encrypt(input: &str) -> PyResult<String> {
    let result: String = rotate(input)?;
    Ok(result)
}

#[pyfunction]
pub fn decrypt(input: &str) -> PyResult<String> {
    let result: String = rotate(input)?;
    Ok(result)
}

fn rotate(input: &str) -> PyResult<String> {
    // Main ROT13 logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        // ! Note: I used i8 here because of integer undeflow/overflow issues
        let byte_num: i8 = byte as i8;
        let base: i8 = if byte.is_ascii_lowercase() {
            b'a' as i8
        } else if byte.is_ascii_uppercase() {
            b'A' as i8
        } else {
            let shifted_byte: u8 = byte as u8;
            result.push(shifted_byte);
            continue;
        };
        let shifted_byte: u8 = ((byte_num - base + 13).rem_euclid(26) + base) as u8;
        result.push(shifted_byte);
    }
    let result_string: String = String::from_utf8(result)?;
    Ok(result_string)
}