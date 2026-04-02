//! # Scenario Configuration API
//!
//! This module provides the `ScenarioBuilder` fluent interface, which allows users
//! to define a "Market Recipe" with total tick counts, theme weights, and segment sizes.
//!
//! The builder ensures that the final `Scenario` is logically consistent (e.g., weights sum to 1.0)
//! before it is passed to the `MarkovGenerator` for execution.

use crate::types::MarketTheme;
use std::collections::HashMap;

/// # Final Scenario Configuration
///
/// An immutable data structure representing a complete "Market Recipe" that
/// the generator will execute. It encapsulates the total tick budget, theme
/// percentages, and reproduction parameters.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Total number of events (ticks) to generate for the entire scenario.
    pub total_ticks: usize,
    /// Mapping of high-level themes to their desired percentage (0.0 to 1.0).
    pub theme_weights: HashMap<MarketTheme, f64>,
    /// Global seed used for all random operations in the scenario.
    pub seed: u64,
    /// Minimum tick count for a single market segment (e.g., 5,000).
    pub min_segment_ticks: usize,
    /// Maximum tick count for a single market segment (e.g., 20,000).
    pub max_segment_ticks: usize,
}

/// # Fluent Scenario Builder
///
/// A stateful builder that collects the user's requirements and validates them.
/// This is the primary entry point for a user to configure a new market simulation.
pub struct ScenarioBuilder {
    total_ticks: usize,
    theme_weights: HashMap<MarketTheme, f64>,
    seed: Option<u64>,
    min_segment_ticks: usize,
    max_segment_ticks: usize,
}

impl ScenarioBuilder {
    /// Starts the builder for a fixed number of total ticks.
    ///
    /// # Parameters
    /// - `total_ticks`: The target number of events (e.g., 1,000,000).
    pub fn new(total_ticks: usize) -> Self {
        Self {
            total_ticks,
            theme_weights: HashMap::new(),
            seed: None,
            min_segment_ticks: 5_000,
            max_segment_ticks: 20_000,
        }
    }

    /// Sets the root seed for the simulation to ensure reproducible output.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Defines the range for the number of ticks in each randomized market segment.
    ///
    /// Smaller segments create more frequent regime changes (more "organic" but noisy),
    /// while larger segments create longer sustained trends (cleaner "story").
    pub fn segment_range(mut self, min: usize, max: usize) -> Self {
        self.min_segment_ticks = min;
        self.max_segment_ticks = max;
        self
    }

    /// Adds a `MarketTheme` and its corresponding target percentage of total ticks.
    ///
    /// # Parameters
    /// - `theme`: The market behavior to include.
    /// - `weight`: The fraction (0.0 to 1.0) of `total_ticks` to allot to this theme.
    pub fn add_theme(mut self, theme: MarketTheme, weight: f64) -> Self {
        self.theme_weights.insert(theme, weight);
        self
    }

    /// Validates and constructs the final `Scenario`.
    ///
    /// # Returns
    /// - `Ok(Scenario)`: If the weights sum to 1.0 and at least one theme is provided.
    /// - `Err(String)`: If the weights are invalid.
    pub fn build(self) -> Result<Scenario, String> {
        if self.theme_weights.is_empty() {
            return Err("At least one theme must be added".to_string());
        }

        let total_weight: f64 = self.theme_weights.values().sum();
        if (total_weight - 1.0).abs() > 1e-6 {
            return Err(format!(
                "Total weight must be 1.0, got: {:.4}",
                total_weight
            ));
        }

        Ok(Scenario {
            total_ticks: self.total_ticks,
            theme_weights: self.theme_weights,
            seed: self.seed.unwrap_or(42),
            min_segment_ticks: self.min_segment_ticks,
            max_segment_ticks: self.max_segment_ticks,
        })
    }
}
