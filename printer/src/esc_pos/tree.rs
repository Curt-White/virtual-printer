use crate::esc_pos::{ print_position, misc, bit_image, character, print };
use crate::error::{PrinterError, Code};
use crate::command::{PrinterFunc};

// Query result for query of the commands
type IntermediateResult = Result<Query, PrinterError>;
pub type QueryResult = Result<PrinterFunc, PrinterError>;

// Result of a command query
enum Query {
    SubQuery(fn(byte: u8) -> IntermediateResult),
    Resource(PrinterFunc),
}

// Top level commands and groupings
enum TopLevelCommands {
    HT = 0x09,
    LF = 0x0A,
    FF = 0x0C,
    CR = 0x0D,
    ESC = 0x1B,
}

// The top level command query
fn top_level_command(byte: u8) -> IntermediateResult {
    Ok(match byte {
        0x1B => Query::SubQuery(esc_commands),
        0x1D => Query::SubQuery(gs_commands),
        0x0A => Query::Resource(print::line_feed),
        _ => return Err(PrinterError{
            code: Code::InvalidCommand,
            message: String::from(format!("Invalid Top Level Command At Token: 0x{:X}", byte)),
        })
    })
}

// Query for esc command subgroup
fn esc_commands(byte: u8) -> IntermediateResult {
    Ok(match byte {
        0x21 => Query::Resource(character::set_printer_mode),
        0x40 => Query::Resource(misc::init_printer),
        0x45 => Query::Resource(character::set_emphasis_mode),
        0x61 => Query::Resource(print_position::set_justification),
        0x64 => Query::Resource(print::print_and_feed),
        _ => return Err(PrinterError{
            code: Code::BadPartialCommand,
            message: String::from(format!("Invalid ESC Group Command, Failed on Byte {:X}", byte)),
        })
    })
}

fn gs_commands(byte: u8) -> IntermediateResult {
    Ok(match byte {
        0x28 => Query::SubQuery(|byte: u8| Ok(Query::Resource(bit_image::command_2_arg))),
        0x38 => Query::SubQuery(|byte: u8| Ok(Query::Resource(bit_image::command_4_arg))),
        _ => return Err(PrinterError{
            code: Code::BadPartialCommand,
            message: String::from(format!("Invalid GS Group Command, Failed on Byte {:X}", byte)),
        })
    })
}

// Resolve a query from the bytes passed in and return a command func or error
// If there is no match for the query or only a partial path no bytes are removed
fn resolve_query(bytes: &mut Vec<u8>) -> QueryResult {
    let mut pieces = 0;
    let mut query_result = top_level_command(bytes[pieces])?;

    loop {
        match query_result {
            Query::Resource(res) => {
                let dat = bytes.drain(0..pieces + 1);
                println!("{:X?}", dat);
                return Ok(res);
            },
            Query::SubQuery(query) =>{
                pieces += 1;

                if bytes.len() <= pieces {
                    return Err (
                        PrinterError {
                            code: Code::InsufficientBytes,
                            message: String::from("No Enough Bytes to Make Command")
                        }
                    );
                }

                query_result = query(bytes[pieces])?;
            },
        }
    }
}

// Query a command and return a possible command function or error based on the
// sequence of bytes provided, the bytes that are matched are removed
pub fn query_command(bytes: &mut Vec<u8>)-> QueryResult {
    if bytes.len() == 0 {
        return Err (
            PrinterError {
                code: Code::InsufficientBytes,
                message: String::from("No Bytes In The Provided Vector"),
            }
        );
    }

    return resolve_query(bytes);
}