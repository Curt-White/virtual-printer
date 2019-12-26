use crate::esc_pos::{ print_position, misc, bit_image, character };
use crate::error::{PrinterError, Code};

// Query result for query of the commands
type IntermediateResult = Result<Query, PrinterError>;

// A function which executes the specified operation on the printer
pub type Command = fn(bytes: &mut Vec<u8>) -> Result<(), PrinterError>;
pub type QueryResult = Result<Command, PrinterError>;

// Result of a command query
enum Query {
    SubQuery(fn(byte: u8) -> IntermediateResult),
    Resource(Command),
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
        0x61 => Query::Resource(print_position::set_justification),
        _ => return Err(PrinterError{
            code: Code::InvalidCommand,
            message: String::from("Invalid ESC Group Command"),
        })
    })
}

fn gs_commands(byte: u8) -> IntermediateResult {
    Ok(match byte {
        // pop off the redundant 4C after command which provides no further de-multiplexing
        0x28 => Query::Resource(|bytes: &mut Vec<u8>| {
            bytes.drain(0..1);
            bit_image::command_2_arg(bytes)
        }),
        0x38 => Query::Resource(|bytes: &mut Vec<u8>| {
            bytes.pop();
            bit_image::command_4_arg(bytes)
        }),
        _ => return Err(PrinterError{
            code: Code::InvalidCommand,
            message: String::from("Invalid GS Group Command"),
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
                bytes.drain(0..pieces + 1);
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
    println!("{:x}", bytes[0]);
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