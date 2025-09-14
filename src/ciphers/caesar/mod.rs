use pyo3::prelude::*;

// Build table for specifc shift
const fn build_table(shift: i8) -> [u8; 256]{
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

#[derive(Copy, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

// All Caesar Lookup Tables
static CAESAR_TABLES: [[u8; 256]; 26] = build_all_tables();

#[pyfunction]
pub fn encrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = decrypt_or_encrypt(input, shift, Mode::Encrypt)?;
    Ok(result)
}

#[pyfunction]
pub fn decrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = decrypt_or_encrypt(input, shift, Mode::Decrypt)?;
    Ok(result)
}

fn decrypt_or_encrypt(input: &str, shift: i32, mode: Mode) -> PyResult<String> {
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
        return Ok(String::from(input))
    }
    // Compute forwards shift to find correct table
    let forward_shift = match mode {
        Mode::Encrypt => shift,
        Mode::Decrypt => (26 - shift) % 26,
    };
    let table = &CAESAR_TABLES[forward_shift];
    // Main encryption/decryption logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        result.push(table[byte as usize]);
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(result) };
    Ok(result_string)
}