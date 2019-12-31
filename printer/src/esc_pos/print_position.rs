use parse_macro::gen_lex;

use crate::printer::{Printer, Justification};
use crate::error::{PrinterError, Code};

#[gen_lex(1)]
struct SetJustificationArgs {
    pub n: u8
}

/// Set the current printer justification. Set using the "ESC a" command
pub fn set_justification(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>   {
    let args: SetJustificationArgs = parse_set_justification_args(bytes)?;
    println!("setting justification");

    let justification = match args.n {
        0 | 48 => Justification::Left,
        1 | 49 => Justification::Center,
        2 | 50 => Justification::Right,
        _ => return Err(PrinterError {
            code: Code::InvalidByteSequence,
            message: format!("Invalid Value for Setting Justification. Provided {:X}", args.n).to_string(),
        })
    };

    printer.set_justification(justification);
    return Ok(());
}

