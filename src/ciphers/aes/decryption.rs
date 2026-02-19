use crate::ciphers::aes::key_expansion::key_expansion;
use crate::ciphers::aes::sub_bytes::inv_sub_bytes;
use crate::ciphers::aes::mix_columns::inv_mix_columns;
use crate::ciphers::aes::shift_rows::inv_shift_rows;
use crate::ciphers::aes::utils::*;

/// Allowed AES key lengths in bytes
const KEY_LENGTHS: [usize; 3] = [16, 24, 32];

/// Decrypt block using provided key
/// Performs KeyExpansion, SubBytes, ShiftRows, MixColumns and AddRoundKey,
/// but no mode specific operations. Assumes block is 128-bit.
pub fn single_decrypt(block: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
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
    // AddRoundKey
    let mut out = separate_block(combine_block(block) ^ expanded_keys[round_num]);
    // round_num - 1 rounds
    for i in 0..round_num - 1 {
        out = inv_mix_columns(
            separate_block(expanded_keys[round_num - 1 - i] ^ combine_block(
                    &inv_sub_bytes(
                        inv_shift_rows(out)
                    )
                )
            )
        );
    }
    // Final round
    out = separate_block(expanded_keys[0] ^ combine_block(
            &inv_sub_bytes(
                inv_shift_rows(out)
            )
        )
    );
    Ok(out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decryption() {
        let ciphertext = single_decrypt(
            &separate_block(0x6d251e6944b051e04eaa6fb4dbf78465),
            &separate_block(0x10a58869d74be5a374cf867cfb473859)
        )
        .unwrap();
        let expected = 0x00000000000000000000000000000000;
        assert_eq!(combine_block(&ciphertext), expected);
    }
}
