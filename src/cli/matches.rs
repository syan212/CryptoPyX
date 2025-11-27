use clap::{Arg, Command};
use clap::{ArgAction, crate_version};

pub fn get_matches() -> Command {
    Command::new("CryptoPyX")
        .about("CryptoPyX - CLI interface for fast cryptographic operations.")
        .version(crate_version!())
        .subcommand(
            Command::new("base32")
                .about("Base32 encoding and decoding.")
                .subcommand(
                    Command::new("encode")
                        .about("Encode a string into base32.")
                        .arg(
                            Arg::new("data")
                                .help("Data to encode. Can be a file name (default behaviour) or a raw string (pass in -s).")
                                .required(true),
                        )
                        .arg(
                            Arg::new("string")
                                .help("Flag to indicate that the input data is a string.")
                                .short('s')
                                .long("string")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("output")
                                .help("Sets the file output of the encoded data. Default is stdout.")
                                .short('o')
                                .long("output")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("decode")
                        .about("Decode a string into base32.")
                        .arg(
                            Arg::new("data")
                                .help("Data to decode. Can be a file name (default behaviour) or a raw string (pass in -s).")
                                .required(true),
                        )
                        .arg(
                            Arg::new("string")
                                .help("Flag to indicate that the input data is a string.")
                                .short('s')
                                .long("string")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("output")
                                .help("Sets the file output of the decoded data. Default is stdout.")
                                .short('o')
                                .long("output")
                                .action(ArgAction::Set),
                        ),
                ),
        )
}
