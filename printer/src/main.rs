use std::fs;

mod esc_pos;
mod error;
mod printer;
mod content;
mod formatter;
mod command;

use esc_pos::tree::query_command;
use crate::printer::Printer;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut contents = fs::read("/Users/curtwhite/Desktop/Projects/virtual-printer/my_receipt.bin")
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

                val(&mut printer, &mut contents);
            },
            Err(e) => match e.code {
                error::Code::InvalidCommand => {
                    if contents.len() > 0 {
                        text.push(contents.remove(0));
                    } else {
                        return;
                    }
                },
                _ => {
                    let data = printer.close_document();
                    let mut file = File::create("./test.html").expect("File Failed");
                    file.write_all(data.as_bytes());
                    println!("An Error Occurred: {:?}, Code: {:?}", e.message, e.code);
                    return;
                }
            }
        };
    }
}
