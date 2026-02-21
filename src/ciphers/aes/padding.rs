use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyResult;

const PKCS_ALIASES: [&str; 4] = ["pkcs", "PKCS", "pkcs#7", "PKCS#7"];
const ISO_ALIASES: [&str; 4] = ["ISO", "iso", "ISO/IEC", "iso/iec"];

pub fn pad(text: Vec<u8>, padding_mode: &str) -> PyResult<Vec<u8>> {
    let padding = (text.len() % 16) as u8;
    let mut out = text;
    if PKCS_ALIASES.contains(&padding_mode) {
        out.extend(vec![padding; padding.into()]);
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
    println!("{:x?}", out);
    Ok(out)
}
