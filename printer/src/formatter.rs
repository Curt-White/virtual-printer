use crate::content::{Text, TextFormat};
use crate::printer::Justification;

pub trait Formatter {
    fn new() -> Self;
    fn format_text(&mut self, data: Text);
    fn new_line(&mut self);
    /// Final Formatting
    fn close(&mut self) -> String;
}

pub struct HTMLFormatter {
    out_doc: String,
}

impl HTMLFormatter {
    fn format_style(format: TextFormat) -> String {
        format!("style=\"{} {} {} {} {}\"",
            { if format.underline { "text-decoration: underline;" } else { "" } },
            { if format.bold { "font-weight: bold;" } else { "" } },
            { format!("text-align: {:?};", format.justification).to_ascii_lowercase() },
            { format!("transform: scale({}, {});", format.width_mag, format.height_mag) },
            { if format.justification != Justification::Center { "transform-origin: 0% 0%;"  } else { "" } },
        )
    }
}

impl Formatter for HTMLFormatter {
    fn new() -> HTMLFormatter {
        HTMLFormatter { out_doc: String::from("") }
    }

    fn format_text(&mut self, data: Text) {
        let content = String::from(format!("<div {}><span>{}</span></div>",
            HTMLFormatter::format_style(data.format).as_str(), data.data));
        self.out_doc.push_str(content.as_str());
    }

    fn new_line(&mut self) {
        self.out_doc.push_str("<br />")
    }

    fn close(&mut self) -> String {
        self.out_doc = format!("
        <!DOCTYPE html>
        <html style=\"white-space:pre; font-family: monospace;\">
            <body><div style=\"display: inline-block; padding: 10px\">{}</div></body>
        </html>", self.out_doc);

        self.out_doc.to_owned()
    }
}
