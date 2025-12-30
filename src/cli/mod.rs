use ciphers::*;
use clap::ArgMatches;
use clap::error::ErrorKind;
use encodings::*;
use matches::get_matches;
use pyo3::prelude::*;
use std::env::args;
use utils::error;

mod ciphers;
mod encodings;
mod matches;
mod utils;

#[pyfunction]
pub fn parse() -> PyResult<()> {
    // Get args, command and matches
    let args: Vec<String> = args().skip(1).collect::<Vec<_>>();
    let mut command = get_matches();
    let matches: ArgMatches = match get_matches().try_get_matches_from(args) {
        Ok(m) => m,
        // Handle errors, includes `--help` and `--version`
        Err(e) => {
            e.exit();
        }
    };
    // Match result and execute
    let (cmd, sub) = matches.subcommand().unwrap_or(("", &matches));
    match (cmd, sub.subcommand()) {
        ("base32", Some(("encode", m))) => {
            base32_encode(m, &mut command);
        }
        ("base32", Some(("decode", m))) => {
            base32_decode(m, &mut command);
        }
        ("base64", Some(("encode", m))) => {
            base64_encode(m, &mut command);
        }
        ("base64", Some(("decode", m))) => {
            base64_decode(m, &mut command);
        }
        ("caesar", Some(("encrypt", m))) => {
            caesar_encrypt(m, &mut command);
        }
        ("caesar", Some(("decrypt", m))) => {
            caesar_decrypt(m, &mut command);
        }
        ("rot13", Some(("encrypt", m))) => {
            rot13_rotate(m, &mut command);
        }
        ("rot13", Some(("decrypt", m))) => {
            rot13_rotate(m, &mut command);
        }
        ("rot13", Some(("rotate", m))) => {
            rot13_rotate(m, &mut command);
        }
        _ => {
            error(
                "Command not recognised or not provided".to_string(),
                Some((ErrorKind::MissingSubcommand, &mut command)),
            );
        }
    }
    Ok(())
}
