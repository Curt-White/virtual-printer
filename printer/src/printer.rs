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
    justification: Justification,
}

impl Printer {
    fn set_justification(&mut self, opt: Justification) {
        self.justification = opt;
    }
}