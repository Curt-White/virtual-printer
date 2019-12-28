use std::str::{from_utf8, Utf8Error};

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

pub struct TextFormat {
    pub bold: bool,
    pub height_mag: u8,
    pub width_mag: u8,
    pub underline: bool,
}

pub struct Text {
    pub format: TextFormat,
    pub data: String,
}

impl Text {
    pub fn from(content: Vec<u8>, format: TextFormat) -> Result<Text, Utf8Error> {
        Ok(Text {
            format,
            data: String::from(from_utf8(content.as_ref())?),
        })
    }

    pub fn append(&mut self, data: Vec<u8>) -> Result<(), Utf8Error> {
        let value = from_utf8(data.as_ref())?;
        self.data.push_str(value);
        return Ok(());
    }
}
