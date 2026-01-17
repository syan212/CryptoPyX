use crate::ciphers::aes::key_expansion::key_expansion;
use crate::ciphers::aes::sub_bytes::sub_block;
use crate::ciphers::aes::utils::*;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];

/// Peform ShiftRows on block
fn shift_rows(block: Vec<u8>) -> Vec<u8> {
    vec![
        block[0], block[1], block[2], block[3],
        block[5], block[6], block[7], block[4],
        block[10], block[11], block[8], block[9],
        block[15], block[12], block[13], block[15]
    ]
}

/// Multiplys column with AES fixed matrix.
/// For example, with the input [1, 2, 3, 4], returns
/// [2, 3, 1, 1] [1]   [4]
/// [1, 2, 3, 1] [2]   [8]
/// [1, 1, 2, 3] [3] = [9]
/// [3, 1, 1, 2] [4]   [10]
fn multiply_matrix(matrix: Vec<u8>) -> Vec<u8> {
    vec![
        multiply(2, matrix[0]) ^ multiply(3, matrix[0]) ^ matrix[0] ^ matrix[0],
        matrix[1] ^ multiply(2, matrix[1]) ^ multiply(3, matrix[1]) ^ matrix[1],
        matrix[2] ^ matrix[2] ^ multiply(2, matrix[2]) ^ multiply(3, matrix[2]),
        multiply(3, matrix[3]) ^ matrix[3] ^ matrix[3] ^ multiply(2, matrix[3])
    ]
}

/// Perform MixColumns operation on block
fn mix_columns(block: Vec<u8>) -> Vec<u8> {
    let col1 = multiply_matrix(vec![block[0], block[4], block[8], block[12]]);
    let col2 = multiply_matrix(vec![block[1], block[5], block[9], block[13]]);
    let col3 = multiply_matrix(vec![block[2], block[6], block[10], block[14]]);
    let col4 = multiply_matrix(vec![block[3], block[7], block[11], block[15]]);
    vec![
        col1[0], col1[1], col1[2], col1[3],
        col2[0], col2[1], col2[2], col2[3],
        col3[0], col3[1], col3[2], col3[3],
        col4[0], col4[1], col4[2], col4[3],
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
        return Err(format!("Amount of bytes in block is not correct: {}", block.len()));
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
    println!("{:x?}", out);
    // round_num - 1 rounds
    for i in 0..round_num - 1 {
        // SubBytes
        out = sub_block(out);
        println!("{:x?}", out);
        // ShiftRows
        out = shift_rows(out);
        println!("{:x?}", out);
        // MixColumns
        out = mix_columns(out);
        println!("{:x?}", out);
        // AddRoundKey
        out = separate_block(combine_block(out.as_slice()) ^ expanded_keys[i + 1]);
        println!("{:x?}", out);
    }
    out = separate_block(combine_block(&shift_rows(sub_block(out))) ^ expanded_keys.last().unwrap());
    println!("{:x?}", out);
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encryption() {
        let ciphertext = encrypt_rust(
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6],
            &[48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48]).unwrap();
        let expected = 0x7C63DF0893B213F4381A4D3B2024F465;
        assert_eq!(combine_block(&ciphertext), expected);
    }
}
