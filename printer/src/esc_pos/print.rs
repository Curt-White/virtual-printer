use parse_macro::gen_lex;
use crate::error::PrinterError;
use crate::printer::Printer;

/// Feed the paper
pub fn line_feed(printer: &mut Printer, _bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    printer.feed_line();
    return Ok(());
}

#[gen_lex(1)]
struct PrintAndFeedArgs { n: u8 }

pub fn print_and_feed(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let args: PrintAndFeedArgs = parse_print_and_feed_args(bytes)?;
    for i in 0..args.n {
        printer.feed_line();
    }

    return Ok(());
}
