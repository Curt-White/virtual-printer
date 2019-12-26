#[macro_use] extern crate log;

use parse_macro::gen_lex;
use std::fs;

mod esc_pos;
mod error;
mod printer;

use esc_pos::tree::query_command;

fn main() {
    let mut contents = fs::read("/Users/curtwhite/Desktop/Projects/virtual-printer/receipt-with-logo.bin")
        .expect("Something went wrong reading the file");

    loop {
        let a = query_command(&mut contents);
        match a {
            Ok(val) => println!("something {:?}", val(&mut contents)),
            Err(e) => match e.code {
                error::Code::InvalidCommand => {
                    return;
                },
                _ => {
                    println!("An Error Occurred: {:?}, Code: {:?}", e.message, e.code);
                }
            }
        }
    }
}
