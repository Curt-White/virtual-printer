use parse_macro::gen_lex;
use crate::error::{PrinterError, Code};
use crate::printer::Printer;

/// Proceed bit image commands denoting the format and size of the image data
enum BitImageHeaders {
    BlockTransfer(BlockTransferHeader), // Max 65535 bytes
    BatchTransfer(BatchTransferHeader) // Max 4.3 million bytes
}

#[gen_lex(1, 1, 1, 1)]
#[derive(Debug)]
struct BlockTransferHeader {
    pub pl: u8,
    pub ph: u8,
    pub m: u8,
    pub func: u8,
}

#[gen_lex(1, 1, 1, 1, 1, 1)]
#[derive(Debug)]
struct BatchTransferHeader {
    pub p1: u8,
    pub p2: u8,
    pub p3: u8,
    pub p4: u8,
    pub m: u8,
    pub func: u8,
}

fn de_multiplex_command(header: BitImageHeaders) -> Result<(), PrinterError> {
    let mut func_id: u8 = 0;
    let mut batch = false;

    match header {
        BitImageHeaders::BlockTransfer(h) => { func_id = h.func; batch = false},
        BitImageHeaders::BatchTransfer(h) => { func_id = h.func; batch = true },
    };

    match (func_id, batch) {
        (0, true) | (48, true) => {
            println!("{:?}", func_id)
        },
        (112, _) => {
            println!("running {:?}", func_id)
        }
        _ => return Err(PrinterError{
            code: Code::InvalidFunction,
            message: String::from("Invalid Bit Image Function"),
        })
    }

    return Ok(());
}

/// Bit image 2 argument variant for specifying image data in batch
pub fn command_2_arg(bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let header: BlockTransferHeader = parse_block_transfer_header(bytes)?;
    de_multiplex_command(BitImageHeaders::BlockTransfer(header))?;
    return Ok(());
}

pub fn command_4_arg(bytes: &mut Vec<u8>) -> Result<(), PrinterError>  {
    let header: BatchTransferHeader = parse_batch_transfer_header(bytes)?;
    return Ok(());
}