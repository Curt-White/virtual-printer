use crate::error::{ PrinterError };
use crate::printer::Printer;

// A function which executes the specified operation on the printer
pub type PrinterFunc = fn(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>;
pub type Command = fn(bytes: &mut Vec<u8>) -> Result<PrinterFunc, PrinterError>;