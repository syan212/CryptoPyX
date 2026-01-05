// Substitution cipher with an arbitrary mapping between characters.
use std::collections::HashMap;
use pyo3::prelude::*;

// The exposed python function
#[pyfunction]
pub fn substitute(data: &str, mapping: HashMap<char, char>) -> PyResult<String> {
    Ok(substitution_rust(data, &mapping))
}

// Substitution function
pub fn substitution_rust(data: &str, mapping: &HashMap<char, char>) -> String {
    let mut out = String::with_capacity(data.len());
    for ch in data.chars() {
        if let Some(&substituted) = mapping.get(&ch) {
            out.push(substituted);
        } else {
            out.push(ch);
        }
    }
    out
}
