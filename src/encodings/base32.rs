use pyo3::prelude::*;

const STANDARD_BASE_32_ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

#[pyfunction]
pub fn encode(data: &str) -> PyResult<String> {
    // Get bytes
    let bytes = data.as_bytes();
    if bytes.is_empty() {
        return Ok(String::new());
    }

    // Preallocate out vector
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() * 8 + 4) / 5);

    // Use buffer and bits left to track bits and convert
    let mut buffer: u32 = 0;
    let mut bits_left: u8 = 0;

    for &b in bytes {
        buffer = (buffer << 8) | b as u32;
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
        out.extend(std::iter::repeat(b'=').take(pad_len));
    }

    unsafe { Ok(String::from_utf8_unchecked(out)) }
}
