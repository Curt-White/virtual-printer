use crate::error::PrinterError;
use crate::printer::Printer;

pub fn line_feed(printer: &mut Printer, _bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    println!("Line Feed");
    printer.feed_line();
    return Ok(());
}