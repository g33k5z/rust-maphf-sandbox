//! # Internal Generator Engine Tests
//!
//! This module contains unit tests for the stochastic generator logic,
//! scenario building, and the budgeted Markov process.

use crate::generator::{MarketTheme, MarkovGenerator, ScenarioBuilder};
use crate::types::MarketState;

/// Checks if the `ScenarioBuilder` correctly validates that weights must sum to 1.0.
#[test]
fn test_scenario_builder_validation_weights_sum() {
    // Valid sum (1.0)
    let scenario = ScenarioBuilder::new(100)
        .add_theme(MarketTheme::Bullish, 0.5)
        .add_theme(MarketTheme::Sideways, 0.5)
        .build();
    assert!(scenario.is_ok());

    // Invalid sum (0.9)
    let scenario = ScenarioBuilder::new(100)
        .add_theme(MarketTheme::Bullish, 0.4)
        .add_theme(MarketTheme::Sideways, 0.5)
        .build();
    assert!(scenario.is_err());
}

/// Ensures that a scenario cannot be built without any themes.
#[test]
fn test_scenario_builder_validation_empty_themes() {
    let scenario = ScenarioBuilder::new(100).build();
    assert!(scenario.is_err());
}

/// Verifies that the `MarkovGenerator` produces exactly the requested number of events.
#[test]
fn test_markov_generator_total_ticks() {
    let total_ticks = 10_000;
    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(123)
        .segment_range(500, 1000)
        .add_theme(MarketTheme::Bullish, 0.3)
        .add_theme(MarketTheme::Sideways, 0.7)
        .build()
        .unwrap();

    let mut state = MarketState::new(5000.0, 1000000, 123);
    let generator = MarkovGenerator::new(0.25, 100.0);
    let events = generator.generate(&scenario, &mut state);

    assert_eq!(events.len(), total_ticks);
}

/// Ensures that each event in the series follows a monotonic time increment.
#[test]
fn test_markov_generator_continuity() {
    let total_ticks = 5000;
    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(456)
        .add_theme(MarketTheme::Bullish, 1.0)
        .build()
        .unwrap();

    let initial_price = 5000.0;
    let initial_ts = 1000000;
    let mut state = MarketState::new(initial_price, initial_ts, 456);
    let generator = MarkovGenerator::new(0.25, 1.0);
    let events = generator.generate(&scenario, &mut state);

    // Verify time continuity
    for i in 0..events.len() - 1 {
        let current_ts = events[i].exch_ts / 1_000_000;
        let next_ts = events[i + 1].exch_ts / 1_000_000;
        assert_eq!(next_ts, current_ts + 1);
    }

    // Verify final state updates correctly
    assert_eq!(state.last_timestamp_ms, initial_ts + total_ticks as i64);
}

/// Verifies that the same seed and scenario configuration produce bit-for-bit identical results.
#[test]
fn test_reproducibility() {
    let scenario = ScenarioBuilder::new(1000)
        .seed(789)
        .add_theme(MarketTheme::FlashCrash, 0.5)
        .add_theme(MarketTheme::Correction, 0.5)
        .build()
        .unwrap();

    let mut state1 = MarketState::new(5000.0, 1000000, 789);
    let generator1 = MarkovGenerator::new(0.25, 1.0);
    let events1 = generator1.generate(&scenario, &mut state1);

    let mut state2 = MarketState::new(5000.0, 1000000, 789);
    let generator2 = MarkovGenerator::new(0.25, 1.0);
    let events2 = generator2.generate(&scenario, &mut state2);

    for (e1, e2) in events1.iter().zip(events2.iter()) {
        assert_eq!(e1.px, e2.px);
        assert_eq!(e1.exch_ts, e2.exch_ts);
        assert_eq!(e1.ev, e2.ev);
    }
}
