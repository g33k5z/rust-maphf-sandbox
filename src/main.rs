use rust_maphf_sandbox::generator::{MarketTheme, MarkovGenerator, ScenarioBuilder};
use rust_maphf_sandbox::io::{save_as_bin, save_as_csv, save_as_npz};
use rust_maphf_sandbox::types::MarketState;
use std::fs::create_dir_all;

fn main() -> std::io::Result<()> {
    create_dir_all("output/data")?;

    let tick_size = 0.25;
    let price_scale = 1.0;
    let total_ticks = 1_000_000;

    let scenario = ScenarioBuilder::new(total_ticks)
        .seed(42)
        .segment_range(5_000, 20_000)
        .add_theme(MarketTheme::Bullish, 0.25) // 25% Bullish
        .add_theme(MarketTheme::Sideways, 0.50) // 50% Sideways
        .add_theme(MarketTheme::FlashCrash, 0.10) // 10% V-Shape Recovery
        .add_theme(MarketTheme::Correction, 0.15) // 15% U-Shape Recovery
        .build()
        .expect("Failed to build scenario");

    // Continuity state: $5000 price, starting at Unix epoch (Jan 1, 2023)
    let mut state = MarketState::new(5000.0, 1672531200000, scenario.seed);

    // Budgeted Markov Engine
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
