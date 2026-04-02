use hftbacktest::types::{Event, BUY_EVENT, EXCH_EVENT, LOCAL_EVENT, SELL_EVENT, TRADE_EVENT};
use market_data_source::{ConfigBuilder, MarketDataGenerator, Tick, TrendDirection};
use rust_decimal::prelude::*;
use std::fs::File;
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

/// Configures a generator for Micro E-mini S&P 500 (MES)
fn setup_mes_generator(
    initial_price: f64,
    volatility: f64,
    trend: f64,
    direction: TrendDirection,
    seed: u64,
) -> MarketDataGenerator {
    let config = ConfigBuilder::new()
        .starting_price_f64(initial_price)
        .volatility_f64(volatility)
        .trend_f64(direction, trend)
        .seed(seed)
        .build()
        .expect("Failed to build config");

    MarketDataGenerator::with_config(config).expect("Failed to create generator")
}

/// Rounds a price to the nearest tick size
fn round_to_tick(price: f64, tick_size: f64) -> f64 {
    (price / tick_size).round() * tick_size
}

/// Maps synthetic ticks to hftbacktest Event schema
fn map_ticks_to_events(ticks: Vec<Tick>, tick_size: f64, price_scale: f64) -> Vec<Event> {
    let mut last_price = 0.0;

    ticks
        .into_iter()
        .map(|t| {
            let current_price = t.price.to_f64().unwrap();
            let rounded_px = round_to_tick(current_price, tick_size);

            // Determine side based on price movement for the mock data
            let side_flag = if rounded_px >= last_price {
                BUY_EVENT
            } else {
                SELL_EVENT
            };
            last_price = rounded_px;

            Event {
                ev: TRADE_EVENT | EXCH_EVENT | LOCAL_EVENT | side_flag,
                exch_ts: t.timestamp * 1_000_000,        // ms to ns
                local_ts: (t.timestamp + 1) * 1_000_000, // 1ms simulated latency
                px: rounded_px * price_scale,
                qty: t.volume.value as f64,
                order_id: 0,
                ival: 0,
                fval: 0.0,
            }
        })
        .collect()
}

/// Saves the events to a .npz file compatible with hftbacktest
fn save_as_npz(events: &[Event], filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file("data.npy", options)?;

    // NPY Header for the structured array
    let header = format!(
        "{{'descr': [('ev', '<u8'), ('exch_ts', '<i8'), ('local_ts', '<i8'), ('px', '<f8'), ('qty', '<f8'), ('order_id', '<u8'), ('ival', '<i8'), ('fval', '<f8')], 'fortran_order': False, 'shape': ({},)}}",
        events.len()
    );

    let mut header_bytes = header.as_bytes().to_vec();
    // Padding for 64-byte alignment (including prefix and length)
    let padding_len = 64 - ((10 + header_bytes.len() + 1) % 64);
    header_bytes.extend(std::iter::repeat(b' ').take(padding_len));
    header_bytes.push(b'\n');

    let header_len = header_bytes.len() as u16;

    // Magic string and version
    zip.write_all(b"\x93NUMPY")?;
    zip.write_all(&[1, 0])?; // Version 1.0
    zip.write_all(&header_len.to_le_bytes())?;
    zip.write_all(&header_bytes)?;

    // Data
    for ev in events {
        zip.write_all(&ev.ev.to_le_bytes())?;
        zip.write_all(&ev.exch_ts.to_le_bytes())?;
        zip.write_all(&ev.local_ts.to_le_bytes())?;
        zip.write_all(&ev.px.to_le_bytes())?;
        zip.write_all(&ev.qty.to_le_bytes())?;
        zip.write_all(&ev.order_id.to_le_bytes())?;
        zip.write_all(&ev.ival.to_le_bytes())?;
        zip.write_all(&ev.fval.to_le_bytes())?;
    }

    zip.finish()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("output/data")?;
    let tick_size = 0.25;
    let price_scale = 100.0; // Scaled for precision in hftbacktest

    println!("Generating normal MES market data...");
    let mut generator = setup_mes_generator(5000.0, 0.005, 0.0001, TrendDirection::Bullish, 42);
    let ticks = generator.generate_ticks(100000);
    let events = map_ticks_to_events(ticks, tick_size, price_scale);
    save_as_npz(&events, "output/data/mes_normal.npz")?;
    println!("Saved 1000 ticks to mes_normal.npz");

    println!("Generating Flash Crash stress scenario...");
    // Flash Crash: High volatility (5%), Sharp downward trend
    let mut stress_gen = setup_mes_generator(5000.0, 0.05, 0.01, TrendDirection::Bearish, 99);
    let stress_ticks = stress_gen.generate_ticks(50000);
    let stress_events = map_ticks_to_events(stress_ticks, tick_size, price_scale);
    save_as_npz(&stress_events, "output/data/mes_flash_crash.npz")?;
    println!("Saved 500 stress ticks to mes_flash_crash.npz");

    Ok(())
}
