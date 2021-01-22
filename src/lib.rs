use std::fs;
use std::error::Error;

use clap::ArgMatches;

use image::io::Reader as ImageReader;

pub struct Args<'a> {
    pub image_path: &'a str,
    pub text_path: Option<&'a str>,
    pub command: CommandType,
}

pub enum CommandType {
    Encode,
    Decode,
}

impl<'a> Args<'a> {
    fn new(image_path: &'a str, text_path: Option<&'a str>, command: CommandType) -> Args<'a> {
        Args {
            image_path,
            text_path,
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

pub fn encode(args: &Args) -> Result<(), Box<dyn Error>> {
    // TODO: Implement on UTF-8
    let text = fs::read_to_string(args.text_path.unwrap())?.as_bytes().to_vec();
    let mut img = ImageReader::open(args.image_path)?.decode()?.to_rgba8();

    // TODO: Check if text can fit in image

    let mut img_x = 0;
    let mut img_y = 0;
    let mut pxl_channel = 0;

    for letter in text {
        // Start with MSB of letter
        let mut bit_ptr: u8 = 1 << 7;

        while bit_ptr != 0 {
            // Move to start of next row when OOB
            if img_x >= img.width() {
                img_y += 1;
                img_x = 0;
            }
            // Reset pixel channel when OOB for RGBA
            if pxl_channel >= 4 {
                pxl_channel = 0;
            }

            let pxl = img.get_pixel_mut(img_x, img_y);
            if letter & bit_ptr > 0 {
                pxl[pxl_channel] |= 1;
            } else {
                pxl[pxl_channel] &= !1;
            }

            bit_ptr >>= 1;
            img_x += 1;
            pxl_channel += 1;
        }
    }

    // TODO: Replace output file with something else (user input or overwrite on orig image)
    img.save("test/test_out.png")?;

    Ok(())
}

pub fn decode(args: &Args) -> Result<(), Box<dyn Error>> {
    Ok(())
}
