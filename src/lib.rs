mod st_error;
pub mod types;

use std::error::Error;
use std::fs;

use clap::ArgMatches;

use image::io::Reader as ImageReader;

use types::{ArgType, DecodeArgs, EncodeArgs};

/// Matches program commands to specified subcommand
pub fn match_subcommand<'a>(matches: &'a ArgMatches) -> Result<ArgType<'a>, &'static str> {
    let image;

    if let Some(ref matches) = matches.subcommand_matches("encode") {
        image = matches
            .value_of("image")
            .expect("Couldn't get input image string");
        let text = matches
            .value_of("text")
            .expect("Couldn't get value of input text");
        let out_img = matches.value_of("out_img");

        // return Ok(Args::new(image, Some(text), None, CommandType::Encode));
        return Ok(ArgType::Encode(EncodeArgs::new(image, text, out_img)));
    } else if let Some(ref matches) = matches.subcommand_matches("decode") {
        image = matches
            .value_of("image")
            .expect("Couldn't get input image string");
        // TODO: Less verbose error ouput on unparseable text size
        let text_size = matches
            .value_of("text_size")
            .expect("Couldn't get text size value")
            .parse()
            .expect("Couldn't convert text size to number");

        return Ok(ArgType::Decode(DecodeArgs::new(image, text_size)));
    }

    Err("No legal subcommand found")
}

/// Encodes image with specified args
pub fn encode(args: &EncodeArgs) -> Result<(), Box<dyn Error>> {
    // TODO: Implement on UTF-8
    let text = fs::read_to_string(args.text_path)?.as_bytes().to_vec();
    let mut img = ImageReader::open(args.image_path)?.decode()?.to_rgba8();

    // Need heuristically 2 pixels for every ASCII character (4 channels/pixel)
    if (2 * text.len()) > img.len() {
        return Err(Box::new(st_error::ImageSizeError));
    }

    let mut img_x = 0;
    let mut img_y = 0;
    let mut pxl_channel = 0;

    for letter in text {
        // Start with MSB of letter
        let mut bit_ptr: u8 = 1 << 6;

        while bit_ptr != 0 {
            // Reset pixel channel when OOB for RGBA
            if pxl_channel >= 4 {
                pxl_channel = 0;
                img_x += 1;
            }
            // Move to start of next row when OOB on image width
            if img_x >= img.width() {
                img_y += 1;
                img_x = 0;
            }

            let pxl = img.get_pixel_mut(img_x, img_y);
            if letter & bit_ptr > 0 {
                pxl[pxl_channel] |= 1;
            } else {
                pxl[pxl_channel] &= !1;
            }

            bit_ptr >>= 1;
            pxl_channel += 1;
        }
    }

    img.save(args.out_img.unwrap_or(args.image_path))?;

    Ok(())
}

/// Decodes image from specified args
pub fn decode(args: &DecodeArgs) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(args.image_path)?.decode()?.to_rgba8();

    let mut decoded_buf: Vec<u8> = Vec::with_capacity(args.text_size);
    let mut cur_char: u8 = 0;

    let mut img_x = 0;
    let mut img_y = 0;
    let mut channel = 0;
    let mut bit_ptr: u8 = 1 << 6;

    while decoded_buf.len() < decoded_buf.capacity() {
        if bit_ptr == 0 {
            decoded_buf.push(cur_char);

            bit_ptr = 1 << 6;
            cur_char = 0;
            continue;
        }

        if channel >= 4 {
            channel = 0;
            img_x += 1;
        }

        if img_x >= img.width() {
            img_y += 1;
            img_x = 0;
        }

        let pxl = img.get_pixel(img_x, img_y);
        if pxl[channel] & 1 > 0 {
            cur_char |= bit_ptr;
        } else {
            cur_char &= !bit_ptr;
        }

        bit_ptr >>= 1;
        channel += 1;
    }

    println!("{}", String::from_utf8_lossy(&decoded_buf));

    Ok(())
}
