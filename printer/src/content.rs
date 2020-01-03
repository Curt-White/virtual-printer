use std::str::{from_utf8, Utf8Error};
use crate::printer::{Justification, Underline};

pub struct Image {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

impl Image {
    pub fn from(data: Vec<u8>, width: u16, height: u16) -> Image {
        Image {
            width,
            height,
            data,
        }
    }
}

#[derive(Debug)]
pub struct TextFormat {
    pub justification: Justification,
    pub bold: bool,
    pub height_mag: u8,
    pub width_mag: u8,
    pub underline: Underline,
}

pub struct Text {
    pub format: TextFormat,
    pub data: Vec<u8>,
}

impl Text {
    pub fn from(content: Vec<u8>, format: TextFormat) -> Result<Text, Utf8Error> {
        Ok(Text {
            format,
            data: content
        })
    }

    pub fn append(&mut self, data: Vec<u8>) -> Result<(), Utf8Error> {
        self.data.extend(data.iter());
        return Ok(());
    }
}
