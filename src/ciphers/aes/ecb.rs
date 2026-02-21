use crate::ciphers::aes::decryption;
use crate::ciphers::aes::encryption;

/// Encrypt text using key, assuming no padding is required. Uses ECB.
pub fn encrypt_ecb_rust(text: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let chunks: Vec<Vec<u8>> = text.to_vec().chunks(16).map(|v| v.to_vec()).collect();
    let mut out = Vec::new();
    for chunk in chunks {
        out.extend(encryption::single_encrypt(&chunk, key)?.into_iter());
    }
    Ok(out)
}

/// Decrypt text using key, Uses ECB.
pub fn decrypt_ecb_rust(text: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let chunks: Vec<Vec<u8>> = text.to_vec().chunks(16).map(|v| v.to_vec()).collect();
    let mut out = Vec::new();
    for chunk in chunks {
        out.extend(decryption::single_decrypt(&chunk, key)?.into_iter());
    }
    Ok(out)
}
