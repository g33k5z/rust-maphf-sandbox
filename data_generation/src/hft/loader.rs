//! # Data Loaders for hftbacktest
//!
//! This module provides tools to load synthetic market data into `hftbacktest`
//! memory-aligned formats.

use hftbacktest::backtest::data::{read_npz_file, Data};
use hftbacktest::types::Event;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Loads events from a raw binary file (little-endian).
pub fn load_from_bin<P: AsRef<Path>>(path: P) -> std::io::Result<Data<Event>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut events = Vec::new();

    // Event size is 8 fields * 8 bytes = 64 bytes
    let mut buf = [0u8; 64];

    while reader.read_exact(&mut buf).is_ok() {
        let ev = Event {
            ev: u64::from_le_bytes(buf[0..8].try_into().unwrap()),
            exch_ts: i64::from_le_bytes(buf[8..16].try_into().unwrap()),
            local_ts: i64::from_le_bytes(buf[16..24].try_into().unwrap()),
            px: f64::from_le_bytes(buf[24..32].try_into().unwrap()),
            qty: f64::from_le_bytes(buf[32..40].try_into().unwrap()),
            order_id: u64::from_le_bytes(buf[40..48].try_into().unwrap()),
            ival: i64::from_le_bytes(buf[48..56].try_into().unwrap()),
            fval: f64::from_le_bytes(buf[56..64].try_into().unwrap()),
        };
        events.push(ev);
    }

    if let Some(first) = events.first() {
        println!(
            " - Loader first event: px={:.2}, exch_ts={}, local_ts={}",
            first.px, first.exch_ts, first.local_ts
        );
    }

    Ok(Data::from_data(&events))
}

/// Loads events from an NPZ file using hftbacktest's native reader.
pub fn load_from_npz<P: AsRef<Path>>(path: P) -> std::io::Result<Data<Event>> {
    let path_str = path
        .as_ref()
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))?;

    // We assume the internal file name is 'data.npy' as used in our save_as_npz
    read_npz_file(path_str, "data")
}
