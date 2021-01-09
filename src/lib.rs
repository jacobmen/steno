use clap::ArgMatches;

pub struct Args<'a> {
    pub image: &'a str,
    pub text: Option<&'a str>,
    pub command: CommandType,
}

pub enum CommandType {
    Encode,
    Decode,
}

impl<'a> Args<'a> {
    fn new(image: &'a str, text: Option<&'a str>, command: CommandType) -> Args<'a> {
        Args {
            image,
            text,
            command,
        }
    }
}

pub fn match_subcommand<'a>(matches: &'a ArgMatches) -> Result<Args<'a>, &'static str> {
    let image;
    let text;

    if let Some(ref matches) = matches.subcommand_matches("encode") {
        image = matches.value_of("image").expect("Couldn't get input image string");
        text = matches.value_of("text").expect("Couldn't get value of input text");

        return Ok(Args::new(image, Some(text), CommandType::Encode));
    } else if let Some(ref matches) = matches.subcommand_matches("decode") {
        image = matches.value_of("image").expect("Couldn't get input image string");

        return Ok(Args::new(image, None, CommandType::Decode));
    }

    Err("No legal subcommand found")
}
