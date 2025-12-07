use pyo3::prelude::*;

// Standard Base64 alphabet
const STANDARD_BASE_64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Exposed python functions
#[pyfunction]
pub fn encode_bytes(data: &[u8]) -> PyResult<Vec<u8>> {
    Ok(encode_bytes_rust(data))
}

// Bytes encoding
pub fn encode_bytes_rust(bytes: &[u8]) -> Vec<u8> {
    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() + 2) / 3 * 4);
    // Unroll input
    let mut i = 0;
    while i + 3 <= bytes.len() {
        // Read 3 bytes as 24 bits
        let chunk = ((bytes[i] as u32) << 16) |
            ((bytes[i + 1] as u32) << 8) |
            (bytes[i + 2] as u32);
        // Extract 4 groups of 6 bits
        out.push(STANDARD_BASE_64_ALPHABET[((chunk >> 18) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[((chunk >> 12) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[((chunk >> 6) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[(chunk & 0x3F) as usize]);
        i += 3;
    }
    // Handle remainder (less than 3 bytes left)
    let rem = bytes.len() - i;
    // 1 byte left
    if rem == 1 {
        // Push remainder bytes
        let chunk = (bytes[i] as u32) << 16;
        out.push(STANDARD_BASE_64_ALPHABET[((chunk >> 18) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[((chunk >> 12) & 0x3F) as usize]);
        // Padding
        out.push(b'=');
        out.push(b'=');
    // 2 bytes left
    } else if rem == 2 {
        let triple = ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8);
        // Push remainder bytes
        out.push(STANDARD_BASE_64_ALPHABET[((triple >> 18) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[((triple >> 12) & 0x3F) as usize]);
        out.push(STANDARD_BASE_64_ALPHABET[((triple >>  6) & 0x3F) as usize]);
        // Padding
        out.push(b'=');
    }
    // Return bytes
    out
}
