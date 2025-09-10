use pyo3::prelude::*;

fn i8(x: u8) -> i8 {
    x as i8
}

#[pyfunction]
pub fn encrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = decrypt_or_encrypt(input, shift, "encrypt")?;
    Ok(result)
}

#[pyfunction]
pub fn decrypt(input: &str, shift: i32) -> PyResult<String> {
    let result: String = decrypt_or_encrypt(input, shift, "decrypt")?;
    Ok(result)
}

fn decrypt_or_encrypt(input: &str, shift: i32, mode: &str) -> PyResult<String> {
    // Validate shift range
    if (-25..=25).contains(&shift) == false {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Shift must be in the range -25 to 25",
        ));
    }
    // ! Note: I used i8 here because of integer undeflow/overflow issues
    // Convert shift to u8 and handle wrap-around
    let shift: i8 = (shift.rem_euclid(26)) as i8;
    // Main encryption/decryption logic
    let mut result: Vec<u8> = Vec::with_capacity(input.len());
    for &byte in input.as_bytes() {
        let byte_num: i8 = i8(byte);
        let base: i8 = if byte.is_ascii_lowercase() {
            i8(b'a')
        } else if byte.is_ascii_uppercase() {
            i8(b'A')
        } else {
            let shifted_byte: u8 = byte as u8;
            result.push(shifted_byte);
            continue;
        };
        let shifted_byte: u8 = if mode == "encrypt" {
            println!("Encrypting");
            ((((byte_num - base + shift)).rem_euclid(26)) + base) as u8
        } else if mode == "decrypt" {
            println!("Decrypting");
            ((((byte_num - base - shift)).rem_euclid(26)) + base) as u8
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Mode must be either 'encrypt' or 'decrypt'",
            ));
        };
        result.push(shifted_byte);
    }
    let result_string: String = String::from_utf8(result)?;
    Ok(result_string)
}