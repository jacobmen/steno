use std::error::Error;
use std::fmt;

// TODO: Add fields to error for better messages
#[derive(Debug, Clone)]
pub struct ImageSizeError;

impl Error for ImageSizeError {}

impl fmt::Display for ImageSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image too small to encode text")
    }
}
