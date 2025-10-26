use pyo3::prelude::*;
use pyo3::types::PyModule;

// List of all submodules
// Ciphers
mod ciphers {
    // Ciphers submodules
    pub mod caesar;
    pub mod rot13;
    pub mod vigenere;
}

// Encodings
mod encodings {
    // Encodings submodules
    pub mod base32;
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
    Ok(())
}

// Register ciphers submodule
fn register_ciphers<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    // For shorter functions name
    use ciphers::{caesar, rot13, vigenere};
    // Submodule
    let ciphers_module: Bound<'_, PyModule> = PyModule::new(m.py(), "ciphers")?;
    // Register submodules under ciphers
    reg_submodule!(ciphers_module, "caesar", [caesar::encrypt, caesar::decrypt])?;
    reg_submodule!(ciphers_module, "vigenere", [vigenere::encrypt, vigenere::decrypt])?;
    reg_submodule!(ciphers_module, "rot13", [rot13::encrypt, rot13::decrypt])?;
    // Add ciphers submodule to parent module
    m.add_submodule(&ciphers_module)?;
    Ok(())
}

// Register encodings submodule
fn register_encodings<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    // For shorter functions name
    use encodings::base32 as b32;
    // Submodule
    let encodings_module: Bound<'_, PyModule> = PyModule::new(m.py(), "encodings")?;
    // Register submodules under ciphers
    reg_submodule!(
        encodings_module,
        "base32",
        [b32::encode, b32::decode, b32::encode_bytes, b32::decode_bytes]
    )?;
    // Add ciphers submodule to parent module
    m.add_submodule(&encodings_module)?;
    Ok(())
}
