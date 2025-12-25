use clap::Command;
use clap::error::ErrorKind;
use colored::Colorize;
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
            // get_matches()
            //     .error(ErrorKind::InvalidUtf8, "Unable to convert bytes to string.")
            //     .exit();
            let e = cmd.error(ErrorKind::InvalidUtf8, "Unable to convert bytes to string.");
            e.exit();
        }
    }
}
