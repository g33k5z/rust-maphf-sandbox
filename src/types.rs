pub use market_data_source::TrendDirection;

/// Tracks the state between chained market segments for continuity and reproducibility.
pub struct MarketState {
    pub last_price: f64,
    pub last_timestamp_ms: i64,
    pub base_seed: u64,
    pub segment_count: u64,
}

impl MarketState {
    pub fn new(initial_price: f64, start_timestamp_ms: i64, base_seed: u64) -> Self {
        Self {
            last_price: initial_price,
            last_timestamp_ms: start_timestamp_ms,
            base_seed,
            segment_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketTheme {
    Bullish,
    Bearish,
    Sideways,
    FlashCrash, // V-Shape
    Correction, // U-Shape
}
