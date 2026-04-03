use hftbacktest::types::{Bot, Event};
use rust_maphf_sandbox::BacktestSessionBuilder;

#[test]
fn test_hft_backtest_build_from_events() {
    let events = vec![
        Event {
            ev: 1, // some event code (e.g., trade or depth)
            exch_ts: 1000,
            local_ts: 1001,
            px: 5000.0,
            qty: 1.0,
            order_id: 0,
            ival: 0,
            fval: 0.0,
        },
        Event {
            ev: 1,
            exch_ts: 2000,
            local_ts: 2001,
            px: 5000.25,
            qty: 1.0,
            order_id: 0,
            ival: 0,
            fval: 0.0,
        },
    ];

    let backtest = BacktestSessionBuilder::new()
        .tick_size(0.25)
        .lot_size(1.0)
        .latency_ns(1000)
        .load_events(&events)
        .build();

    assert!(
        backtest.is_ok(),
        "Failed to build backtest: {:?}",
        backtest.err()
    );

    let mut backtest = backtest.unwrap();

    // Advance time - 5000ns should cover our 2000ns exchange ts
    let result = backtest.elapse(5000);
    assert!(
        result.is_ok(),
        "Failed to run simulation: {:?}",
        result.err()
    );
}

#[test]
fn test_hft_backtest_with_large_contract_size() {
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

    let backtest = BacktestSessionBuilder::new()
        .contract_size(25.0) // Like ES instead of MES
        .load_events(&events)
        .build();

    assert!(backtest.is_ok());
}
