use crate::error::{PrinterError, Code};

pub fn init_printer(_bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    println!("Init Printer");
    return Ok(());
}
