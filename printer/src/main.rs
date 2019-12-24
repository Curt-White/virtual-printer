#[macro_use] extern crate log;

use parse_macro::gen_lex;
use std::fs;

mod esc_pos;
mod error;
mod printer;

use esc_pos::tree::query_command;

#[gen_lex(1, some_len+1, 1)]
#[derive(Debug)]
pub struct Tester {
    some_len: u8,
    some_item: Vec<u8>,
    some_item2: Vec<u8>,
}

fn main() {
    let mut contents = fs::read("/Users/curtwhite/Desktop/Projects/virtual-printer/receipt-with-logo.bin")
        .expect("Something went wrong reading the file");

    loop {
        let a = query_command(&mut contents);
        match a {
            Ok(val) => println!("something {:?}", val(&mut contents)),
            Err(e) => return println!("ded {:?}", e),
        }
    }
}
