use clap::Command;
use clap::error::ErrorKind;
use colored::Colorize;
use std::fs;
use std::process::exit;

/// Alert user of unexpected error and details and then exit
pub fn error(message: String, error_kind: Option<(ErrorKind, &mut Command)>) -> ! {
    match error_kind {
        None => {
            eprintln!("Unexpected error occurred.\nDetails: {}", message.red());
            exit(1);
        }
        Some((kind, cmd)) => {
            let e = cmd.error(kind, message);
            e.exit();
        }
    }
}

/// Convert bytes to `String` with error handling
pub fn utf8_string(bytes: Vec<u8>, cmd: &mut Command) -> String {
    match String::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => {
            let e = cmd.error(ErrorKind::InvalidUtf8, "Unable to convert bytes to string.");
            e.exit();
        }
    }
}

/// Write to file if output is Some, else print to stdout
pub fn output_string(output: Option<&String>, data: String, cmd: &mut Command) {
    if let Some(output) = output {
        // Write to file
        fs::write(output, data).unwrap_or_else(|_| {
            error(
                format!("Could not write to file: {}", output),
                Some((ErrorKind::Io, cmd)),
            )
        });
        println!(
            "{}",
            format!("Successfully wrote data to {}", output).green()
        );
    } else {
        // Output to stdout
        println!("{}", data.green());
    }
}

/// Write vec to file if output is Some, else raise error
pub fn output_vec(output: Option<&String>, data: Vec<u8>, cmd: &mut Command) {
    if let Some(output) = output {
        // Write to file
        fs::write(output, data).unwrap_or_else(|_| {
            error(
                format!("Could not write to file: {}", output),
                Some((ErrorKind::Io, cmd)),
            )
        });
        println!(
            "{}",
            format!("Successfully wrote data to {}", output).green()
        );
    } else {
        utf8_string(data, cmd);
    }
}
