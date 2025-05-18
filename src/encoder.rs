use std::hash::Hash;
use std::fmt::Debug;


use crate::bitmanipulator::BitManipulator;
use crate::models::modeltrait::ModelTrait;
use crate::models::standard::Standard;
use crate::range::Range;

pub struct ArithmeticEncoder<T: 'static> {
    model: Box<dyn ModelTrait<T>>,
    bitmanipulator: BitManipulator,
    pending: u64,
    range: Range,
}

impl<T: Eq + Hash + Clone + Debug + Default + 'static> Default for ArithmeticEncoder<T> {
    /// Create a new ArithmeticEncoder with default values
    /// 
    /// - model: Standard model
    /// - bitmanipulator: BitManipulator
    /// - pending: 0
    /// - range: Range with a precision of 32 bits
    fn default() -> Self {
        Self {            
            model: Box::new(Standard::default()),
            bitmanipulator: BitManipulator::default(),
            pending: 0,
            range: Range::new(32),
        }
    }
}

impl<T: Eq + Hash + Clone + Debug + Default + 'static> ArithmeticEncoder<T> {

    /// Write a bit to the bitstream and handle pending bits
    fn write(&mut self, bit: u8) {
        self.bitmanipulator.write_bit(bit);
        while self.pending > 0 {
            self.bitmanipulator.write_bit(bit ^ 1);
            self.pending -= 1;
        }
    }

    /// Encode a vector of data using arithmetic coding
    pub fn encode(&mut self, data: Vec<T>) {
        let mut mutable_data = data.clone();
        
        for symbol in &data {
            let (intervals, cumulative_frequency) = self.model.update_encode(&mutable_data);
            let (symbol_low, symbol_high) = intervals.get(symbol).unwrap();

            self.range.update_range(*symbol_low, *symbol_high, cumulative_frequency);
            loop {
                if self.range.is_bottom_half() {
                    self.write(0);
                    self.range.scale_bottom_half();
                } else if self.range.is_above_half() {
                    self.write(1);
                    self.range.scale_above_half();
                } else if self.range.is_middle_half() {
                    self.pending += 1;
                    self.range.scale_middle_half();
                } else {
                    break;
                }
            }
            mutable_data.remove(0);
        }
        self.pending += 1;
        if self.range.is_above_quarter() {
            self.write(1);
        } else {
            self.write(0);
        }
        self.bitmanipulator.flush();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arithmetic_encoder_standard_model() {
        let data: Vec<u8> = vec![1, 1, 255, 255, 255, 3, 3, 4, 5];
        let mut encoder = ArithmeticEncoder::default();
        let mut model = Standard::default();
        model.intervals.insert(1,(0, 2));
        model.intervals.insert(255,(2, 5));
        model.intervals.insert(3,(5, 7));
        model.intervals.insert(4, (7, 8));
        model.intervals.insert(5, (8, 9));
        model.cumulative_frequency = 9;
        model.length = 9;
        encoder.model = Box::new(model);
        encoder.encode(data);
        let bits = encoder.bitmanipulator.output;
        let expected_bits = vec![4, 101, 104];
        assert_eq!(bits, expected_bits);
    }
}