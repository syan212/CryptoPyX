use pyo3::exceptions::*;
use pyo3::prelude::*;
use std::str;

// Standard base 32 alphabet (RFC 3548, RFC 4648)
const STANDARD_BASE_32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

// Precomputed decode map (256 entries)
static DECODE_MAP: [u8; 256] = {
    const INVALID: u8 = 0xFF;
    let mut table = [INVALID; 256];
    let mut i = 0;
    // Uppercase A–Z
    while i < 26 {
        let ch = b'A' + i as u8;
        table[ch as usize] = i as u8;
        // Lowercase a–z
        table[(ch + 32) as usize] = i as u8; // 'a' = 'A' + 32 in ASCII
        i += 1;
    }
    // Digits 2–7 (values 26–31)
    let mut j = 0;
    while j < 6 {
        table[(b'2' + j as u8) as usize] = (26 + j) as u8;
        j += 1;
    }
    table
};

// Exposed python encode function for strings
#[pyfunction]
pub fn encode(data: &str) -> PyResult<String> {
    // Get bytes
    let bytes = data.as_bytes();
    // Decode
    let out = encode_bytes_rust(bytes);
    // Return decoded string (UTF-8 assumed)
    unsafe { Ok(String::from_utf8_unchecked(out)) }
}

// Exposed python decode function for strings
#[pyfunction]
pub fn decode(data: &str) -> PyResult<String> {
    // Trim any whitespace and remove padding
    let input = data.trim().trim_end_matches('=');
    // Convert to bytes and decode
    let bytes = input.as_bytes();
    let out = decode_bytes_rust(bytes)?;
    // Check for valid UTF-8
    match str::from_utf8(&out) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err(PyErr::new::<PyValueError, _>("Invalid utf8. Use decode_bytes() instead.")),
    }
}

// Python encode bytes function
#[pyfunction]
pub fn encode_bytes(data: &[u8]) -> PyResult<Vec<u8>> {
    Ok(encode_bytes_rust(data))
}

#[pyfunction]
pub fn decode_bytes(data: &[u8]) -> PyResult<Vec<u8>> {
    decode_bytes_rust(data)
}

// Bytes encoding fully rust
fn encode_bytes_rust(bytes: &[u8]) -> Vec<u8> {
    // Empty vector
    if bytes.is_empty() {
        return Vec::new();
    }
    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() * 8).div_ceil(5));
    // Use buffer and bits left to track bits and convert
    let mut buffer: u64 = 0;
    let mut bits_left: u8 = 0;
    // Loop to encode bytes
    for &b in bytes {
        buffer = (buffer << 8) | b as u64;
        bits_left += 8;
        // Extract bytes when we have 5 or more bits
        while bits_left >= 5 {
            bits_left -= 5;
            unsafe {
                out.push(
                    *STANDARD_BASE_32_ALPHABET
                        .get_unchecked(((buffer >> bits_left) & 0x1F) as usize),
                );
            }
        }
    }
    // Ensure all bits are converted
    if bits_left > 0 {
        unsafe {
            out.push(
                *STANDARD_BASE_32_ALPHABET
                    .get_unchecked(((buffer << (5 - bits_left)) & 0x1F) as usize),
            );
        }
    }
    // Padding
    let pad_len = (8 - (out.len() % 8)) % 8;
    if pad_len != 0 {
        out.extend(std::iter::repeat_n(b'=', pad_len));
    }
    // Return bytes
    out
}

// Bytes decoding fully rust
fn decode_bytes_rust(bytes: &[u8]) -> PyResult<Vec<u8>> {
    // Preallocate out vector
    let mut out = Vec::with_capacity((bytes.len() * 5) / 8);
    // Use buffer to track bits
    let mut buffer: u64 = 0;
    let mut bits_left: u8 = 0;
    // Loop to decode bytes
    for &b in bytes {
        // Check for padding
        if b == b'=' {
            break;
        }
        // Validate character
        let val: u8 = unsafe { *DECODE_MAP.get_unchecked(b as usize) };
        if val == 0xFF {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Invalid Base32 character: '{}'",
                b as char
            )));
        }
        // Update buffer and bit count
        buffer = (buffer << 5) | val as u64;
        bits_left += 5;
        // Extract bytes when we have 8 or more bits
        while bits_left >= 8 {
            bits_left -= 8;
            out.push(((buffer >> bits_left) & 0xFF) as u8);
        }
    }
    // Return bytes
    Ok(out)
}
