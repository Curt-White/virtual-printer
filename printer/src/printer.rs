use crate::content::{Text, Image, TextFormat};
use crate::error::PrinterError;

use std::str::Utf8Error;
use crate::formatter::{HTMLFormatter, Formatter};

pub enum Mode {
    Page,
    Standard
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Justification {
    Left,
    Center,
    Right
}

pub struct Printer {
    pub formatter: HTMLFormatter,
    pub text_buffer:   Vec<Text>,
    pub image_buffer:  Vec<Image>,
    // Formatting Modes
    pub justification: Justification,
    pub bold: bool,
    pub height_mag: u8,
    pub width_mag: u8,
    pub underline: bool,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            formatter: HTMLFormatter::new(),
            text_buffer: Vec::new(),
            image_buffer: Vec::new(),
            justification: Justification::Left,
            bold: false,
            width_mag: 1,
            height_mag: 1,
            underline: false,
        }
    }
    /// Add the provided text to the printers printing buffed
    pub fn buffer_text(&mut self, text: &mut Vec<u8>) -> Option<Utf8Error>  {
        let format = TextFormat {
            justification: self.justification,
            bold: self.bold,
            height_mag: self.height_mag,
            width_mag: self.width_mag,
            underline: self.underline,
        };

        let text = Text::from(text.to_owned(), format);
        match text {
            Ok(text) => {self.text_buffer.push(text); return None},
            Err(e) => return Some(e),
        }
    }

    pub fn set_justification(&mut self, opt: Justification) { self.justification = opt; }
    pub fn set_underline_mode(&mut self, on: bool) { self.underline = on }
    pub fn set_bold_mode(&mut self, on: bool) { self.bold = on; }
    pub fn set_width_mag(&mut self, scale: u8) { self.width_mag = scale }
    pub fn set_height_mag(&mut self, scale: u8) { self.height_mag = scale }

    /// Add the image to the image buffer
    pub fn buffer_image(&mut self, image: Image) {
        self.image_buffer.push(image);
    }

    pub fn feed_line(&mut self) {
        let texts = self.text_buffer.drain(..);
        for item in texts {
            self.formatter.format_text(item);
        }

        self.formatter.new_line();
    }

    pub fn close_document(&mut self) -> String {
        self.formatter.close()
    }

    pub fn print_image(&mut self) -> Result<(), PrinterError> {
        println!("Printing the Image from Buffer");
        return Ok(());
    }
}