//! # High-Frequency Backtesting Sandbox
//!
//! This module provides a simplified, fluent API for configuring and running
//! backtests with `hftbacktest`. It wraps the complex generic builders of
//! the underlying engine with sensible defaults for synthetic data.

pub mod loader;
#[cfg(test)]
mod tests;

use crate::hft::loader::{load_from_bin, load_from_npz};
use hftbacktest::backtest::assettype::LinearAsset;
use hftbacktest::backtest::data::{Data, DataSource};
use hftbacktest::backtest::models::{
    CommonFees, ConstantLatency, PowerProbQueueFunc, ProbQueueModel, TradingValueFeeModel,
};
use hftbacktest::backtest::{Asset, Backtest};
use hftbacktest::depth::HashMapMarketDepth;
use hftbacktest::types::Event;
use std::borrow::Cow;
use std::path::PathBuf;

/// A fluent builder for configuring an `hftbacktest` session.
pub struct BacktestSessionBuilder<'a> {
    tick_size: f64,
    lot_size: f64,
    latency: i64,
    maker_fee: f64,
    taker_fee: f64,
    contract_size: f64,
    data_path: Option<PathBuf>,
    is_npz: bool,
    events: Option<Cow<'a, [Event]>>,
}

impl<'a> BacktestSessionBuilder<'a> {
    /// Creates a new builder with default Micro E-mini S&P 500 (MES) settings.
    pub fn new() -> Self {
        Self {
            tick_size: 0.25,
            lot_size: 1.0,
            latency: 0,
            maker_fee: 0.0,
            taker_fee: 0.0,
            contract_size: 5.0, // $5 per point for MES
            data_path: None,
            is_npz: false,
            events: None,
        }
    }

    pub fn tick_size(mut self, tick_size: f64) -> Self {
        self.tick_size = tick_size;
        self
    }

    pub fn lot_size(mut self, lot_size: f64) -> Self {
        self.lot_size = lot_size;
        self
    }

    pub fn latency_ns(mut self, latency_ns: i64) -> Self {
        self.latency = latency_ns;
        self
    }

    pub fn fees(mut self, maker: f64, taker: f64) -> Self {
        self.maker_fee = maker;
        self.taker_fee = taker;
        self
    }

    pub fn contract_size(mut self, contract_size: f64) -> Self {
        self.contract_size = contract_size;
        self
    }

    pub fn load_bin<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.data_path = Some(path.into());
        self.is_npz = false;
        self.events = None;
        self
    }

    pub fn load_npz<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.data_path = Some(path.into());
        self.is_npz = true;
        self.events = None;
        self
    }

    /// Loads events directly from a vector in memory.
    pub fn load_events(mut self, events: impl Into<Cow<'a, [Event]>>) -> Self {
        self.events = Some(events.into());
        self.data_path = None;
        self
    }

    fn load_data(&self) -> Result<Data<Event>, Box<dyn std::error::Error>> {
        if let Some(events) = &self.events {
            return Ok(Data::from_data(events));
        }

        let data_path = self
            .data_path
            .as_ref()
            .ok_or("No data path or events provided")?;

        if !self.is_npz {
            return Ok(load_from_bin(data_path)?);
        }

        Ok(load_from_npz(data_path)?)
    }

    /// Builds a single-asset L2 backtest session.
    pub fn build(self) -> Result<Backtest<HashMapMarketDepth>, Box<dyn std::error::Error>> {
        let data = self.load_data()?;

        // hftbacktest 0.9.4 ConstantLatency::new(entry_latency, response_latency)
        let latency_model = ConstantLatency::new(self.latency, self.latency);
        let asset_type = LinearAsset::new(self.contract_size);
        let fee_model = TradingValueFeeModel::new(CommonFees::new(self.maker_fee, self.taker_fee));

        // PowerProbQueueFunc::new(n) - using 1.0 as a neutral default
        let queue_model = ProbQueueModel::new(PowerProbQueueFunc::new(1.0));

        let tick_size = self.tick_size;
        let lot_size = self.lot_size;

        // Corrected type parameters: ProbQueueModel<P, MD>
        type QM = ProbQueueModel<PowerProbQueueFunc, HashMapMarketDepth>;

        let asset = Asset::<(), (), Event>::l2_builder::<
            ConstantLatency,
            LinearAsset,
            QM,
            HashMapMarketDepth,
            TradingValueFeeModel<CommonFees>,
        >()
        .latency_model(latency_model)
        .asset_type(asset_type)
        .fee_model(fee_model)
        .queue_model(queue_model)
        .depth(move || HashMapMarketDepth::new(tick_size, lot_size))
        .data(vec![DataSource::Data(data)])
        .last_trades_capacity(100)
        .build()?;

        let backtest = Backtest::builder().add_asset(asset).build()?;

        Ok(backtest)
    }
}

impl Default for BacktestSessionBuilder<'static> {
    fn default() -> Self {
        Self::new()
    }
}
