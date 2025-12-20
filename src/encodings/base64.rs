use pyo3::prelude::*;

// Standard Base64 alphabet
const STANDARD_BASE_64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

static DECODE_MAP: [u8; 256] = {
    let mut table: [u8; 256] = [0xff; 256];
    let mut i: u8 = 0;
    while i < 26 {
        table[(b'A' + i) as usize] = i;
        i += 1;
    }
    i = 0;
    while i < 26 {
        table[(b'a' + i) as usize] = 26 + i;
        i += 1;
    }
    i = 0;
    while i < 10 {
        table[(b'0' + i) as usize] = 52 + i;
        i += 1;
    }

    table[b'+' as usize] = 62;
    table[b'/' as usize] = 63;

    table
};

// Exposed python functions
#[pyfunction]
pub fn encode(data: &str) -> PyResult<String> {
    Ok(String::from_utf8(encode_bytes_rust(data.as_bytes()))?)
}

#[pyfunction]
pub fn decode(data: &str) -> PyResult<String> {
    match String::from_utf8(decode_bytes_rust(
        data.trim().trim_end_matches('=').as_bytes(),
    )?) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err(pyo3::exceptions::PyValueError::new_err(
            "Invalid utf8. Use decode_bytes() instead.",
        )),
    }
}

#[pyfunction]
pub fn encode_bytes(data: &[u8]) -> PyResult<Vec<u8>> {
    Ok(encode_bytes_rust(data))
}

#[pyfunction]
pub fn decode_bytes(data: &[u8]) -> PyResult<Vec<u8>> {
    decode_bytes_rust(data)
}

// Bytes encoding
pub fn encode_bytes_rust(bytes: &[u8]) -> Vec<u8> {
    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len().div_ceil(3) * 4);
    // Unroll input
    let mut i = 0;
    while i + 3 <= bytes.len() {
        // Read 3 bytes as 24 bits
        let chunk =
            ((bytes[i] as u32) << 16) | ((bytes[i + 1] as u32) << 8) | (bytes[i + 2] as u32);
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
        out.push(STANDARD_BASE_64_ALPHABET[((triple >> 6) & 0x3F) as usize]);
        // Padding
        out.push(b'=');
    }
    // Return bytes
    out
}

// Bytes decoding
pub fn decode_bytes_rust(data: &[u8]) -> PyResult<Vec<u8>> {
    // Prepare output vector
    let mut out: Vec<u8> = Vec::with_capacity((data.len() * 6).div_ceil(8));
    // Buffer
    let mut buffer: u32 = 0;
    let mut bits_left: u8 = 0;
    // Decode each byte
    for &b in data {
        // Get value from alphabet
        let val = DECODE_MAP[b as usize];
        if val == 0xff {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid base64 character: '{}'",
                b as char
            )));
        }
        // Update buffer
        buffer = (buffer << 6) | val as u32;
        bits_left += 6;
        // While we have at least 8 bits, extract byte
        while bits_left >= 8 {
            bits_left -= 8;
            let byte = ((buffer >> bits_left) & 0xFF) as u8;
            out.push(byte);
        }
    }
    Ok(out)
}
