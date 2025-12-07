#![cfg_attr(rustfmt, rustfmt::skip)]

// For shorter functions name
use ciphers::{caesar, rot13, vigenere};
use encodings::{base32 as b32, base64 as b64};
// pyo3
use pyo3::prelude::*;
use pyo3::types::PyModule;

// CLI
mod cli;

// Ciphers submodules
pub mod ciphers {
    pub mod caesar;
    pub mod rot13;
    pub mod vigenere;
}

// Encodings submodules
pub mod encodings {
    pub mod base32;
    pub mod base64;
}

// Macro to register submodules
macro_rules! reg_submodule {
    ($parent:expr, $name:expr, [$( $func:path ),* $(,)?]) => {{
        let py = $parent.py();
        let submodule = PyModule::new(py, $name)?;
        $(
            submodule.add_function(wrap_pyfunction!($func, &submodule)?)?;
        )*
        $parent.add_submodule(&submodule)?;
        Ok::<(), PyErr>(())
    }};
}

// Main module
#[pymodule]
fn _cryptopyx<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    register_ciphers(m)?;
    register_encodings(m)?;
    reg_submodule!(m, "cli", [cli::parse])?;
    Ok(())
}

// Register ciphers submodule
fn register_ciphers<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    // Submodule
    let ciphers_module: Bound<'_, PyModule> = PyModule::new(m.py(), "ciphers")?;
    // Register submodules under ciphers
    reg_submodule!(ciphers_module, "caesar", [caesar::encrypt, caesar::decrypt])?;
    reg_submodule!(ciphers_module, "vigenere", [vigenere::encrypt, vigenere::decrypt])?;
    reg_submodule!(ciphers_module, "rot13", [rot13::encrypt, rot13::decrypt, rot13::rotate])?;
    // Add ciphers submodule to parent module
    m.add_submodule(&ciphers_module)?;
    Ok(())
}

// Register encodings submodule
fn register_encodings<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    // Submodule
    let encodings_module: Bound<'_, PyModule> = PyModule::new(m.py(), "encodings")?;
    // Register submodules under encodings
    reg_submodule!(
        encodings_module,
        "base32",
        [b32::encode, b32::decode, b32::encode_bytes, b32::decode_bytes]
    )?;
    reg_submodule!(
        encodings_module,
        "base64",
        [b64::encode_bytes]
    )?;
    // Add encodings submodule to parent module
    m.add_submodule(&encodings_module)?;
    Ok(())
}
