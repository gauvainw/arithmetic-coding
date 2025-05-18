#[derive(Default)]
pub struct BitManipulator {
    pub buffer: u8,
    pub buffer_size: u8,
    pub output: Vec<u8>,
    pub current_bit: usize,
}

impl BitManipulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_bit(&mut self, bit: u8) {
        self.buffer <<= 1;
        self.buffer |= bit;
        self.buffer_size += 1;

        if self.buffer_size == 8 {
            self.output.push(self.buffer);
            self.buffer = 0;
            self.buffer_size = 0;
        }
    }

    pub fn flush(&mut self) {
        if self.buffer_size > 0 {
            self.buffer <<= 8 - self.buffer_size;
            self.output.push(self.buffer);
            self.buffer = 0;
            self.buffer_size = 0;
        }
    }

    fn read_bit_at_index(&mut self, index_of_bit: usize) -> u8 {
        if index_of_bit >= self.output.len() * 8 {
            panic!("Index out of bounds");
        }
        let byte_index = index_of_bit / 8;
        let bit_index = index_of_bit % 8;
        (self.output[byte_index] >> (7 - bit_index)) & 1
    }

    pub fn read_next_bit(&mut self) -> u8 {
        if self.current_bit >= self.output.len() * 8 {
            return 0;
        }
        let bit = self.read_bit_at_index(self.current_bit);
        self.current_bit += 1;
        bit
    }

}