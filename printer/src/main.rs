use std::fs;

mod esc_pos;
mod error;
mod printer;
mod content;
mod formatter;
mod command;

use esc_pos::tree::query_command;
use crate::printer::Printer;

fn main() {
    let mut contents = fs::read("/Users/curtwhite/Desktop/Projects/virtual-printer/receipt-with-logo.bin")
        .expect("Something went wrong reading the file");
    let mut printer = Printer::new();
    let mut text: Vec<u8> = Vec::new();

    loop {
        let a = query_command(&mut contents);
        match a {
            Ok(val) => {
                if text.len() > 0 {
                    printer.buffer_text(&mut text);
                    text.drain(..);
                }

                let a = val(&mut printer, &mut contents);
                println!("{:?}", a);
            },
            Err(e) => match e.code {
                error::Code::InvalidCommand => {
                    if contents.len() > 0 {
                        text.push(contents.remove(0));
                    } else {
                        return;
                    }
                    println!("{:?}", text);
                },
                _ => {
                    println!("An Error Occurred: {:?}, Code: {:?}", e.message, e.code);
                    return;
                }
            }
        };
    }
}
