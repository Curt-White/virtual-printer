use parse_macro::gen_lex;
use crate::error::{PrinterError};
use crate::printer::Printer;

#[gen_lex(1)]
struct PrinterModeArgs {
    n: u8,
}

pub fn set_printer_mode(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args: PrinterModeArgs = parse_printer_mode_args(bytes)?;

    if args.n & 0x00 > 0 { println!("Font One") } else { println!("Font Two") };
    if args.n & 0x08 > 0 { printer.set_bold_mode(true) } else { printer.set_bold_mode(false) };
    if args.n & 0x10 > 0 { printer.set_height_mag(2) } else { printer.set_height_mag(1) };
    if args.n & 0x20 > 0 { printer.set_width_mag(2) } else { printer.set_width_mag(1) };
    if args.n & 0x80 > 0 { printer.set_underline_mode(true) } else { printer.set_underline_mode(false) };

    return Ok(());
}

#[gen_lex(1)]
struct EmphasisModeArgs {
    n: u8,
}

/// Change the emphasis mode, this is denote by "ESC E" command
pub fn set_emphasis_mode(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args: EmphasisModeArgs = parse_emphasis_mode_args(bytes)?;
    let is_bold = (args.n & 0x01) == 0x01;
    printer.set_bold_mode(is_bold);
    return Ok(());
}

