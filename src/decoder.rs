use std::hash::Hash;
use std::fmt::Debug;

use crate::bitmanipulator::BitManipulator;
use crate::models::modeltrait::ModelTrait;
use crate::models::standard::Standard;
use crate::range::Range;

pub struct ArithmeticDecoder<T: 'static> {
    model: Box<dyn ModelTrait<T>>,
    bitmanipulator: BitManipulator,
    range: Range,
}

impl<T: Eq + Hash + Clone + Debug + Default + 'static> Default for ArithmeticDecoder<T> {
    /// Create a new ArithmeticEncoder with default values
    /// 
    /// - model: Standard model
    /// - bitmanipulator: BitManipulator
    /// - pending: 0
    /// - range: Range with a precision of 32 bits
    fn default() -> Self {
        Self {            
            model: Box::new(Standard::default()),
            bitmanipulator: BitManipulator::new(),
            range: Range::new(32),
        }
    }
}

impl<T: Eq + Hash + Clone + Debug + Default + 'static> ArithmeticDecoder<T> {

    /// Encode a vector of data using arithmetic coding
    pub fn decode(&mut self) -> Vec<T> {
        let mut decoded_data = Vec::new();
        let mut decoding_buffer = 0;
        let mut cumulated_length_so_far = 0;
        for i in 1..=self.range.precision as u64 {
            decoding_buffer += (self.bitmanipulator.read_next_bit() as u64) << (self.range.precision - i as u8);
        }

        loop {
            // iterate over model intervals
            let (intervals, cumulative_frequency, length) = self.model.model();

            for (key, &(symbol_low, symbol_high)) in intervals.iter() {
                let (temp_low, temp_high) = self.range.calculate_range(symbol_low, symbol_high, cumulative_frequency);
                if temp_low <= decoding_buffer && decoding_buffer < temp_high {
                    decoded_data.push(key.clone());
                    self.range.update_range(symbol_low, symbol_high, cumulative_frequency);
                    cumulated_length_so_far += 1;
                    if cumulated_length_so_far == length {
                        return decoded_data;
                    } else if cumulated_length_so_far > length {
                        panic!("Error: cumulated_length_so_far > length");
                    }
                    break;
                }
            }

            loop {
                if self.range.is_bottom_half() {
                    self.range.scale_bottom_half();
                    decoding_buffer = 2 * decoding_buffer + self.bitmanipulator.read_next_bit() as u64;
                } else if self.range.is_above_half() {
                    self.range.scale_above_half();
                    decoding_buffer = (decoding_buffer - self.range.half) * 2 + self.bitmanipulator.read_next_bit() as u64;
                } else if self.range.is_middle_half() {
                    self.range.scale_middle_half();
                    decoding_buffer = (decoding_buffer - self.range.quarter) * 2 + self.bitmanipulator.read_next_bit() as u64;
                } else {
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arithmetic_encoder_standard_model() {
        let excpected_data: Vec<u8> = vec![1, 1, 255, 255, 255, 3, 3, 4, 5];

        let mut decoder = ArithmeticDecoder::default();

        let mut model = Standard::default();
        model.intervals.insert(1,(0, 2));
        model.intervals.insert(255,(2, 5));
        model.intervals.insert(3,(5, 7));
        model.intervals.insert(4, (7, 8));
        model.intervals.insert(5, (8, 9));
        model.cumulative_frequency = 9;
        model.length = 9;
        decoder.model = Box::new(model);

        decoder.bitmanipulator.output = vec![4, 101, 104];

        let decoded_data: Vec<u8> = decoder.decode();
        // let bits = encoder.bitmanipulator.output;
        assert_eq!(excpected_data, decoded_data);
    }
}