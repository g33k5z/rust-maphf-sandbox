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

use rust_maphf_sandbox::BacktestSessionBuilder;
use rust_maphf_sandbox::generator::{MarketTheme, MarkovGenerator, ScenarioBuilder};
use rust_maphf_sandbox::io::save_as_npz; //save_as_bin, save_as_csv
use rust_maphf_sandbox::types::MarketState;
use std::fs::create_dir_all;

/// Main entry point for the simulation.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("output/data")?;

    let tick_size = 0.25;
    let price_scale = 100.0;
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
    for i in 0..5.min(all_events.len()) {
        println!(
            " - Event {}: px={:.2}, exch_ts={}, local_ts={}",
            i, all_events[i].px, all_events[i].exch_ts, all_events[i].local_ts
        );
    }

    println!("...and the last 5 events:");
    for i in (all_events.len() - 5)..all_events.len() {
        println!(
            " - Event {}: px={:.2}, exch_ts={}, local_ts={}",
            i, all_events[i].px, all_events[i].exch_ts, all_events[i].local_ts
        );
    }

    save_as_npz(&all_events, "output/data/mes_1m_dynamic.npz")?;

    println!(
        "Successfully generated and saved {} total events.",
        all_events.len()
    );

    println!("\nInitializing hftbacktest session using Fluent API (NPZ)...");
    let mut backtest = BacktestSessionBuilder::new()
        .tick_size(tick_size * price_scale)
        .contract_size(5.0 / price_scale)
        .load_events(&all_events) // or from file .load_npz("output/data/mes_1m_dynamic.npz")
        .build()?;

    println!("Running backtest simulation (Trade-only)...");

    use hftbacktest::depth::MarketDepth;
    use hftbacktest::prelude::Bot;
    // Advance backtest until end of data
    let result = backtest.goto_end();
    println!("Backtest complete with result: {:?}", result);

    let last_price = backtest.depth(0).best_bid();
    if last_price.is_nan() {
        if let Some(last_trade) = backtest.last_trades(0).last() {
            println!("Final Last Trade Price: {:.2}", last_trade.px);
            println!(
                "Final Last Trade Price (bits): 0x{:x}",
                last_trade.px.to_bits()
            );
            println!("Final Last Trade Qty: {:.2}", last_trade.qty);
            println!("Final Last Trade ev: 0x{:x}", last_trade.ev);
            println!("Total Tades: {}", backtest.last_trades(0).len());
        } else {
            println!("No trades processed.");
        }
    } else {
        println!("Final Best Bid: {:.2}", last_price);
    }

    Ok(())
}
