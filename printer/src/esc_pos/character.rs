use parse_macro::gen_lex;
use crate::error::{PrinterError, Code};
use crate::printer::{ Printer, Underline };

#[gen_lex(1)]
struct PrinterModeArgs {
    n: u8,
}

/// Set the printer mode, this is denote by "ESC !" command
pub fn set_printer_mode(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args: PrinterModeArgs = parse_printer_mode_args(bytes)?;

    if args.n & 0x00 > 0 { println!("Font One") } else { println!("Font Two") };
    if args.n & 0x08 > 0 { printer.toggle_bold(true) } else { printer.toggle_bold(false) };
    if args.n & 0x10 > 0 { printer.set_height_mag(2) } else { printer.set_height_mag(1) };
    if args.n & 0x20 > 0 { printer.set_width_mag(2) } else { printer.set_width_mag(1) };
    if args.n & 0x80 > 0 { printer.toggle_underline(true) } else { printer.toggle_underline(false) };

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
    printer.toggle_bold(is_bold);
    return Ok(());
}

#[gen_lex(1)]
struct UnderlineModeArgs {
    n: u8,
}

/// Change the underline mode, this is denote by "ESC -" command
pub fn set_underline_mode(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args: UnderlineModeArgs = parse_underline_mode_args(bytes)?;
    let mode = match args.n {
        0 | 48 => (false, Underline::None),
        1 | 49 => (true, Underline::SingleWidth),
        2 | 50 => (true, Underline::DoubleWidth),
        _ => return Err(PrinterError {
            code: Code::BadArguments,
            message: format!("Invalid Underline Mode Arguments, Got: {}", args.n).to_string()
        })
    };

    printer.toggle_underline(mode.0);
    printer.set_underline_mode(mode.1);
    return Ok(());
}
