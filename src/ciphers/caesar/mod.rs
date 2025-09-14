use pyo3::prelude::*;

#[derive(Copy, Clone)]
enum Mode {
    Encrypt,
    Decrypt,
}

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
    // ! Note: I used i8 here because of integer undeflow/overflow issues
    // Convert shift to u8 and handle wrap-around
    let shift: i8 = shift.rem_euclid(26) as i8;
    if shift == 0 as i8 {
        return Ok(String::from(input))
    }
    // Calculate direction
    let dir: i8 = match mode {
        Mode::Encrypt => 1,
        Mode::Decrypt => -1,
    };
    // Main encryption/decryption logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        let byte_num: i8 = byte as i8;
        let base: i8 = if (b'a'..=b'z').contains(&byte)  {
            b'a' as i8
        } else if (b'A'..=b'Z').contains(&byte) {
            b'A' as i8
        } else {
            let shifted_byte: u8 = byte as u8;
            result.push(shifted_byte);
            continue;
        };
        let shifted_byte: u8 = ((byte_num - base + shift * dir).rem_euclid(26) + base) as u8;
        result.push(shifted_byte);
    }
    let result_string: String = unsafe { String::from_utf8_unchecked(result) };
    Ok(result_string)
}