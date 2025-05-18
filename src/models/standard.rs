use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::modeltrait::ModelTrait;

#[derive(Default)]
pub struct Standard<T> {
    pub intervals: HashMap<T, (u64, u64)>,
    pub cumulative_frequency: u64,
    pub length: u64,
    pub data: Vec<T>,
}

impl<T: Eq + Hash + Clone + Default + Debug> ModelTrait<T> for Standard<T> {
    fn init(&mut self, data: &Vec<T>) -> (&HashMap<T, (u64, u64)>, u64, u64) {
        let mut frequencies: HashMap<T, u64> = HashMap::new();
        for value in data.iter() {
            *frequencies.entry(value.clone()).or_insert(0u64) += 1;
        }
        for (unit, &count) in frequencies.iter() {
            self.intervals.insert(unit.clone(), (self.cumulative_frequency, self.cumulative_frequency + count));
            self.cumulative_frequency += count;
        }
        self.length = data.len() as u64;
        (&self.intervals, self.cumulative_frequency, self.length)
    }

    fn model(&self) -> (&HashMap<T, (u64, u64)>, u64, u64) {
        (&self.intervals, self.cumulative_frequency, self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_func() {
        let data: Vec<u8> = vec![0, 255, 255, 255, 67, 67];
        let mut model = Standard::default();
        
        // Intervals is a hashmap, thus we can't really compare it directly cause the order of the keys is not guaranteed
        // We can only compare the cumulative_frequency and length and (symbol_high - symbol_low) values of the intervals
        let (intervals_ref, cumulative_frequency, length) = model.init(&data);
        let intervals = intervals_ref.clone();

        assert_eq!(model.cumulative_frequency, 6); // 6 occurrences au total
        assert_eq!(model.length, 6); // 6 symbols au total
        assert_eq!(model.intervals.len(), 3); // 3 symboles différents
        assert_eq!(model.intervals.get(&0).unwrap().1 - model.intervals.get(&0).unwrap().0, 1); // 1 occurrence de 0
        assert_eq!(model.intervals.get(&255).unwrap().1 - model.intervals.get(&255).unwrap().0, 3); // 3 occurrences de 255
        assert_eq!(model.intervals.get(&67).unwrap().1 - model.intervals.get(&67).unwrap().0, 2); // 2 occurrences de 67

        assert_eq!(cumulative_frequency, 6); // 6 occurrences au total
        assert_eq!(length, 6); // 6 symbols au total
        assert_eq!(intervals.len(), 3); // 3 symboles différents
        assert_eq!(intervals.get(&0).unwrap().1 - intervals.get(&0).unwrap().0, 1); // 1 occurrence de 0
        assert_eq!(intervals.get(&255).unwrap().1 - intervals.get(&255).unwrap().0, 3); // 3 occurrences de 255
        assert_eq!(intervals.get(&67).unwrap().1 - intervals.get(&67).unwrap().0, 2); // 2 occurrences de 67

        println!("Fréquences: {:?}", model.cumulative_frequency);
        println!("Intervalles: {:?}", model.intervals);
        println!("Longueur: {}", model.length);
    }
}