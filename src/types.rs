//! # Core Types and Models
//!
//! This module defines the common language used across the library to track market state
//! and represent high-level thematic behaviors.

pub use market_data_source::TrendDirection;

/// # Market Continuity State
///
/// Tracks the last generated price, timestamp, and seed across chained segments.
/// This state ensures that each segment seamlessly continues from where the previous one
/// left off, preventing artificial price gaps and ensuring time-series integrity.
pub struct MarketState {
    /// The last traded price in absolute dollars.
    pub last_price: f64,
    /// The last exchange timestamp in milliseconds.
    pub last_timestamp_ms: i64,
    /// The global seed for the entire simulation to ensure reproducibility.
    pub base_seed: u64,
    /// Counter for the number of segments generated, used to derive sub-seeds.
    pub segment_count: u64,
}

impl MarketState {
    /// Initializes a new market state for the start of a simulation.
    ///
    /// # Parameters
    /// - `initial_price`: The starting price (e.g., 5000.0).
    /// - `start_timestamp_ms`: The starting time in Unix epoch milliseconds.
    /// - `base_seed`: The root seed for all random operations in the scenario.
    pub fn new(initial_price: f64, start_timestamp_ms: i64, base_seed: u64) -> Self {
        Self {
            last_price: initial_price,
            last_timestamp_ms: start_timestamp_ms,
            base_seed,
            segment_count: 0,
        }
    }
}

/// # Market Themes
///
/// High-level market behaviors that the `ScenarioBuilder` uses to construct a market story.
/// These themes map to one or more internal `RegimePhase` definitions in the generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketTheme {
    /// Low-to-medium volatility with a steady positive drift.
    Bullish,
    /// Medium volatility with a steady negative drift.
    Bearish,
    /// Low volatility with zero trend, simulating mean-reversion.
    Sideways,
    /// A sharp, high-volatility drop immediately followed by a rapid recovery (V-Shape).
    FlashCrash,
    /// A moderate-high volatility sell-off followed by a stagnant bottom and gradual recovery (U-Shape).
    Correction,
}
