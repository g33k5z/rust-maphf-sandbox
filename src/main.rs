//! # Market Data Sandbox CLI
//!
//! This is the primary entry point for the market data generation tool.
//! It utilizes the library's `ScenarioBuilder` and `MarkovGenerator` to create
//! large-scale, high-frequency market data in multiple file formats.
//!
//! Example usage:
//! 1. Configure the scenario with themes (Bullish, FlashCrash, etc.).
//! 2. Generate exactly 1,000,000 events based on the desired percentages.
//! 3. Save to output/data/ in CSV, NPZ, and BIN formats.

use rust_maphf_sandbox::generator::{MarketTheme, MarkovGenerator, ScenarioBuilder};
use rust_maphf_sandbox::io::{save_as_bin, save_as_csv, save_as_npz};
use rust_maphf_sandbox::types::MarketState;
use std::fs::create_dir_all;

/// Main entry point for the simulation.
///
/// This function:
/// 1. Initializes the output directory.
/// 2. Defines the simulation parameters (tick size, price scale, total ticks).
/// 3. Builds a `Scenario` with specific market themes and weights.
/// 4. Executes the `MarkovGenerator` to produce a high-frequency time series.
/// 5. Saves the final event list to multiple files.
fn main() -> std::io::Result<()> {
    create_dir_all("output/data")?;

    let tick_size = 0.25;
    let price_scale = 1.0;
    let total_ticks = 1_000_000;

    // Defining the Market
    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(42)
        .segment_range(5_000, 20_000)
        .add_theme(MarketTheme::Bullish, 0.25) // 25% of time is upward drift
        .add_theme(MarketTheme::Sideways, 0.50) // 50% of time is consolidation
        .add_theme(MarketTheme::FlashCrash, 0.10) // 10% of time is spent in sharp V-shapes
        .add_theme(MarketTheme::Correction, 0.15) // 15% of time is spent in U-shape corrections
        .build()
        .expect("Failed to build scenario");

    // Continuity state
    let mut state = MarketState::at_date(2023, 1, 1, 5000.0, scenario.seed);

    // Budgeted Markov Engine:
    // Orchestrates the transitions between themes to meet the desired percentages
    let generator = MarkovGenerator::new(tick_size, price_scale);

    println!(
        "Generating {} ticks using Theme-based Budgeted Markov approach...",
        total_ticks
    );
    let all_events = generator.generate(&scenario, &mut state);

    println!("Saving data to multiple formats (CSV, NPZ, BIN)...");
    save_as_csv(&all_events, "output/data/mes_1m_dynamic.csv")?;
    save_as_npz(&all_events, "output/data/mes_1m_dynamic.npz")?;
    save_as_bin(&all_events, "output/data/mes_1m_dynamic.bin")?;

    println!(
        "Successfully generated and saved {} total events.",
        all_events.len()
    );
    Ok(())
}
