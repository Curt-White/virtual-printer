use std::string::ParseError;
use std::error::Error;

// Specific codes for errors which occur in printing/parsing
#[derive(Debug)]
pub enum Code {
    DeprecatedCommand = 21,
    InvalidByteSequence = 22,
    InvalidCommand = 23,
    InsufficientBytes = 24,
    InvalidFunction = 25,

    UnknownError = 100,
    ParseMacroError = 101
}

#[derive(Debug)]
pub struct PrinterError {
    pub code: Code,
    pub message: String,
}

impl From<std::io::Error> for PrinterError {
    fn from(error: std::io::Error) -> Self {
       PrinterError {
           code: Code::ParseMacroError,
           message: String::from(error.description()),
       }
    }
}