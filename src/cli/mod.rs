use crate::encodings::base32 as b32;
use clap::ArgMatches;
use clap::error::{Error, ErrorKind};
use colored::Colorize;
use matches::get_matches;
use pyo3::prelude::*;
use std::{env::args, fs, process::exit};

mod matches;

/// Alert user of unexpected error and details and then exit
fn unexpected_error(message: String) -> ! {
    eprintln!("Unexpected error occurred.\nDetails: {}", message.red());
    exit(1);
}

/// Covert bytes to `String` with error handling
fn utf8_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            get_matches()
                .error(ErrorKind::InvalidUtf8, "Unable to convert bytes to string.")
                .exit();
        }
    }
}

#[pyfunction]
pub fn parse() -> PyResult<()> {
    // Get args
    let args: Vec<String> = args().skip(1).collect::<Vec<_>>();
    // Get matches
    let matches_result: Result<ArgMatches, Error> = get_matches().try_get_matches_from(args);
    // Handle clap
    let matches: ArgMatches = match matches_result {
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
            // Get options and args with error handling
            let output_location = m.get_one::<String>("output");
            let string = m.get_flag("string");
            let data = m
                .get_one::<String>("data")
                .unwrap_or_else(|| unexpected_error("Argument <data> was not found".to_string()));
            // Compute output
            let out: String = if string {
                utf8_string(b32::encode_bytes_rust(data.as_bytes()))
            } else {
                // Get data from file
                let data = fs::read(data)
                    .unwrap_or_else(|_| unexpected_error(format!("Could not read file: {}", data)));
                utf8_string(b32::encode_bytes_rust(&data))
            };
            // Output encoded data
            if let Some(output) = output_location {
                // Write to file
                fs::write(output, out).unwrap_or_else(|_| {
                    unexpected_error(format!("Could not write to file: {}", output))
                });
                println!("Successfully wrote data to {}", output);
            } else {
                // Output to stdout
                println!("{}", out.green());
            }
        }
        ("base32", Some(("decode", m))) => {
            // Get options and args with error handling
            let output_location = m.get_one::<String>("output");
            let string = m.get_flag("string");
            let data = m
                .get_one::<String>("data")
                .unwrap_or_else(|| unexpected_error("Argument <data> was not found".to_string()));
            // Compute output
            let out: String = if string {
                utf8_string(b32::decode_bytes_rust(data.as_bytes())?)
            } else {
                // Get data from file
                let data = fs::read(data)
                    .unwrap_or_else(|_| unexpected_error(format!("Could not read file: {}", data)));
                utf8_string(b32::decode_bytes_rust(&data)?)
            };
            // Output decoded data
            if let Some(output) = output_location {
                // Write to file
                fs::write(output, out).unwrap_or_else(|_| {
                    unexpected_error(format!("Could not write to file: {}", output))
                });
                println!("Successfully wrote data to {}", output);
            } else {
                // Output to stdout
                println!("{}", out.green());
            }
        }
        _ => {
            unexpected_error("Command not recognised or not provided".to_string());
        }
    }
    Ok(())
}
