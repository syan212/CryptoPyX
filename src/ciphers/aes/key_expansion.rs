use crate::ciphers::aes::sub_bytes::sub_word;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];
const ROUND_CONSTANTS: [usize; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// One byte left circular shift for KeyExpansion
fn rot_word(word: u32) -> u32 {
    word << 8 | word >> 24
}

/// Combine four bytes into a single number
fn combine_bytes(bytes: &[u8]) -> u32 {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}

/// Get nth 32-bit word
fn get_word(n: usize, list: &Vec<u8>) -> Vec<u8> {
    list[n * 4..(n + 1) * 4].to_vec()
}

/// Expands key using AES key schedule
/// For a 16 byte (128 bit) key returns 176 bytes (11 keys),
/// 312 bytes for 24 bytes (192 bit) and 480 bytes for 32 bytes (256 bit).
/// Returns a Vec<u32>
fn key_expandsion(key: &[u8]) -> Result<Vec<u32>, String> {
    if !KEY_LENGTHS.contains(&key.len()) {
        return Err(format!("Invalid key length: {}", key.len()));
    }
    // Allocate vector
    let mut out = Vec::new();
    // Calculate number of keys and 32-bit output words
    let num_keys = (key.len() - 16) * 2 + 11;
    let num_words = num_keys * 4;
    // Original key length in 32-bit words
    let key_length = key.len() / 4;
    // Loop
    for i in 0..num_words {
        // Calculate word
        if i < key_length {
            // Return original key part
            out.push(combine_bytes(&key[(i * 4)..i * 4 + 4]));
        } else if i % key_length == 0 {
            let word =
                out[i - key_length] ^
                sub_word(rot_word(out[i - 1])) ^
                (ROUND_CONSTANTS[i / key_length] as u32) << 24;
            out.push(word);
        } else if key_length > 6 && i % key_length == 4 {
            out.push(out[i - key_length] ^ sub_word(out[i - 1]));
        } else {
            out.push(out[i - key_length] ^ out[i - 1]);
        }
    }
    Ok(out)
}
