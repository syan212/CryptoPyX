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

// Special enum for encrypt/decrypt mode
#[derive(Copy, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

// All Caesar Lookup Tables
static CAESAR_TABLES: [[u8; 256]; 26] = {
    let mut tables = [[0u8; 256]; 26];
    let mut shift = 0;
    while shift < 26 {
        tables[shift] = build_table(shift as u8);
        shift += 1;
    }
    tables
};

// The exposed python functions
#[pyfunction]
pub fn encrypt(data: &str, shift: i32) -> PyResult<String> {
    let result: String = encrypt_or_decrypt(data, shift, Mode::Encrypt)?;
    Ok(result)
}

#[pyfunction]
pub fn decrypt(data: &str, shift: i32) -> PyResult<String> {
    let result: String = encrypt_or_decrypt(data, shift, Mode::Decrypt)?;
    Ok(result)
}

// Actual implementation
fn encrypt_or_decrypt(data: &str, shift: i32, mode: Mode) -> PyResult<String> {
    // Validate shift range
    if !(-25..=25).contains(&shift) {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Shift must be in the range -25 to 25",
        ));
    }
    // Convert shift to usize for indexing and handle wrap-around
    let shift: usize = shift.rem_euclid(26) as usize;
    // Compute forwards shift to find correct table
    let forward_shift: usize = match mode {
        Mode::Encrypt => shift,
        Mode::Decrypt => (26 - shift) % 26,
    };
    let input = data.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(input.len());
    // Main encryption/decryption logic inside unsafe block
    unsafe {
        let table: &[u8; 256] = CAESAR_TABLES.get_unchecked(forward_shift);
        for i in 0..input.len() {
            *out.get_unchecked_mut(i) = *table.get_unchecked(*input.get_unchecked(i) as usize);
        }
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(out) };
    Ok(result_string)
}
