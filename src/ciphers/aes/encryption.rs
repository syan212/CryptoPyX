use crate::ciphers::aes::key_expansion::key_expansion;
use crate::ciphers::aes::sub_bytes::sub_block;
use crate::ciphers::aes::utils::*;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];

/// Perform ShiftRows on block
fn shift_rows(block: Vec<u8>) -> Vec<u8> {
    vec![
        block[0], block[5], block[10], block[15], block[4], block[9], block[14], block[3],
        block[8], block[13], block[2], block[7], block[12], block[1], block[6], block[11],
    ]
}

/// Multiplies column with AES fixed matrix.
/// For example, with the input [1, 2, 3, 4], returns
fn multiply_matrix(matrix: Vec<u8>) -> Vec<u8> {
    // `a` is the input multiplied by 2 in GF(2^8)
    let mut a = [0, 0, 0, 0];
    // `h` is flag for whether modulo is needed
    let mut h: u8;
    for i in 0..4 {
        h = matrix[i] >> 7;
        a[i] = matrix[i] << 1;
        a[i] ^= h * 0x1b;
    }
    vec![
        a[0] ^ matrix[3] ^ matrix[2] ^ a[1] ^ matrix[1],
        a[1] ^ matrix[0] ^ matrix[3] ^ a[2] ^ matrix[2],
        a[2] ^ matrix[1] ^ matrix[0] ^ a[3] ^ matrix[3],
        a[3] ^ matrix[2] ^ matrix[1] ^ a[0] ^ matrix[0],
    ]
}

/// Perform MixColumns operation on block
fn mix_columns(block: Vec<u8>) -> Vec<u8> {
    let col1 = multiply_matrix(block[0..4].to_vec());
    let col2 = multiply_matrix(block[4..8].to_vec());
    let col3 = multiply_matrix(block[8..12].to_vec());
    let col4 = multiply_matrix(block[12..16].to_vec());
    vec![
        col1[0], col1[1], col1[2], col1[3], col2[0], col2[1], col2[2], col2[3], col3[0], col3[1],
        col3[2], col3[3], col4[0], col4[1], col4[2], col4[3],
    ]
}

/// Encrypt block using provided key
/// Performs KeyExpansion, SubBytes, ShiftRows, MixColumns and AddRoundKey,
/// but no mode specific operations. Assumes block is 128-bit.
pub fn encrypt_rust(block: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if !KEY_LENGTHS.contains(&key.len()) {
        return Err(format!("Invalid key length: {}", key.len()));
    }
    if block.len() != 16 {
        return Err(format!(
            "Amount of bytes in block is not correct: {}",
            block.len()
        ));
    }
    // Number of rounds
    let round_num = match key.len() / 4 {
        4 => 10,
        6 => 12,
        8 => 14,
        _ => unreachable!(),
    };
    // Key expansion
    let expanded_keys = key_expansion(key)?;
    // Initial AddRoundKey
    let mut out = separate_block(combine_block(block) ^ expanded_keys[0]);
    // round_num - 1 rounds
    for i in 0..round_num - 1 {
        // SubBytes
        out = sub_block(out);
        // ShiftRows
        out = shift_rows(out);
        // MixColumns
        out = mix_columns(out);
        // AddRoundKey
        out = separate_block(combine_block(&out) ^ expanded_keys[i + 1]);
    }
    out =
        separate_block(combine_block(&shift_rows(sub_block(out))) ^ expanded_keys.last().unwrap());
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption() {
        let ciphertext = encrypt_rust(
            &[
                0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
                0x17, 0x2a,
            ],
            &[
                0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
                0x4f, 0x3c,
            ],
        )
        .unwrap();
        let expected = 0x3ad77bb40d7a3660a89ecaf32466ef97;
        assert_eq!(combine_block(&ciphertext), expected);
    }
}
