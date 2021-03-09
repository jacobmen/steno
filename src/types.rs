/// Enum representing the possible subcommands and their respective args
pub enum ArgType<'a> {
    Encode(EncodeArgs<'a>),
    Decode(DecodeArgs<'a>),
}

/// Arguments for image encoding
pub struct EncodeArgs<'a> {
    /// Path of image file to encode text in
    pub image_path: &'a str,
    /// Path of text file to encode
    pub text_path: &'a str,
    /// Optional argument for new image file to create with encoding
    pub out_img: Option<&'a str>,
}

/// Arguments for text decoding from image
pub struct DecodeArgs<'a> {
    /// Path of image file to encode text in
    pub image_path: &'a str,
    /// Number of characters to extract from image
    pub text_size: usize,
}

impl<'a> EncodeArgs<'a> {
    pub fn new(
        image_path: &'a str,
        text_path: &'a str,
        out_img: Option<&'a str>,
    ) -> EncodeArgs<'a> {
        EncodeArgs {
            image_path,
            text_path,
            out_img,
        }
    }
}

impl<'a> DecodeArgs<'a> {
    pub fn new(image_path: &'a str, text_size: usize) -> DecodeArgs<'a> {
        DecodeArgs {
            image_path,
            text_size,
        }
    }
}

// pub fn test() -> ArgType<'static> {
//     ArgType::Decode(DecodeArgs {
//         image_path: "sada",
//         text_size: 32,
//     })
// }
