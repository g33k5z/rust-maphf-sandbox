//! # Market Data Generator Engine
//!
//! This module contains the logic for defining, building, and executing complex,
//! theme-based market data scenarios.
//!
//! It is organized into three key areas:
//! - `builder`: The `ScenarioBuilder` API for configuration.
//! - `markov`: The `MarkovGenerator` engine for execution.
//! - `theme`: Parameters and phase sequences for market behaviors.

pub mod builder;
pub mod markov;
pub mod theme;

pub use crate::types::MarketTheme;
pub use builder::{Scenario, ScenarioBuilder};
pub use markov::MarkovGenerator;
pub use theme::ThemeParams;
