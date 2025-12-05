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
pub fn encrypt(data: &str) -> PyResult<String> {
    rot13_rust(data)
}

#[pyfunction]
pub fn decrypt(data: &str) -> PyResult<String> {
    rot13_rust(data)
}

#[pyfunction]
pub fn rotate(data: &str) -> PyResult<String> {
    rot13_rust(data)
}

// Main ROT13 logic
pub fn rot13_rust(data: &str) -> PyResult<String> {
    let bytes = data.as_bytes();
    let mut out: Vec<u8> = vec![0; bytes.len()];
    for i in 0..bytes.len() {
        out[i] = ROT13_TABLE[bytes[i] as usize];
    }
    let result_string: String = String::from_utf8(out)?;
    Ok(result_string)
}
