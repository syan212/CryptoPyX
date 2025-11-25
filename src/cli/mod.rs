use crate::encodings::base32 as b32;
use clap::ArgMatches;
use clap::error::{Error, ErrorKind::DisplayHelp, ErrorKind::DisplayVersion};
use matches::get_matches;
use pyo3::prelude::*;
use std::{env, process::exit};

mod matches;

fn not_found() {
    eprintln!("(Sub)command not found. For more information, try '--help'.");
    exit(1);
}

#[pyfunction]
pub fn parse() -> PyResult<()> {
    // Get args
    let args: Vec<String> = env::args().skip(1).collect::<Vec<_>>();
    // Get matches
    let matches_result: Result<ArgMatches, Error> = get_matches().try_get_matches_from(args);
    // Match result and execute
    match matches_result {
        Ok(matches) => match matches.subcommand() {
            Some(("base32", m)) => match m.subcommand() {
                Some(("encode", m)) => {
                    let data = m.get_one::<String>("data").unwrap();
                    println!(
                        "{}",
                        String::from_utf8(b32::encode_bytes_rust(data.as_bytes()))?
                    );
                }
                Some(("decode", m)) => {
                    let data = m.get_one::<String>("data").unwrap();
                    println!(
                        "{}",
                        String::from_utf8(b32::decode_bytes_rust(data.as_bytes())?)?
                    );
                }
                _ => {
                    not_found();
                }
            },
            _ => {
                not_found();
            }
        },
        // Display help
        Err(e) if e.kind() == DisplayHelp || e.kind() == DisplayVersion => {
            e.print()?;
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
