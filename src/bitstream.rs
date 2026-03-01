use std::io::{self, Read};

pub struct Bitstream<S: Read> {
    current_byte: u8,
    current_bit_idx: u8,
    stream: S,
}
impl<S: Read> Bitstream<S> {
    pub fn new(stream: S) -> Self {
        Self {
            current_byte: 0,
            current_bit_idx: 8,
            stream,
        }
    }
    pub fn get_next_bit(&mut self) -> Result<bool, io::Error> {
        if self.current_bit_idx >= 8 {
            let mut byte_buf = [0; 1];
            self.stream.read_exact(&mut byte_buf)?;
            self.current_byte = byte_buf[0];
            self.current_bit_idx = 0;
        }

        let res = (self.current_byte >> self.current_bit_idx) & 1 == 1;
        self.current_bit_idx += 1;
        Ok(res)
    }
}
