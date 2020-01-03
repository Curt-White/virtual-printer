use crate::content::{Text, Image, TextFormat};
use crate::error::{PrinterError, Code};

use std::str::Utf8Error;
use crate::formatter::{HTMLFormatter, Formatter};
use std::fmt::{Display, Error};

#[derive(PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Underline {
    None = 0,
    SingleWidth = 1,
    DoubleWidth = 2
}

impl Display for Underline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        match *self {
            Underline::None => write!(f, "0"),
            Underline::SingleWidth => write!(f, "1"),
            Underline::DoubleWidth => write!(f, "2")
        }
    }
}

pub struct Printer {
    pub mode: Mode,
    pub formatter: HTMLFormatter,
    pub text_buffer:   Vec<Text>,
    pub image_buffer:  Vec<Image>,
    // Formatting Modes
    pub justification: Justification,
    pub bold: bool,
    pub height_mag: u8,
    pub width_mag: u8,
    /// The thickness of underline as multiple of size 0 is none, 1 is 1x, 2 is 2x ...
    pub underline: bool,
    pub underline_width: Underline,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            mode: Mode::Standard,
            formatter: HTMLFormatter::new(),
            text_buffer: Vec::new(),
            image_buffer: Vec::new(),
            justification: Justification::Left,
            bold: false,
            width_mag: 1,
            height_mag: 1,
            underline: false,
            underline_width: Underline::None,
        }
    }
    /// Add the provided text to the printers printing buffed
    pub fn buffer_text(&mut self, text: &mut Vec<u8>) -> Option<Utf8Error>  {
        let format = TextFormat {
            justification: self.justification,
            bold: self.bold,
            height_mag: self.height_mag,
            width_mag: self.width_mag,
            underline: if self.underline { self.underline_width } else { Underline::None },
        };

        let text = Text::from(text.to_owned(), format);
        match text {
            Ok(text) => {self.text_buffer.push(text); return None},
            Err(e) => return Some(e),
        }
    }

    /// Set the justification of the printer. The printers justification
    pub fn set_justification(&mut self, opt: Justification) -> Result<(), PrinterError>  {
        // Printers cannot set the justification while in page mode
        if self.mode == Mode::Page {
            return Err(PrinterError {
                code: Code::InvalidFunction,
                message: "Can Not Set Justification in Page Mode".to_string()
            });
        }

        self.justification = opt;
        return Ok(());
    }


    pub fn toggle_underline(&mut self, on: bool) { self.underline = on }
    pub fn toggle_bold(&mut self, on: bool) { self.bold = on; }

    /// Underline Mode holds
    pub fn set_underline_mode(&mut self, mode: Underline) { self.underline_width = mode }
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