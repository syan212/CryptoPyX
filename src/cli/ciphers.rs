use crate::ciphers::{caesar, rot13, vigenere};
use crate::cli::utils::*;
use clap::error::ErrorKind;
use clap::{ArgMatches, Command};
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
        caesar::caesar_rust(data, *shift, caesar::Mode::Encrypt)
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
    output_string(output_location, out, command);
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
        caesar::caesar_rust(data, *shift, caesar::Mode::Decrypt)
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
    output_string(output_location, out, command);
}

pub fn rot13_rotate(m: &ArgMatches, command: &mut Command) {
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
        rot13::rot13_rust(data)
    } else {
        // Get data from file
        let data = fs::read_to_string(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        rot13::rot13_rust(&data)
    };
    // Output data
    output_string(output_location, out, command);
}

pub fn vigenere_encrypt(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let skip_non_alpha = m.get_flag("skip-non-alpha");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    let key = m.get_one::<String>("key").unwrap_or_else(|| {
        error(
            "Argument <key> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: String = if string {
        match vigenere::vigenere_rust(data, key, vigenere::Mode::Encrypt, skip_non_alpha) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), Some((ErrorKind::InvalidValue, command))),
        }
    } else {
        // Get data from file
        let data = fs::read_to_string(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        match vigenere::vigenere_rust(&data, key, vigenere::Mode::Encrypt, skip_non_alpha) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), Some((ErrorKind::InvalidValue, command))),
        }
    };
    // Output data
    output_string(output_location, out, command);
}

pub fn vigenere_decrypt(m: &ArgMatches, command: &mut Command) {
    // Get options and args with error handling
    let output_location = m.get_one::<String>("output");
    let string = m.get_flag("string");
    let skip_non_alpha = m.get_flag("skip-non-alpha");
    let data = m.get_one::<String>("data").unwrap_or_else(|| {
        error(
            "Argument <data> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    let key = m.get_one::<String>("key").unwrap_or_else(|| {
        error(
            "Argument <key> was not found".to_string(),
            Some((ErrorKind::MissingRequiredArgument, command)),
        )
    });
    // Compute output
    let out: String = if string {
        match vigenere::vigenere_rust(data, key, vigenere::Mode::Decrypt, skip_non_alpha) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), Some((ErrorKind::InvalidValue, command))),
        }
    } else {
        // Get data from file
        let data = fs::read_to_string(data).unwrap_or_else(|_| {
            error(
                format!("Could not read file: {}", data),
                Some((ErrorKind::Io, command)),
            )
        });
        match vigenere::vigenere_rust(&data, key, vigenere::Mode::Decrypt, skip_non_alpha) {
            Ok(out) => out,
            Err(e) => error(e.to_string(), Some((ErrorKind::InvalidValue, command))),
        }
    };
    // Output data
    output_string(output_location, out, command);
}
