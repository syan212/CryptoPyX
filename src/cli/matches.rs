use clap::crate_version;
use clap::{Arg, Command};

pub fn get_matches() -> Command {
    Command::new("CryptoPyX")
        .about("CryptoPyX - CLI interface for fast cryptographic operations")
        .version(crate_version!())
        .subcommand(
            Command::new("base32")
                .about("Base32 encoding and decoding.")
                .subcommand(
                    Command::new("encode")
                        .about("Encode a string into base32.")
                        .arg(Arg::new("data").required(true)),
                )
                .subcommand(
                    Command::new("decode")
                        .about("Decode a string into base32.")
                        .arg(Arg::new("data").required(true)),
                ),
        )
}
