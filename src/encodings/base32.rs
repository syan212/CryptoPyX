use pyo3::prelude::*;

// Standard base 32 alphabet (RFC 3548, RFC 4648)
const STANDARD_BASE_32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

// Precomputed decode map (256 entries)
static DECODE_MAP: [u8; 256] = {
    const INVALID: u8 = 0xFF;
    let mut table = [INVALID; 256];
    let mut i = 0;
    while i < 32 {
        table[STANDARD_BASE_32_ALPHABET[i] as usize] = i as u8;
        i += 1;
    }
    table
};

#[pyfunction]
pub fn encode(data: &str) -> PyResult<String> {
    // Get bytes
    let bytes = data.as_bytes();
    if bytes.is_empty() {
        return Ok(String::new());
    }

    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() * 8).div_ceil(5));

    // Use buffer and bits left to track bits and convert
    let mut buffer: u64 = 0;
    let mut bits_left: u8 = 0;

    for &b in bytes {
        buffer = (buffer << 8) | b as u64;
        bits_left += 8;
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

    unsafe { Ok(String::from_utf8_unchecked(out)) }
}

#[pyfunction]
pub fn decode(data: &str) -> PyResult<String> {
    // Trim any whitespace and remove padding
    let input = data.trim().trim_end_matches('=');
    if input.is_empty() {
        return Ok(String::new());
    }

    // Convert to bytes
    let bytes = input.as_bytes();
    let mut buffer: u64 = 0;
    let mut bits_left: u8 = 0;
    let mut out = Vec::with_capacity((bytes.len() * 5) / 8);

    for &b in bytes {
        // Get value or throw error
        let val: u8 = unsafe { *DECODE_MAP.get_unchecked(b as usize) };
        if val == 0xFF {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid Base32 character: '{}'", b as char)
            ));
        }

        buffer = (buffer << 5) | val as u64;
        bits_left += 5;

        // Extract bytes when we have 8 or more bits
        while bits_left >= 8 {
            bits_left -= 8;
            out.push(((buffer >> bits_left) & 0xFF) as u8);
        }
    }

    // Return decoded string (UTF-8 assumed)
    unsafe { Ok(String::from_utf8_unchecked(out)) }
}
