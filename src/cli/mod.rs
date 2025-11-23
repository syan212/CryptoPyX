use pyo3::prelude::*;
use clap::{Arg, Command, ArgMatches};
use std::{env, process::exit};

#[pyfunction]
pub fn parse() -> PyResult<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let matches_result: Result<ArgMatches, clap::error::Error> = Command::new("cryptopyx")
        .subcommand(
            Command::new("greet")
                .arg(
                    Arg::new("name")
                        .required(true)
                        .value_parser(clap::builder::ValueParser::string())
                )
        )
        .try_get_matches_from(args);

    match matches_result {
        Ok(matches) => {
            let matches = matches;
            match matches.subcommand() {
                Some(("greet", m)) => {
                    let name = m.get_one::<String>("name").unwrap();
                    println!("Hello, {}", name);
                }
                _ => {}
            }
        }
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayHelp => {
            e.print()?;
            exit(0);
        }
        Err(e) if e.kind() == clap::error::ErrorKind::DisplayVersion => {
            e.print()?;
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }

    Ok(())
}