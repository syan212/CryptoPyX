use anstyle::{AnsiColor, Color, Style};
use clap::builder::Styles;
use clap::{Arg, ArgAction, Command, crate_version, value_parser};

fn get_styles() -> Styles {
    Styles::styled()
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))))
}

pub fn get_matches() -> Command {
    Command::new("CryptoPyX")
        .about("CryptoPyX - CLI interface for fast cryptographic operations.")
        .version(crate_version!())
        .subcommand_required(true)
        .subcommand(
            Command::new("base32")
                .subcommand_required(true)
                .about("Base32 encoding and decoding.")
                .subcommand(
                    Command::new("encode")
                        .about("Encode data into base32. Data can be a binary file, but then outputting to stdout is not allowed.")
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
                    Command::new("decode Output data can be binary, but then output must be to a file.")
                        .about("Decode base32.")
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
        .subcommand(
            Command::new("base64")
                .subcommand_required(true)
                .about("Base64 encoding and decoding.")
                .subcommand(
                    Command::new("encode")
                        .about("Encode data into base64. Data can be a binary file, but then outputting to stdout is not allowed.")
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
                        .about("Decode base64. Output data can be binary, but then output must be to a file.")
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
                )
        )
        .subcommand(
            Command::new("caesar")
                .subcommand_required(true)
                .about("Caesar encryption and decryption. Data must be utf8 and all non-alphabetical characters are ignored.")
                .subcommand(
                    Command::new("encrypt")
                        .about("Encrypt a string using Caesar shift.")
                        .arg(
                            Arg::new("data")
                                .help("Data to encode. Can be a file name (default behaviour) or a raw string (pass in -s).")
                                .required(true),
                        )
                        .arg(
                            Arg::new("shift")
                                .help("Shift to use when encrypting data.")
                                .value_parser(value_parser!(i8).range(-25..=25))
                                .required(true)
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
                                .help("Sets the file output of the encrypted data. Default is stdout.")
                                .short('o')
                                .long("output")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("decrypt")
                        .about("Decrypt data using Caesar shift.")
                        .arg(
                            Arg::new("data")
                                .help("Data to decode. Can be a file name (default behaviour) or a raw string (pass in -s).")
                                .required(true),
                        )
                        .arg(
                            Arg::new("shift")
                                .help("Shift to use when decrypting data. Should be the same as the one used to encrypt the data.")
                                .value_parser(value_parser!(i8).range(-25..=25))
                                .required(true)
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
                )
        )
        .subcommand(
            Command::new("rot13")
                .subcommand_required(true)
                .about("Rot13 encryption and decryption. Data must be utf8 and all non-alphabetical characters are ignored.")
                .subcommand(
                    Command::new("encrypt")
                        .about("\"Encrypt\" a string using rot13. Same as decrypt and rotate.")
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
                                .help("Sets the file output of the encrypted data. Default is stdout.")
                                .short('o')
                                .long("output")
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("decrypt")
                        .about("\"Decrypt\" data using rot13. Same as encrypt and rotate.")
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
                )
                .subcommand(
                    Command::new("rotate")
                        .about("Rotate data using rot13. Same as encrypt and decrypt.")
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
                )
        )
        .styles(get_styles())
}
