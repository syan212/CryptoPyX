use crate::ciphers::aes::key_expansion::key_expansion;
use crate::ciphers::aes::sub_bytes::sub_block;
use crate::ciphers::aes::mix_columns::mix_columns;
use crate::ciphers::aes::shift_rows::shift_rows;
use crate::ciphers::aes::utils::*;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];

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
        out = separate_block(
            combine_block(
                &mix_columns( // MixColumns (3rd)
                    shift_rows( // ShiftRows (2nd)
                        sub_block(out) // SubBytes (1st)
                    )
                )
            ) ^ expanded_keys[i + 1] // AddRoundKey (4th)
        );
    }
    // Final round without MixColumns
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
