use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyResult;

const PKCS_ALIASES: [&str; 4] = ["pkcs", "PKCS", "pkcs#7", "PKCS#7"];
const ISO_ALIASES: [&str; 4] = ["ISO", "iso", "ISO/IEC", "iso/iec"];

/// Pad the text according to the padding mode
pub fn pad(text: Vec<u8>, padding_mode: &str) -> PyResult<Vec<u8>> {
    let mut out = text;
    if PKCS_ALIASES.contains(&padding_mode) {
        let padding_amount = if out.len() % 16 != 0 {
            16u8
        } else {
            (out.len() % 16) as u8
        };
        out.extend(vec![padding_amount; padding_amount.into()]);
    } else if ISO_ALIASES.contains(&padding_mode) {
        out.push(0x80);
        while out.len() % 16 != 0 {
            out.push(0);
        }
    } else {
        return Err(PyValueError::new_err(format!(
            "Not a valid padding mode: {}. Acceptable padding mode are: iso, pkcs.",
            padding_mode
        )));
    }
    Ok(out)
}

/// Unpad the text according to the padding mode
/// Assumes text is a multiple of 16
pub fn unpad(text: Vec<u8>, padding_mode: &str) -> PyResult<Vec<u8>> {
    let mut out = text;
    if out.len() % 16 != 0 || out.len() == 0 {
        return Err(PyValueError::new_err(format!(
            "Invalid ciphertext length: {}",
            out.len()
        )));
    }
    if PKCS_ALIASES.contains(&padding_mode) {
        let amount = out.last().unwrap();
        for _ in 0..(*amount as usize) {
            out.pop();
        }
    } else if ISO_ALIASES.contains(&padding_mode) {
        out.pop();
        let last = match out.last() {
            Some(num) => *num,
            None => return Err(PyValueError::new_err(format!(
                "Unexpected end when unpadding text",
            )))
        };
        while *(out.last().unwrap()) != 0x80 {
            out.pop();
        }
    } else {
        return Err(PyValueError::new_err(format!(
            "Not a valid padding mode: {}. Acceptable padding mode are: iso, pkcs.",
            padding_mode
        )));

    }
    Ok(out)
}
