use hftbacktest::backtest::Backtest;
use hftbacktest::depth::HashMapMarketDepth;
use hftbacktest::depth::MarketDepth;
use hftbacktest::prelude::ElapseResult;
use hftbacktest::types::{Bot, Event};

pub fn print_generation_start(total_ticks: usize) {
    println!(
        "Generating {} ticks using Theme-based Budgeted Markov approach...",
        total_ticks
    );
}

pub fn print_event_preview(events: &[Event]) {
    println!("Saving data to multiple formats (CSV, NPZ, BIN)...");
    for i in 0..5.min(events.len()) {
        println!(
            " - Event {}: px={:.2}, exch_ts={}, local_ts={}",
            i, events[i].px, events[i].exch_ts, events[i].local_ts
        );
    }

    if events.len() > 5 {
        println!("...and the last 5 events:");
        for i in (events.len() - 5)..events.len() {
            println!(
                " - Event {}: px={:.2}, exch_ts={}, local_ts={}",
                i, events[i].px, events[i].exch_ts, events[i].local_ts
            );
        }
    }
}

pub fn print_generation_summary(count: usize) {
    println!("Successfully generated and saved {} total events.", count);
}

pub fn print_backtest_init() {
    println!("\nInitializing hftbacktest session using Fluent API (NPZ)...");
}

pub fn print_backtest_start() {
    println!("Running backtest simulation (Trade-only)...");
}

pub fn print_backtest_complete(result: ElapseResult) {
    println!("Backtest complete with result: {:?}", result);
}

pub fn print_backtest_results(backtest: &Backtest<HashMapMarketDepth>) {
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
            println!("Total Trades: {}", backtest.last_trades(0).len());
        } else {
            println!("No trades processed.");
        }
    } else {
        println!("Final Best Bid: {:.2}", last_price);
    }
}

pub fn display_depth(depth: &HashMapMarketDepth, levels: usize) {
    println!("  Asks:");
    let best_ask_tick = depth.best_ask_tick;
    let best_bid_tick = depth.best_bid_tick;

    if best_ask_tick < i64::MAX {
        for i in (0..levels).rev() {
            let tick = best_ask_tick + i as i64;
            if let Some(&qty) = depth.ask_depth.get(&tick) {
                println!("    {:<10.2}: {:>10.2}", tick as f64 * depth.tick_size, qty);
            }
        }
    } else {
        println!("    (No Ask Liquidity)");
    }

    let best_bid = depth.best_bid();
    let best_ask = depth.best_ask();
    let mid_price = if !best_bid.is_nan() && !best_ask.is_nan() {
        (best_bid + best_ask) / 2.0
    } else {
        f64::NAN
    };
    println!("  ------ Mid: {:.2} ------", mid_price);
    println!("  Bids:");

    if best_bid_tick > i64::MIN {
        for i in 0..levels {
            let tick = best_bid_tick - i as i64;
            if let Some(&qty) = depth.bid_depth.get(&tick) {
                println!("    {:<10.2}: {:>10.2}", tick as f64 * depth.tick_size, qty);
            }
        }
    } else {
        println!("    (No Bid Liquidity)");
    }
}
