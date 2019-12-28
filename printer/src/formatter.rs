use crate::content::{Text, TextFormat};

pub trait Formatter {
    fn new() -> Self;
    fn format_text(&mut self, data: Text) -> String;
    fn new_line(&mut self) -> String;
    /// Final Formatting
    fn close(&mut self) -> String;
}

pub struct HTMLFormatter {
    out_doc: String,
}

impl HTMLFormatter {
    fn format_style(format: TextFormat) -> String {
        format!("{:?}", {
            ""
        })
    }
}

impl Formatter for HTMLFormatter {
    fn new() -> HTMLFormatter {
        HTMLFormatter { out_doc: String::from("") }
    }

    fn format_text(&mut self, data: Text) -> String {
        println!("Formatting the Next Text Piece: {:?}", data.data);
        String::from(format!(""))
    }

    fn new_line(&mut self) -> String {
        String::from(format!(""))
    }

    fn close(&mut self) -> String {
        self.out_doc = format!("\
        <html>\
            <body>
                {:?}
            </body>
        </html>", self.out_doc);

        self.out_doc.to_owned()
    }
}
