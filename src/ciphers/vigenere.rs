// Vigenere cipher is really just a series of Caesar ciphers
// So we can compute the Caesar tables just like the caesar cipher implementation
use pyo3::prelude::*;

// Build table for specific shift
const fn build_table(shift: i8) -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut byte = 0;
    while byte < 256 {
        let char = byte as i8;
        table[byte] = if byte as u8 >= b'a' && byte as u8 <= b'z' {
            ((char - b'a' as i8 + shift).rem_euclid(26) + b'a' as i8) as u8
        } else if byte as u8 >= b'A' && byte as u8 <= b'Z' {
            ((char - b'A' as i8 + shift).rem_euclid(26) + b'A' as i8) as u8
        } else {
            byte as u8
        };
        byte += 1;
    }
    table
}

// Build all 26 Caesar tables in a single static
const fn build_all_tables() -> [[u8; 256]; 26] {
    let mut tables = [[0u8; 256]; 26];
    let mut shift = 0;
    while shift < 26 {
        tables[shift] = build_table(shift as i8);
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
#[pyo3(signature = (input, key, skip_non_alpha=true))]
pub fn encrypt(input: &str, key: &str, skip_non_alpha: bool) -> PyResult<String> {
    let result: String = vigenere_rotate(input, key, Mode::Encrypt, skip_non_alpha)?;
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (input, key, skip_non_alpha=true))]
pub fn decrypt(input: &str, key: &str, skip_non_alpha: bool) -> PyResult<String> {
    let result: String = vigenere_rotate(input, key, Mode::Decrypt, skip_non_alpha)?;
    Ok(result)
}

// Main Vigenere function
fn vigenere_rotate(input: &str, key: &str, mode: Mode, skip_non_alpha: bool) -> PyResult<String> {
    // Validate key
    if !key.chars().all(|c| c.is_ascii_alphabetic()) || key.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err(
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
    let expanded_key_shifts: Vec<u8> = expand_key_shifts(&key_shifts, input.len());
    // Main encryption/decryption logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    let mut key_index: usize = 0;
    // Main loop
    for &byte in input.as_bytes() {
        let shift: u8 = unsafe { *expanded_key_shifts.get_unchecked(key_index) };
        let rotated_char: u8 = single_rotate(byte, shift);
        result.push(rotated_char);
        key_index += if !skip_non_alpha || byte != rotated_char {
            1
        } else {
            0
        };
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(result) };
    Ok(result_string)
}

// Expand key shifts to match input length
fn expand_key_shifts(key: &[u8], input_len: usize) -> Vec<u8> {
    let mut expanded_key: Vec<u8> = Vec::with_capacity(input_len);
    let key_len: usize = key.len();
    while expanded_key.len() + key_len <= input_len {
        expanded_key.extend_from_slice(key);
    }
    expanded_key.extend_from_slice(&key[..(input_len - expanded_key.len())]);
    expanded_key
}

// Implementation of single rotate on a single character
// As the end user will not be able to use this, we can assume no errors
#[inline(always)]
fn single_rotate(input: u8, shift: u8) -> u8 {
    // No need for shift validation as no one outside this module can call this function
    unsafe {
        *CAESAR_TABLES
            .get_unchecked(shift as usize)
            .get_unchecked(input as usize)
    }
}
