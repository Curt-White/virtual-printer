use parse_macro::gen_lex;
use crate::error::{PrinterError, Code};
use crate::printer::Printer;

#[gen_lex(1)]
struct PrinterModeArgs {
    n: u8,
}

pub fn set_printer_mode(bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args = parse_printer_mode_args(bytes)?;
    return Ok(());
}