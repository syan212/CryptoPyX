use pyo3::prelude::*;

mod ciphers{
    pub mod caesar;
}

#[pymodule]
fn cryptopyx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_ciphers(m)?;
    Ok(())
}

fn register_ciphers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let ciphers_module = PyModule::new(m.py(), "ciphers")?;
    register_caesar_submodule(&ciphers_module)?;
    m.add_submodule(&ciphers_module)?;
    Ok(())
}

fn register_caesar_submodule(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let caesar_module = PyModule::new(m.py(), "caesar")?;
    caesar_module.add_function(wrap_pyfunction!(ciphers::caesar::test, &caesar_module)?)?;
    // Test call to verify everything is working
    ciphers::caesar::test("Hello from Rust!".to_string());
    m.add_submodule(&caesar_module)?;
    Ok(())
}