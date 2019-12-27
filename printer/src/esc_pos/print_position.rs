use parse_macro::gen_lex;

use crate::printer::Printer;
use crate::error::{PrinterError};

#[gen_lex(1)]
struct SetJustificationArgs {
    pub n: u8
}

pub fn set_justification(_printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>   {
    let args = parse_set_justification_args(bytes);
    println!("setting justification");
    return Ok(());
}

