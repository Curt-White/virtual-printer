use parse_macro::gen_lex;

use crate::error::{PrinterError, Code};
use crate::printer::Printer;
use crate::command::PrinterFunc;

/// Proceed bit image commands denoting the format and size of the image data
enum BitImageHeaders {
    BlockTransfer(BlockTransferHeader), // Max 65,535 bytes
    BatchTransfer(BatchTransferHeader) // Max 4.3 million bytes
}

#[gen_lex(1, 1, 1, 1)]
struct BlockTransferHeader {
    pub pl: u8,
    pub ph: u8,
    pub m: u8,
    pub func: u8,
}

#[gen_lex(1, 1, 1, 1, 1, 1)]
struct BatchTransferHeader {
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub p4: u8,
    pub m: u8,
    pub func: u8,
}

/// Bit image 2 argument variant for specifying image data in block
pub fn command_2_arg(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let header = parse_block_transfer_header(bytes)?;
    let function = de_multiplex_command(BitImageHeaders::BlockTransfer(header), printer, bytes)?;
    let res = function(printer, bytes)?;

    return Ok(());
}

/// Bit image command with 4 arguments variant for large batch image data
/// larger than 65,535 bytes in size
pub fn command_4_arg(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>   {
    let header = parse_batch_transfer_header(bytes)?;
    let function = de_multiplex_command(BitImageHeaders::BatchTransfer(header), printer, bytes)?;
    let res = function(printer, bytes)?;

    return Ok(());
}

/// Parse and select the function that will be used stored in the fn parameter in the bytestream
fn de_multiplex_command(header: BitImageHeaders, printer: &mut Printer,
                                bytes: &mut Vec<u8>) -> Result<PrinterFunc, PrinterError> {
    let mut func_id: u8 = 0;
    let mut batch = false;

    match header {
        BitImageHeaders::BlockTransfer(h) => { func_id = h.func; batch = false},
        BitImageHeaders::BatchTransfer(h) => { func_id = h.func; batch = true },
    };

    match (func_id, batch) {
//        (0, true) | (48, true) => Ok(|printer: &mut Printer| -> Option<PrinterError> {
//            return None;
//        }),
        (2, false) | (50, false) => Ok(print_image),
        (112, _) => Ok(store_raster_image),
        _ => return Err(PrinterError{
            code: Code::InvalidFunction,
            message: String::from(format!("Invalid Bit Image Function: ID {:?}", func_id)),
        })
    }
}

fn print_image(printer: &mut Printer, _bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    printer.print_image()?;
    return Ok(());
}

#[gen_lex(1, 1, 1, 1, 1, 1, 1, 1,
    ((((xl as u16 + xh as u16 * 256) + 7)/8) * (yl as u16 + yh as u16 * 256)))]
struct StoreRasterImageArgs {
    tone: u8, // 'a' the tone of the graphics
    bx: u8,
    by: u8,
    color: u8, // 'c' the color depending on printer
    xl: u8,
    xh: u8,
    yl: u8,
    yh: u8,
    data: Vec<u8>,
}

fn store_raster_image(printer: &mut Printer, bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let a = parse_store_raster_image_args(bytes)?;

    return Ok(());
}
