// Vigenere cipher is really just a series of Caesar ciphers
// So we can compute the Caesar tables just like the caesar cipher implementation
use pyo3::exceptions::*;
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
pub enum Mode {
    Encrypt,
    Decrypt,
}

// All Caesar Lookup Tables
static CAESAR_TABLES: [[u8; 256]; 26] = build_all_tables();

// The exposed python functions
#[pyfunction]
#[pyo3(signature = (data, key, skip_non_alpha=true))]
pub fn encrypt(data: &str, key: &str, skip_non_alpha: bool) -> PyResult<String> {
    vigenere_rotate(data, key, Mode::Encrypt, skip_non_alpha)
}

#[pyfunction]
#[pyo3(signature = (data, key, skip_non_alpha=true))]
pub fn decrypt(data: &str, key: &str, skip_non_alpha: bool) -> PyResult<String> {
    vigenere_rotate(data, key, Mode::Decrypt, skip_non_alpha)
}

// Main Vigenere function
pub fn vigenere_rotate(
    data: &str,
    key: &str,
    mode: Mode,
    skip_non_alpha: bool,
) -> PyResult<String> {
    // Validate key
    if !data.is_ascii() || !key.is_ascii() {
        return Err(PyValueError::new_err("Only ASCII input is supported"));
    }
    if !key.chars().all(|c| c.is_ascii_alphabetic()) || key.is_empty() {
        return Err(PyValueError::new_err(
            "Key must be non-empty and contain only alphabetic characters",
        ));
    }
    // Convert key to shifts
    let key_shifts: Vec<u8> = key
        .chars()
        .map(|c| {
            let lower_c = c.to_ascii_lowercase();
            let shift = lower_c as u8 - b'a';
            match mode {
                Mode::Encrypt => shift,
                Mode::Decrypt => (26 - shift) % 26,
            }
        })
        .collect();

    let key_len = key_shifts.len();
    let input = data.as_bytes();
    let mut out = vec![0; input.len()];
    let mut key_i = 0usize;
    unsafe {
        out.set_len(input.len());
        // Main loop
        for i in 0..input.len() {
            let b = *input.get_unchecked(i);
            let shift = *key_shifts.get_unchecked(key_i % key_len);
            let rotated = single_rotate(b, shift);
            *out.get_unchecked_mut(i) = rotated;
            if !skip_non_alpha || rotated != b {
                key_i += 1;
            }
        }
        // Return result
        Ok(String::from_utf8_unchecked(out))
    }
}

// Implementation of single rotate on a single character
// As the end user will not be able to use this, we can assume no errors
#[inline(always)]
fn single_rotate(data: u8, shift: u8) -> u8 {
    // No need for shift validation as no one outside this module can call this function
    unsafe {
        *CAESAR_TABLES
            .get_unchecked(shift as usize)
            .get_unchecked(data as usize)
    }
}
