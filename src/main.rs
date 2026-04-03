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
use rust_maphf_sandbox::types::MarketState;
use rust_maphf_sandbox::ui;
use rust_maphf_sandbox::BacktestSessionBuilder;
use std::fs::create_dir_all;

/// Main entry point for the simulation.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("output/data")?;

    let tick_size = 0.25;
    let price_scale = 100.0;
    let total_ticks = 10_000_000;

    // Defining the Market
    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(67)
        .segment_range(5_000, 1_000_000)
        .add_theme(MarketTheme::Bullish, 0.25)
        .add_theme(MarketTheme::Sideways, 0.50)
        .add_theme(MarketTheme::FlashCrash, 0.10)
        .add_theme(MarketTheme::Correction, 0.15)
        .build()
        .expect("Failed to build scenario");

    // Continuity state
    let mut state = MarketState::at_date(2069, 4, 20, 5000.0, scenario.seed);

    // Budgeted Markov Engine
    let generator = MarkovGenerator::new(tick_size, price_scale);

    ui::print_generation_start(total_ticks);
    let all_events = generator.generate(&scenario, &mut state);

    // ui::print_event_preview(&all_events);
    ui::print_generation_summary(all_events.len());

    ui::print_backtest_init();
    let mut backtest = BacktestSessionBuilder::new()
        .tick_size(tick_size * price_scale)
        .contract_size(5.0 / price_scale)
        .load_events(&all_events)
        .build()?;

    ui::print_backtest_start();

    // Advance backtest until end of data
    let result = backtest.goto_end();
    ui::print_backtest_complete(result.unwrap());

    ui::print_backtest_results(&backtest);

    Ok(())
}
