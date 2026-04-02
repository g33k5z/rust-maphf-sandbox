//! # IO Integration Tests
//!
//! Validates the creation of CSV, NPZ, and BIN files by using the public library API.
//! This ensures that the serializers handle data correctly and interact with the
//! filesystem as expected in a real-world usage scenario.

use rust_maphf_sandbox::generator::{MarketTheme, MarkovGenerator, ScenarioBuilder};
use rust_maphf_sandbox::io::{save_as_bin, save_as_csv, save_as_npz};
use rust_maphf_sandbox::types::MarketState;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;

/// Verifies that all IO serializers (CSV, NPZ, BIN) successfully create files on disk.
#[test]
fn test_io_serializers_creation() {
    let output_dir = "output/test_integration_data";
    create_dir_all(output_dir).unwrap();

    let scenario = ScenarioBuilder::new(100)
        .add_theme(MarketTheme::Sideways, 1.0)
        .build()
        .unwrap();

    let mut state = MarketState::new(5000.0, 1000000, 42);
    let generator = MarkovGenerator::new(0.25, 1.0);
    let events = generator.generate(&scenario, &mut state);

    let csv_file = format!("{}/test.csv", output_dir);
    let npz_file = format!("{}/test.npz", output_dir);
    let bin_file = format!("{}/test.bin", output_dir);

    // Run serializers
    save_as_csv(&events, &csv_file).expect("Failed to save CSV");
    save_as_npz(&events, &npz_file).expect("Failed to save NPZ");
    save_as_bin(&events, &bin_file).expect("Failed to save BIN");

    // Verify file existence
    assert!(Path::new(&csv_file).exists());
    assert!(Path::new(&npz_file).exists());
    assert!(Path::new(&bin_file).exists());

    // Clean up
    remove_dir_all(output_dir).unwrap();
}
