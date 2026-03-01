use std::io::{self, Write};

pub struct BitBuilder<S: Write> {
    current_byte: u8,
    current_bit_index: u8,
    stream: S,
}

impl<S: Write> BitBuilder<S> {
    pub fn new(stream: S) -> Self {
        Self {
            current_byte: 0,
            current_bit_index: 0,
            stream,
        }
    }
    pub fn write_bit(&mut self, bit: bool) -> Result<(), io::Error> {
        self.current_byte |= (bit as u8) << self.current_bit_index;
        self.current_bit_index += 1;
        if self.current_bit_index >= 8 {
            let buf = [self.current_byte];
            self.stream.write_all(&buf)?;
            self.current_byte = 0;
            self.current_bit_index = 0;
        }
        Ok(())
    }
    pub fn flush(mut self) -> Result<(), io::Error> {
        if self.current_bit_index == 0 {
            return Ok(());
        }
        let buf = [self.current_byte];
        self.stream.write_all(&buf)
    }
}
