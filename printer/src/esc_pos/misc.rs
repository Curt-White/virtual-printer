use crate::error::{PrinterError};
use crate::printer::Printer;

pub fn init_printer(_printer: &mut Printer, _bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    println!("Init Printer");
    return Ok(());
}
