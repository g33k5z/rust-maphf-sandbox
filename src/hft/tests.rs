use super::*;
use hftbacktest::types::Event;

#[test]
fn test_builder_defaults() {
    let builder = BacktestSessionBuilder::new();
    assert_eq!(builder.tick_size, 0.25);
    assert_eq!(builder.lot_size, 1.0);
    assert_eq!(builder.latency, 1_000_000);
    assert_eq!(builder.maker_fee, 0.0);
    assert_eq!(builder.taker_fee, 0.0);
    assert_eq!(builder.contract_size, 5.0);
    assert!(builder.data_path.is_none());
    assert!(!builder.is_npz);
    assert!(builder.events.is_none());
}

#[test]
fn test_builder_custom_config() {
    let builder = BacktestSessionBuilder::new()
        .tick_size(0.01)
        .lot_size(100.0)
        .latency_ns(500_000)
        .fees(0.0001, 0.0002)
        .contract_size(1.0);

    assert_eq!(builder.tick_size, 0.01);
    assert_eq!(builder.lot_size, 100.0);
    assert_eq!(builder.latency, 500_000);
    assert_eq!(builder.maker_fee, 0.0001);
    assert_eq!(builder.taker_fee, 0.0002);
    assert_eq!(builder.contract_size, 1.0);
}

#[test]
fn test_load_events_cow() {
    let events = vec![Event {
        ev: 1,
        exch_ts: 1000,
        local_ts: 1001,
        px: 5000.0,
        qty: 1.0,
        order_id: 0,
        ival: 0,
        fval: 0.0,
    }];

    // Test with borrowed events
    let builder_borrowed = BacktestSessionBuilder::new().load_events(&events);
    assert!(builder_borrowed.events.is_some());
    if let Some(Cow::Borrowed(e)) = builder_borrowed.events {
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].px, 5000.0);
    } else {
        panic!("Expected borrowed events");
    }

    // Test with owned events
    let builder_owned = BacktestSessionBuilder::new().load_events(events.clone());
    assert!(builder_owned.events.is_some());
    if let Some(Cow::Owned(e)) = builder_owned.events {
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].px, 5000.0);
    } else {
        panic!("Expected owned events");
    }
}

#[test]
fn test_load_data_no_source_fails() {
    let builder = BacktestSessionBuilder::new();
    let result = builder.load_data();
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "No data path or events provided"
    );
}
