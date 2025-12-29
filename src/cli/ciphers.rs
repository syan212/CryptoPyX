use crate::cli::utils::*;
use crate::ciphers::{caesar, rot13, vigenere};
use clap::error::ErrorKind;
use clap::{ArgMatches, Command};
use colored::Colorize;
use std::fs;

pub fn caesar_encrypt(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    let shift = m.get_one::<i8>("shift").unwrap_or_else(|| {
        error(
            "Argument <shift> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: String = if string {
        caesar::caesar_rust(&data, *shift, caesar::Mode::Encrypt)
    } else {
        // Get data from file
        let data = fs::read_to_string(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        caesar::caesar_rust(&data, *shift, caesar::Mode::Encrypt)
    };
    // Output data
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

pub fn caesar_decrypt(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    let shift = m.get_one::<i8>("shift").unwrap_or_else(|| {
        error(
            "Argument <shift> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: String = if string {
        caesar::caesar_rust(&data, *shift, caesar::Mode::Decrypt)
    } else {
        // Get data from file
        let data = fs::read_to_string(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        caesar::caesar_rust(&data, *shift, caesar::Mode::Decrypt)
    };
    // Output data
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
