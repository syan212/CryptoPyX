use pyo3::exceptions::*;
use pyo3::prelude::*;

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
    // Decode and return string (UTF-8 assumed)
    Ok(String::from_utf8(encode_bytes_rust(data.as_bytes()))?)
}

// Exposed python decode function for strings
#[pyfunction]
#[pyo3(signature = (data, strict=false))]
pub fn decode(data: &str, strict: bool) -> PyResult<String> {
    // If strict, check for length
    if data.len() % 8 != 0 && strict {
        return Err(PyErr::new::<PyValueError, _>(
            "Length of input is not valid. Use strict=False to disable length checking.",
        ));
    }
    // Trim any whitespace and padding, decode and check for valid UTF-8
    match String::from_utf8(decode_bytes_rust(
        data.trim().trim_end_matches('=').as_bytes(),
    )?) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err(PyErr::new::<PyValueError, _>(
            "Invalid utf8. Use decode_bytes() instead.",
        )),
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
pub fn encode_bytes_rust(bytes: &[u8]) -> Vec<u8> {
    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() * 8).div_ceil(5));
    // Unroll input into 40 bits chunk
    let mut i = 0;
    while i + 5 <= bytes.len() {
        // Read 5 bytes as 40 bits
        let chunk = ((bytes[i] as u64) << 32)
            | ((bytes[i + 1] as u64) << 24)
            | ((bytes[i + 2] as u64) << 16)
            | ((bytes[i + 3] as u64) << 8)
            | (bytes[i + 4] as u64);
        // Extract 8 groups of 5 bits
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 35) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 30) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 25) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 20) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 15) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 10) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[((chunk >> 5) & 0x1F) as usize]);
        out.push(STANDARD_BASE_32_ALPHABET[(chunk & 0x1F) as usize]);
        i += 5;
    }
    // Handle remainder (less than 5 bytes left)
    let rem = &bytes[i..];
    if !rem.is_empty() {
        // Buffer
        let mut buffer = 0u32;
        let mut bits_left: u8 = 0;
        // Loop, change buffer, push to vector
        for &b in rem {
            buffer = (buffer << 8) | b as u32;
            bits_left += 8;
            while bits_left >= 5 {
                bits_left -= 5;
                let index = ((buffer >> bits_left) & 0x1F) as usize;
                out.push(STANDARD_BASE_32_ALPHABET[index]);
            }
        }
        // If leftover bits remain, emit one more 5-bit group (padded on the right)
        if bits_left > 0 {
            let index = ((buffer << (5 - bits_left)) & 0x1F) as usize;
            out.push(STANDARD_BASE_32_ALPHABET[index]);
        }
        // Compute padding (8 chars per full 5-byte group)
        let pad_chars: u8 = match rem.len() {
            1 => 6,
            2 => 4,
            3 => 3,
            4 => 1,
            _ => 0,
        };
        // Padding
        if pad_chars > 0 {
            out.extend(std::iter::repeat_n(b'=', pad_chars as usize));
        }
    }
    // Return bytes
    out
}

// Bytes decoding fully rust
pub fn decode_bytes_rust(bytes: &[u8]) -> PyResult<Vec<u8>> {
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
        let val: u8 = DECODE_MAP[b as usize];
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
