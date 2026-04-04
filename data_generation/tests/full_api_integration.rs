//! # Full Public API Integration Test
//!
//! This test exercises the entire market data generation pipeline from a user's perspective.
//! It follows the end-to-end flow:
//! 1. Configuration via `ScenarioBuilder`.
//! 2. State initialization via `MarketState`.
//! 3. Generation via `MarkovGenerator`.
//! 4. Serialization via `io` module.
//!
//! This ensures that all public-facing components work together seamlessly.

use rust_maphf_sandbox::io::{save_as_bin, save_as_csv, save_as_npz};
use rust_maphf_sandbox::{MarketState, MarketTheme, MarkovGenerator, ScenarioBuilder};
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;

#[test]
fn test_end_to_end_pipeline() {
    let output_dir = "output/test_full_api_integration";
    create_dir_all(output_dir).unwrap();

    let total_ticks = 50_000;
    let tick_size = 0.25;
    let price_scale = 100.0;
    let seed = 12345;

    // 1. Build a complex scenario using the public Builder API
    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(seed)
        .segment_range(1000, 5000)
        .add_theme(MarketTheme::Bullish, 0.2)
        .add_theme(MarketTheme::Sideways, 0.4)
        .add_theme(MarketTheme::FlashCrash, 0.2)
        .add_theme(MarketTheme::Correction, 0.2)
        .build()
        .expect("Failed to build scenario");

    // 2. Initialize the public MarketState at a specific date
    let mut state = MarketState::at_date(2023, 1, 1, 1000.0, seed);

    // 3. Execute the public MarkovGenerator
    let generator = MarkovGenerator::new(tick_size, price_scale);
    let events = generator.generate(&scenario, &mut state);

    // Assertions on generated data
    assert_eq!(events.len(), total_ticks, "Generated tick count mismatch");

    // Verify time continuity across the entire generated set
    for i in 0..events.len() - 1 {
        let current_ts = events[i].exch_ts / 1_000_000;
        let next_ts = events[i + 1].exch_ts / 1_000_000;
        assert_eq!(
            next_ts,
            current_ts + 1,
            "Timestamp continuity break at index {}",
            i
        );
    }

    // 4. Save using all public serializers
    let csv_path = format!("{}/market.csv", output_dir);
    let npz_path = format!("{}/market.npz", output_dir);
    let bin_path = format!("{}/market.bin", output_dir);

    save_as_csv(&events, &csv_path).expect("Failed to save CSV");
    save_as_npz(&events, &npz_path).expect("Failed to save NPZ");
    save_as_bin(&events, &bin_path).expect("Failed to save BIN");

    // Verify files were actually created
    assert!(Path::new(&csv_path).exists(), "CSV file missing");
    assert!(Path::new(&npz_path).exists(), "NPZ file missing");
    assert!(Path::new(&bin_path).exists(), "BIN file missing");

    // Clean up
    remove_dir_all(output_dir).unwrap();
}
