// This ROT13 implementation uses a single precomputed lookup table for speed
// The 2 exposed python functions both call the same internal function, as ROT13 is symmetric
use pyo3::prelude::*;

// Precomputed ROT13 lookup table
static ROT13_TABLE: [u8; 256] = {
    let mut table: [u8; 256] = [0u8; 256];
    let mut i: usize = 0;
    while i < 256 {
        let char: u8 = i as u8;
        table[i] = if char >= b'a' && char <= b'z' {
            (char - b'a' + 13_u8) % 26 + b'a'
        } else if char >= b'A' && char <= b'Z' {
            (char - b'A' + 13_u8) % 26 + b'A'
        } else {
            char
        };
        i += 1;
    }
    table
};

// The exposed python functions
#[pyfunction]
pub fn encrypt(input: &str) -> PyResult<String> {
    rotate(input)
}

#[pyfunction]
pub fn decrypt(input: &str) -> PyResult<String> {
    rotate(input)
}

// Main ROT13 logic
fn rotate(input: &str) -> PyResult<String> {
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        result.push(ROT13_TABLE[byte as usize]);
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(result) };
    Ok(result_string)
}
