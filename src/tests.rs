//! # Internal Core Tests
//!
//! This module contains unit tests for the core types and configurations that
//! are not specific to the generator engine.

use crate::types::{MarketState, MarketTheme};

/// Verifies that `MarketState::new` correctly assigns all initialization values.
#[test]
fn test_market_state_initialization() {
    let price = 5000.0;
    let ts = 1672531200000;
    let seed = 42;
    let state = MarketState::new(price, ts, seed);

    assert_eq!(state.last_price, price);
    assert_eq!(state.last_timestamp_ms, ts);
    assert_eq!(state.base_seed, seed);
    assert_eq!(state.segment_count, 0);
}

/// Verifies that `MarketState::at_date` correctly converts a date to the correct epoch milliseconds.
#[test]
fn test_market_state_at_date() {
    let price = 5000.0;
    let seed = 42;
    let state = MarketState::at_date(2023, 1, 1, price, seed);

    assert_eq!(state.last_price, price);
    assert_eq!(state.last_timestamp_ms, 1672531200000);
}

/// Ensures that different `MarketTheme` variants are not considered equal.
#[test]
fn test_market_theme_uniqueness() {
    let t1 = MarketTheme::Bullish;
    let t2 = MarketTheme::Bearish;
    assert_ne!(t1, t2);
}
