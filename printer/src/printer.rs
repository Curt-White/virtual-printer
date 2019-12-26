pub enum Mode {
    Page,
    Standard
}

pub enum Justification {
    Left,
    Center,
    Right
}

pub struct Printer {
    text_buffer:   Vec<u8>,
    image_buffer:  Vec<u8>,
    // Formatting Modes
    justification: Justification,
    bold: bool,
    double_height: bool,
    double_width: bool,
    underline: bool,
}

impl Printer {
    fn set_justification(&mut self, opt: Justification) {
        self.justification = opt;
    }
}