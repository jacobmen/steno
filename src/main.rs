use std::process;

use clap::{App, Arg, crate_version};

use steno::match_subcommand;
use steno::CommandType;

use steno::encode;
use steno::decode;

fn main() {
    let matches = App::new("Steno")
        .version(crate_version!())
        .author("Jacob Men")
        .about("Stenography library for encoding and decoding text to/from an image")
        .subcommand(
            App::new("encode")
                .arg(
                    Arg::with_name("image")
                        .help("Image to encode text in")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("text")
                        .help("Text to encode in the image")
                        .index(2)
                        .required(true)
                )
        )
        .subcommand(
            App::new("decode")
                .arg(
                    Arg::with_name("image")
                        .help("Image to decode text from")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("text_size")
                        .help("Number of characters to extract")
                        .index(2)
                        .required(true)
                )
        )
        .get_matches();

    let sub_args = match_subcommand(&matches).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    match sub_args.command {
        CommandType::Encode => encode(&sub_args),
        CommandType::Decode => decode(&sub_args),
    }.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    })
}
