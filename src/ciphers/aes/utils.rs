/// Combine four bytes into a 32-bit word
/// Opposite of `separate_word`
pub fn combine_bytes(bytes: &[u8]) -> u32 {
    (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
}

/// Combine four 32-bit words into a u128 block
pub fn combine_words(words: &[u32]) -> u128 {
    (words[0] as u128) << 96
        | (words[1] as u128) << 64
        | (words[2] as u128) << 32
        | words[3] as u128
}

/// Combine 128-bit block into u128
pub fn combine_block(block: &[u8]) -> u128 {
    let mut out = 0u128;
    for byte in block {
        out = out << 8 | *byte as u128;
    }
    out
}

/// Separate a 32-bit word into a Vec<u8>
/// Opposite of `combine_bytes`
pub fn separate_word(word: u32) -> Vec<u8> {
    vec![
        (word >> 24) as u8,
        (word >> 16) as u8,
        (word >> 8) as u8,
        word as u8,
    ]
}

/// Separate u128 into Vec<u8>
pub fn separate_block(block: u128) -> Vec<u8> {
    vec![
        (block >> 120) as u8,
        (block >> 112) as u8,
        (block >> 104) as u8,
        (block >> 96) as u8,
        (block >> 88) as u8,
        (block >> 80) as u8,
        (block >> 72) as u8,
        (block >> 64) as u8,
        (block >> 56) as u8,
        (block >> 48) as u8,
        (block >> 40) as u8,
        (block >> 32) as u8,
        (block >> 24) as u8,
        (block >> 16) as u8,
        (block >> 8) as u8,
        block as u8,
    ]
}

/// Separate block into Vec<u32>
pub fn separate_block_into_u32(block: u128) -> Vec<u32> {
    vec![
        (block >> 96) as u32,
        (block >> 64) as u32,
        (block >> 32) as u32,
        block as u32,
    ]
}
