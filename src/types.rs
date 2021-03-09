pub enum ArgType<'a> {
    Encode(EncodeArgs<'a>),
    Decode(DecodeArgs<'a>),
}

pub struct EncodeArgs<'a> {
    pub image_path: &'a str,
    pub text_path: &'a str,
    pub out_img: Option<&'a str>,
}

pub struct DecodeArgs<'a> {
    pub image_path: &'a str,
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
