use pyo3::prelude::*;

#[pyfunction]
pub fn test(input: String) {
    println!("Test: {}", input);
}