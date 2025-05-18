pub struct Range {
    pub low: u64,
    pub high: u64,
    pub precision: u8,
    pub half: u64,
    pub quarter: u64,
    pub three_quarters: u64,
}
impl Range {
    pub fn new(precision: u8) -> Self {
        let whole: u64 = 1 << precision;
        let half: u64 = 1 << (precision - 1);
        let quarter: u64 = 1 << (precision - 2);
        let three_quarters: u64 = 3 << (precision - 2);
        Self {
            low: 0,
            high: whole,
            precision,
            half,
            quarter,
            three_quarters,
        }
    }

    pub fn update_range(&mut self, symbol_low: u64, symbol_high: u64, cumulative_frequency: u64) {
        let (low, high) = self.calculate_range(symbol_low, symbol_high, cumulative_frequency);
        self.low = low;
        self.high = high;
    }

    pub fn calculate_range(&self, symbol_low: u64, symbol_high: u64, cumulative_frequency: u64) -> (u64, u64) {
        let range = self.high - self.low;
        let high = self.low + (range * symbol_high) / cumulative_frequency;
        let low = self.low + (range * symbol_low) / cumulative_frequency;
        (low, high)
    }

    pub fn is_bottom_half(&self) -> bool {
        self.high < self.half
    }
    pub fn is_above_half(&self) -> bool {
        self.low >= self.half
    }
    pub fn is_above_quarter(&self) -> bool {
        self.low > self.quarter
    }
    pub fn is_middle_half(&self) -> bool {
        self.low >= self.quarter && self.high < self.three_quarters
    }
    pub fn scale_bottom_half(&mut self) {
        self.low *= 2;
        self.high *= 2;
    }
    pub fn scale_above_half(&mut self) {
        self.low = (self.low - self.half) * 2;
        self.high = (self.high - self.half) * 2;
    }
    pub fn scale_middle_half(&mut self) {
        self.low = (self.low - self.quarter) * 2;
        self.high = (self.high - self.quarter) * 2;
    }
}