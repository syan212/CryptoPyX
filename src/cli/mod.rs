use crate::encodings::base32 as b32;
use clap::ArgMatches;
use clap::error::{Error, ErrorKind::DisplayHelp, ErrorKind::DisplayVersion};
use colored::Colorize;
use matches::get_matches;
use pyo3::prelude::*;
use std::{env::args, process::exit};

mod matches;

fn no_commands() -> ! {
    eprintln!(
        "{}",
        "No commands provided. For more information, try '--help'.".red()
    );
    exit(1);
}

fn unexpected_error(message: String) -> ! {
    eprintln!("Unexpected error occurred.\nDetails: {}", message.red());
    exit(1);
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
        // Display help or version
        Err(e) if e.kind() == DisplayHelp || e.kind() == DisplayVersion => {
            e.print()?;
            exit(0);
        }
        // Handle other errors
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    // Match result and execute
    let (cmd, sub) = matches.subcommand().unwrap_or(("", &matches));
    match (cmd, sub.subcommand()) {
        ("base32", Some(("encode", m))) => {
            let data = m
                .get_one::<String>("data")
                .unwrap_or_else(|| unexpected_error("Argument <data> was not found".to_string()));
            let out = String::from_utf8(b32::encode_bytes_rust(data.as_bytes()))?;
            println!("{}", out.green());
        }
        ("base32", Some(("decode", m))) => {
            let data = m
                .get_one::<String>("data")
                .unwrap_or_else(|| unexpected_error("Argument <data> was not found".to_string()));
            let out = String::from_utf8(b32::decode_bytes_rust(data.as_bytes())?)?;
            println!("{}", out.green());
        }
        _ => {
            no_commands();
        }
    }
    Ok(())
}
