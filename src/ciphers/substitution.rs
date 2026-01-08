// Substitution cipher with an arbitrary mapping between characters.
use pyo3::prelude::*;
use std::collections::HashMap;

// The exposed python functions
#[pyfunction]
pub fn substitute(data: &str, mapping: HashMap<char, char>) -> PyResult<String> {
    // Convert char mapping to u8 mapping
    let mapping: HashMap<u8, u8> = mapping
        .into_iter()
        .map(|(k, v)| (k as u8, v as u8))
        .collect();
    let out = substitution_rust(data.as_bytes(), &mapping);
    // Unwrap should be safe here
    Ok(String::from_utf8(out).unwrap())
}

#[pyfunction]
pub fn substitute_reverse(data: &str, mapping: HashMap<char, char>) -> PyResult<String> {
    // Invert the mapping
    let mapping = mapping
        .into_iter()
        .map(|(k, v)| (v as u8, k as u8))
        .collect();
    let out = substitution_rust(data.as_bytes(), &mapping);
    // Unwrap should be safe here
    Ok(String::from_utf8(out).unwrap())
}

#[pyfunction]
pub fn substitute_bytes(data: &[u8], mapping: HashMap<u8, u8>) -> PyResult<Vec<u8>> {
    Ok(substitution_rust(data, &mapping))
}

#[pyfunction]
pub fn substitute_reverse_bytes(data: &[u8], mapping: HashMap<u8, u8>) -> PyResult<Vec<u8>> {
    // Invert the mapping
    let mapping: HashMap<u8, u8> = mapping.into_iter().map(|(k, v)| (v, k)).collect();
    Ok(substitution_rust(data, &mapping))
}

// Substitution function using u8 mapping
pub fn substitution_rust(data: &[u8], mapping: &HashMap<u8, u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    for &byte in data {
        if let Some(&substituted) = mapping.get(&byte) {
            out.push(substituted);
        } else {
            out.push(byte);
        }
    }
    out
}
