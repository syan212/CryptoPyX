use crate::cli::utils::*;
use crate::encodings::base32 as b32;
use clap::error::ErrorKind;
use clap::{ArgMatches, Command};
use colored::Colorize;
use std::fs;

pub fn base32_encode(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: String = if string {
        utf8_string(b32::encode_bytes_rust(data.as_bytes()), command)
    } else {
        // Get data from file
        let data = fs::read(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        utf8_string(b32::encode_bytes_rust(&data), command)
    };
    // Output encoded data
    if let Some(output) = output_location {
        // Write to file
        fs::write(output, out).unwrap_or_else(|_| {
            error(
                format!("Could not write to file: {}", output),
                Some((ErrorKind::Io, command)),
            )
        });
        println!(
            "{}",
            format!("Successfully wrote data to {}", output).green()
        );
    } else {
        // Output to stdout
        println!("{}", out.green());
    }
}

pub fn base32_decode(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: Vec<u8> = if string {
        match b32::decode_bytes_rust(data.as_bytes()) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), None),
        }
    } else {
        // Get data from file (could result in binary data)
        let data = fs::read(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        match b32::decode_bytes_rust(&data) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), None),
        }
    };
    // Output decoded data
    if let Some(output) = output_location {
        // Write to file
        fs::write(output, out).unwrap_or_else(|_| {
            error(
                format!("Could not write to file: {}", output),
                Some((ErrorKind::Io, command)),
            )
        });
        println!(
            "{}",
            format!("Successfully wrote data to {}", output).green()
        );
    } else {
        // Output to stdout
        println!("{}", utf8_string(out, command).green());
    }
}
