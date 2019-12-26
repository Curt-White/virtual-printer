use parse_macro::gen_lex;
use std::fs::File;
use crate::error::{PrinterError, Code};
use crate::printer::Printer;
use std::io::Write;

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
pub fn command_2_arg(bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let header: BlockTransferHeader = parse_block_transfer_header(bytes)?;
    de_multiplex_command(BitImageHeaders::BlockTransfer(header), bytes)?;
    return Ok(());
}

/// Bit image command with 4 arguments variant for large batch image data
/// larger than 65,535 bytes in size
pub fn command_4_arg(bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let header: BatchTransferHeader = parse_batch_transfer_header(bytes)?;
    return Ok(());
}

/// Parse and select the function that will be used stored in the fn parameter in the bytestream
fn de_multiplex_command(header: BitImageHeaders, bytes: &mut Vec<u8>) -> Result<(), PrinterError> {
    let mut func_id: u8 = 0;
    let mut batch = false;

    match header {
        BitImageHeaders::BlockTransfer(h) => { func_id = h.func; batch = false},
        BitImageHeaders::BatchTransfer(h) => { func_id = h.func; batch = true },
    };
    println!("{:?} {:?}", func_id, batch);
    match (func_id, batch) {
        (0, true) | (48, true) => {
            println!("{:?}", func_id)
        },
        (2, false) | (50, false) => {
            println!("Printing Buffered Data {:?}", func_id)
            // cannot be used in page mode
        },
        (112, _) => {
            let a: StoreRasterImageArgs = parse_store_raster_image_args(bytes)?;
//            let mut file = File::create("done.pbm")?;
//            file.write(String::from("P4\n300 236\n").into_bytes().as_slice());
//            file.write(a.data.as_slice());
        },
        _ => return Err(PrinterError{
            code: Code::InvalidFunction,
            message: String::from("Invalid Bit Image Function"),
        })
    }

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

fn store_raster_image() -> Result<(), PrinterError> {
    return Ok(());
}
