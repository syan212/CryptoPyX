// This Caesar shift implementation uses a precomputed lookup table for speed
// The table is reused in the Vigenere implementation
use pyo3::prelude::*;

// Build table for specific shift
const fn build_table(shift: u8) -> [u8; 256] {
    let mut table: [u8; 256] = [0u8; 256];
    let mut i: usize = 0;
    while i < 256 {
        let char = i as u8;
        table[i] = if char >= b'a' && char <= b'z' {
            (char - b'a' + shift) % 26 + b'a'
        } else if char >= b'A' && char <= b'Z' {
            (char - b'A' + shift) % 26 + b'A'
        } else {
            char
        };
        i += 1;
    }
    table
}

// Build all 26 Caesar tables in a single static
const fn build_all_tables() -> [[u8; 256]; 26] {
    let mut tables = [[0u8; 256]; 26];
    let mut shift = 0;
    while shift < 26 {
        tables[shift] = build_table(shift as u8);
        shift += 1;
    }
    tables
}

// Special enum for encrypt/decrypt mode
#[derive(Copy, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

// All Caesar Lookup Tables
static CAESAR_TABLES: [[u8; 256]; 26] = build_all_tables();

// The exposed python functions
#[pyfunction]
pub fn encrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = encrypt_or_decrypt(input, shift, Mode::Encrypt)?;
    Ok(result)
}

#[pyfunction]
pub fn decrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = encrypt_or_decrypt(input, shift, Mode::Decrypt)?;
    Ok(result)
}

// Actual implementation
fn encrypt_or_decrypt(input: &str, shift: i32, mode: Mode) -> PyResult<String> {
    // Validate shift range
    if (-25..=25).contains(&shift) == false {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Shift must be in the range -25 to 25",
        ));
    }
    // Convert shift to usize for indexing and handle wrap-around
    let shift: usize = shift.rem_euclid(26) as usize;
    // If the shift is 0, just return the string
    if shift == 0 {
        return Ok(String::from(input));
    }
    // Compute forwards shift to find correct table
    let forward_shift = match mode {
        Mode::Encrypt => shift,
        Mode::Decrypt => (26 - shift) % 26,
    };
    let table: [u8; 256] = unsafe { *CAESAR_TABLES.get_unchecked(forward_shift) };
    // Main encryption/decryption logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        result.push(table[byte as usize]);
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(result) };
    Ok(result_string)
}
