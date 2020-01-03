use crate::content::{Text, TextFormat, Image};
use crate::printer::{Justification, Underline};

pub trait Formatter {
    fn new() -> Self;
    fn format_text(&mut self, data: Text);
    fn format_image(&mut self, data: Image);
    fn new_line(&mut self);
    /// Final Formatting
    fn close(&mut self) -> String;
}

pub struct HTMLFormatter {
    out_doc: String,
    next_margin: u16,
    scaled: bool,
}

impl HTMLFormatter {
    fn format_style(&mut self, format: TextFormat) -> String {
        let magnify = format.width_mag != 1 || format.height_mag != 1;
        self.scaled = if magnify { true } else { false };

        let style = format!("style=\"{}{}{}{}{}{}\"",
            {
                if format.underline != Underline::None {
                    format!("text-decoration: underline;")
                } else { "".to_string() }
            },
            { if format.bold { "font-weight: bold;" } else { "" } },
            {
                if format.justification != Justification::Left {
                    format!("text-align: {:?};", format.justification).to_ascii_lowercase()
                } else { "".to_string() }
            },
            { if magnify { format!("transform: scale({}, {});", format.width_mag, format.height_mag)} else { "".to_string() } },
            { if (format.justification != Justification::Center) && magnify { "transform-origin: 0% 0%;"  } else { "" } },
            { if self.next_margin != 0 { format!("margin-top: {}px;", self.next_margin) } else { "".to_string() } }
        );

        if style.contains("\"\"") { String::new() } else { style }
    }
}

impl Formatter for HTMLFormatter {
    fn new() -> HTMLFormatter {
        HTMLFormatter {
            out_doc: String::from(""),
            scaled: false,
            next_margin: 0
        }
    }

    /// Add new text to the document. Text is placed inside a new span and div element.
    fn format_text(&mut self, mut data: Text) {
        let mut text: Vec<u8> = Vec::new();
        for (idx, item) in data.data.iter().enumerate() {
            // If the tab is not on the index mod 8
            if !(idx % 8 == 0 && *item == 0x09) {
                text.push(*item);
            }
        }

        let content = String::from(format!("<span {}>{}</span>",
            self.format_style(data.format).as_str(), String::from_utf8(text).expect("Invalid ASCII")));

        self.out_doc.push_str(content.as_str());
        if self.scaled {
            self.out_doc.push_str("<br />");
        }

        self.next_margin = 0;
    }

    fn format_image(&mut self, data: Image) {
//        let image = format!("<img src=""></img>");
    }

    /// Create a new line character and adds it to the document
    fn new_line(&mut self) {
        self.next_margin += 5;
    }

    fn close(&mut self) -> String {
        self.out_doc = format!("
        <!DOCTYPE html>
        <style>
            #all {{
                white-space:pre;
                font-family: monospace;
            }}

            span {{
                display: block;
            }}
        </style>
        <html id=\"all\">
            <body><div style=\"display: inline-block; padding: 10px\">{}</div></body>
        </html>", self.out_doc);

        self.out_doc.to_owned()
    }
}
