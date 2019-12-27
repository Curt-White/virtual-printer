use crate::content::{Text, Image, TextFormat};
use std::str::Utf8Error;
use crate::error::PrinterError;

pub enum Mode {
    Page,
    Standard
}

pub enum Justification {
    Left,
    Center,
    Right
}

pub struct Printer {
    text_buffer:   Vec<Text>,
    image_buffer:  Vec<Image>,
    // Formatting Modes
    pub justification: Justification,
    pub bold: bool,
    pub double_height: bool,
    pub double_width: bool,
    pub underline: bool,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            text_buffer: Vec::new(),
            image_buffer: Vec::new(),
            justification: Justification::Left,
            bold: false,
            double_width: false,
            double_height: false,
            underline: false,
        }
    }
    /// Add the provided text to the printers printing buffed
    fn buffer_text(&mut self, text: Vec<u8>) -> Option<Utf8Error>  {
        let format = TextFormat {
            bold: self.bold,
            double_height: self.double_height,
            double_width: self.double_width,
            underline: self.underline,
        };

        let text = Text::from(text, format);
        match text {
            Ok(text) => {self.text_buffer.push(text); return None},
            Err(e) => return Some(e),
        }
    }

    /// Add the image to the image buffer
    pub fn buffer_image(&mut self, image: Image) {
        self.image_buffer.push(image);
    }
    pub fn set_justification(&mut self, opt: Justification) {
        self.justification = opt;
    }
    pub fn feed_line() {  }

    pub fn print_image(&mut self) -> Result<(), PrinterError> {
        println!("Printing the Image from Buffer");
        return Ok(());
    }
}