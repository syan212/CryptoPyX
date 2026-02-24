use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub mod decryption;
pub mod ecb;
pub mod encryption;
pub mod key_expansion;
pub mod mix_columns;
pub mod padding;
pub mod shift_rows;
pub mod sub_bytes;
pub mod utils;

#[pyfunction]
pub fn encrypt_ecb(text: Vec<u8>, key: &[u8], padding_mode: &str) -> PyResult<Vec<u8>> {
    let text = padding::pad(text, padding_mode)?;
    match ecb::encrypt_ecb_rust(&text, key) {
        Ok(ciphertext) => Ok(ciphertext),
        Err(e) => Err(PyValueError::new_err(e)),
    }
}

#[pyfunction]
pub fn decrypt_ecb(text: Vec<u8>, key: &[u8], padding_mode: &str) -> PyResult<Vec<u8>> {
    let unpadded = match ecb::decrypt_ecb_rust(&text, key) {
        Ok(ciphertext) => ciphertext,
        Err(e) => return Err(PyValueError::new_err(e)),
    };
    Ok(padding::unpad(unpadded, padding_mode)?)
}
