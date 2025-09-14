use pyo3::prelude::*;
use pyo3::types::PyModule;

// List of all submodules
mod ciphers{
    // ciphers submodules
    pub mod caesar;
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
    // Fix sys.modules problem manually
    fix_sys(m, "cryptopyx.ciphers", &ciphers_module)?;
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
    // Fix sys.modules problem manually
    fix_sys(m, "cryptopyx.ciphers.caesar", &caesar_module)?;
    Ok(())
}

// Register rot13 submodule under ciphers
fn register_rot13_submodule<'py>(m: &Bound<'py, PyModule>) -> PyResult<()> {
    let rot13_module: Bound<'_, PyModule> = PyModule::new(m.py(), "rot13")?;
    // Add functions to rot13 submodule
    rot13_module.add_function(wrap_pyfunction!(ciphers::rot13::encrypt, &rot13_module)?)?;
    rot13_module.add_function(wrap_pyfunction!(ciphers::rot13::decrypt, &rot13_module)?)?;
    // Add rot13 submodule to parent module
    m.add_submodule(&rot13_module)?;
    // Fix sys.modules problem manually
    fix_sys(m, "cryptopyx.ciphers.rot13", &rot13_module)?;
    Ok(())
}

// Helper function to fix sys.modules problem
fn fix_sys<'py>(m: &Bound<'py, PyModule>, module_name: &str, module: &Bound<'py, PyModule>) -> PyResult<()> {
    let sys = PyModule::import(m.py(), "sys")?;
    let sys_modules = sys.getattr("modules")?;
    sys_modules.set_item(module_name, module)?;
    Ok(())
}