//! # Market Data Sandbox Library
//!
//! This library provides a modular framework for generating synthetic, high-frequency market data
//! compatible with `hftbacktest`. It bridges the gap between simple stochastic models and complex
//! historical-like scenarios by using a Theme-based, Budgeted Markov approach.
//!
//! The library is organized into three primary areas:
//! - `types`: Fundamental domain models and state tracking.
//! - `generator`: The orchestration engine for stochastic market simulation.
//! - `io`: Multi-format serializers for integration with Rust and Python environments.

pub mod generator;
pub mod io;
pub mod types;

#[cfg(test)]
mod tests;

pub use generator::{MarkovGenerator, ScenarioBuilder};
pub use types::{MarketState, MarketTheme};
