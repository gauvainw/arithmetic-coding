use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait ModelTrait<T: Eq + Hash + Clone + Default + Debug> {

    /// Init the model based on a vector of data before encoding
    /// Returns the (intervals, cumulative_frequency, length) of the model.
    /// 
    /// intervals is a HashMap where the key is the symbol and the value is a tuple of (symbol_low, symbol_high).
    /// 
    /// cumulative_frequency is usually the sum of the (high - low) values for all symbols
    /// cumulative_frequency is used in the encoder to determine the new range for the symbol.
    /// 
    /// length is the total number of symbols in the (data.len())
    fn init(&mut self, _data: &Vec<T>) -> (&HashMap<T, (u64, u64)>, u64, u64) {
        self.model()
    }

    /// Update the model, when encoding, according to the new vector of data.
    /// Returns the (intervals, cumulative_frequency) of the model based on the new vector of data.
    /// 
    /// intervals is a HashMap where the key is the symbol and the value is a tuple of (symbol_low, symbol_high).
    /// cumulative_frequency is usually the sum of the (high - low) values for all symbols
    /// cumulative_frequency is used in the encoder to determine the new range for the symbol.
    /// 
    /// Note : the model is not updated in the standard implementation.
    fn update_encode(&mut self, _data: &Vec<T>) -> (&HashMap<T, (u64, u64)>, u64) {
        (self.model().0, self.model().1) 
    }

    /// Update the model, when decoding, according to the current decoding data
    /// Returns the (intervals, cumulative_frequency) of the model based on the new vector of data.
    /// 
    /// intervals is a HashMap where the key is the symbol and the value is a tuple of (symbol_low, symbol_high).
    /// cumulative_frequency is usually the sum of the (high - low) values for all symbols
    /// cumulative_frequency is used in the encoder to determine the new range for the symbol.
    /// 
    /// Note : the model is not updated in the standard implementation.
    fn update_decode(&mut self, _data: &Vec<T>) -> (&HashMap<T, (u64, u64)>, u64) {
        (self.model().0, self.model().1)
    }

    /// Return the model of the data at the current state, without updating it.
    fn model(&self) -> (&HashMap<T, (u64, u64)>, u64, u64);
    
    /// Return the total number of symbols in the data.
    /// This is used in the decoder to determine the end of the stream.
    fn message_length(&self) -> u64 {
        self.model().2
    }
}