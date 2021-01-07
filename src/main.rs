use clap::{App, Arg, crate_version};

fn main() {
    let matches = App::new("Steno")
        .version(crate_version!())
        .author("Jacob Men")
        .about("Stenography library for encoding and decoding text to/from an image")
        .subcommand(
            App::new("encode")
                .arg(
                    Arg::with_name("text")
                        .help("Text to encode in the image")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("image")
                        .help("Image to encode text in")
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
        )
        .get_matches();
}
