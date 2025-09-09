use pyo3::prelude::*;

// List of all submodules
mod ciphers{
    // ciphers submodules
    pub mod caesar;
    pub mod vigenere;
    pub mod rot13;
}

// Main module
#[pymodule]
fn cryptopyx<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    register_ciphers(m)?;
    Ok(())
}

// Register ciphers submodule
fn register_ciphers<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    let ciphers_module: Bound<'_, PyModule> = PyModule::new(m.py(), "ciphers")?;
    // Register submodules under ciphers
    register_caesar_submodule(&ciphers_module)?;
    register_vigenere_submodule(&ciphers_module)?;
    register_rot13_submodule(&ciphers_module)?;
    // Add ciphers submodule to parent module
    m.add_submodule(&ciphers_module)?;
    Ok(())
}

// Register caesar submodule under ciphers
fn register_caesar_submodule<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    let caesar_module: Bound<'_, PyModule> = PyModule::new(m.py(), "caesar")?;
    // Add functions to caesar submodule
    caesar_module.add_function(wrap_pyfunction!(ciphers::caesar::encrypt, &caesar_module)?)?;
    caesar_module.add_function(wrap_pyfunction!(ciphers::caesar::decrypt, &caesar_module)?)?;
    // Add caesar submodule to parent module
    m.add_submodule(&caesar_module)?;
    Ok(())
}

// Register vigenere submodule under ciphers
fn register_vigenere_submodule<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    let vigenere_module: Bound<'_, PyModule> = PyModule::new(m.py(), "vigenere")?;
    // TODO: Add functions to vigenere submodule
    m.add_submodule(&vigenere_module)?;
    Ok(())
}

// Register rot13 submodule under ciphers
fn register_rot13_submodule<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    let rot13_module: Bound<'_, PyModule> = PyModule::new(m.py(), "rot13")?;
    // TODO: Add functions to vigenere submodule
    m.add_submodule(&rot13_module)?;
    Ok(())
}