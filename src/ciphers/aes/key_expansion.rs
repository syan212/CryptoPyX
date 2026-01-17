use crate::ciphers::aes::sub_bytes::sub_word;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];
const ROUND_CONSTANTS: [usize; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// One byte left circular shift for KeyExpansion
fn rot_word(word: u32) -> u32 {
    word.rotate_left(8)
}

/// Combine four bytes into a single number
fn combine_bytes(bytes: &[u8]) -> u32 {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}

/// Combine four 32-bit words into a u128 block
fn combine_words(words: &[u32]) -> u128 {
    (words[0] as u128) << 96
        | (words[1] as u128) << 64
        | (words[2] as u128) << 32
        | words[3] as u128
}

/// Expands key using AES key schedule
/// For a 16 byte (128 bit) key returns 176 bytes (11 keys),
/// 312 bytes for 24 bytes (192 bit) and 480 bytes for 32 bytes (256 bit).
/// Returns a Vec<u128>
fn key_expansion(key: &[u8]) -> Result<Vec<u128>, String> {
    if !KEY_LENGTHS.contains(&key.len()) {
        return Err(format!("Invalid key length: {}", key.len()));
    }
    // Allocate vector
    let mut pre_out = Vec::new();
    // Calculate number of keys and 32-bit output words
    let num_keys = (key.len() - 16) * 2 + 11;
    // Original key length in 32-bit words
    let key_length = key.len() / 4;
    // Number of rounds
    let nr = match key_length {
        4 => 10,
        6 => 12,
        8 => 14,
        _ => unreachable!(),
    };
    // Number of 32-bit words in final output
    let num_words = 4 * (nr + 1);
    // Loop
    for i in 0..num_words {
        // Calculate word
        if i < key_length {
            // Return original key part
            pre_out.push(combine_bytes(&key[(i * 4)..i * 4 + 4]));
        } else if i % key_length == 0 {
            let word = pre_out[i - key_length]
                ^ sub_word(rot_word(pre_out[i - 1]))
                ^ (ROUND_CONSTANTS[i / key_length - 1] as u32) << 24;
            pre_out.push(word);
        } else if key_length > 6 && i % key_length == 4 {
            pre_out.push(pre_out[i - key_length] ^ sub_word(pre_out[i - 1]));
        } else {
            pre_out.push(pre_out[i - key_length] ^ pre_out[i - 1]);
        }
    }
    // Combine all words into 128-bit blocks
    let mut out = Vec::new();
    for i in 0..(pre_out.len() / 4) {
        out.push(combine_words(&pre_out[i * 4..(i + 1) * 4]))
    }
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expansion() {
        let key = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected: Vec<u128> = vec![
            0x00000000000000000000000000000000,
            0x62636363626363636263636362636363,
            0x9b9898c9f9fbfbaa9b9898c9f9fbfbaa,
            0x90973450696ccffaf2f457330b0fac99,
            0xee06da7b876a1581759e42b27e91ee2b,
            0x7f2e2b88f8443e098dda7cbbf34b9290,
            0xec614b851425758c99ff09376ab49ba7,
            0x217517873550620bacaf6b3cc61bf09b,
            0x0ef903333ba9613897060a04511dfa9f,
            0xb1d4d8e28a7db9da1d7bb3de4c664941,
            0xb4ef5bcb3e92e21123e951cf6f8f188e,
        ];
        let keys = key_expansion(&key).unwrap();
        assert_eq!(keys, expected);
    }
}
