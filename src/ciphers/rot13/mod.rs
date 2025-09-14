use pyo3::prelude::*;

// Precomputed ROT13 lookup table
static ROT13_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let u8_i = i as u8;
        table[i] = if u8_i >= b'a' && u8_i <= b'z' {
            (((i as u8 - b'a') + 13) % 26) + b'a'
        } else if u8_i >= b'A' && u8_i <= b'Z' {
            (((i as u8 - b'A') + 13) % 26) + b'A'
        } else {
            i as u8
        };
        i += 1;
    }
    table
};

#[pyfunction]
pub fn encrypt(input: &str) -> PyResult<String> {
    rotate(input)
}

#[pyfunction]
pub fn decrypt(input: &str) -> PyResult<String> {
    rotate(input)
}

fn rotate(input: &str) -> PyResult<String> {
    // Main ROT13 logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        result.push(ROT13_TABLE[byte as usize]);
    }
    let result_string: String = String::from_utf8(result)?;
    Ok(result_string)
}