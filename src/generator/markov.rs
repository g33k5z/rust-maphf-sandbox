//! # Stochastic Data Engine
//!
//! This module implements the "Budgeted Markov" generator. It is the heart of the library,
//! where the high-level themes from a `Scenario` are converted into millions of market events.
//!
//! The engine uses a state-machine approach that:
//! 1.  **Budgets:** Calculates exact tick counts per theme (e.g., 250,000 for Bullish).
//! 2.  **Transitions:** Uses a stochastic model to pick the next theme based on availability.
//! 3.  **Executes:** Generates the underlying data using `MarketDataGenerator` while ensuring
//!     price and time continuity through `MarketState`.

use crate::generator::builder::Scenario;
use crate::generator::theme::{RegimePhase, ThemeParams};
use crate::types::{MarketState, MarketTheme};
use hftbacktest::types::{Event, BUY_EVENT, EXCH_EVENT, LOCAL_EVENT, SELL_EVENT, TRADE_EVENT};
use market_data_source::{ConfigBuilder, MarketDataGenerator};
use rand::prelude::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;

/// # The "Budgeted Markov" Generator
///
/// This engine is responsible for executing a `Scenario` by maintaining "Tick Budgets"
/// for each requested theme and orchestrating the transitions between them.
pub struct MarkovGenerator {
    /// The minimum price increment (e.g., 0.25 for S&P 500 futures).
    tick_size: f64,
    /// The multiplier used to scale prices in the final `Event` struct.
    price_scale: f64,
}

impl MarkovGenerator {
    /// Creates a new generator instance for the specified market instrument parameters.
    pub fn new(tick_size: f64, price_scale: f64) -> Self {
        Self {
            tick_size,
            price_scale,
        }
    }

    /// Generates a complete market scenario based on the provided recipe and state.
    ///
    /// This is the main simulation loop that:
    /// - Initializes budgets for all themes.
    /// - Iteratively picks a theme, generates a segment of random length, and decrements its budget.
    /// - Repeats until the total requested ticks (e.g., 1,000,000) are generated.
    ///
    /// # Parameters
    /// - `scenario`: The configuration (total ticks, weights, seed) to execute.
    /// - `state`: The initial market state, which is updated with the final price/time.
    pub fn generate(&self, scenario: &Scenario, state: &mut MarketState) -> Vec<Event> {
        let mut rng = StdRng::seed_from_u64(scenario.seed);
        let mut all_events = Vec::with_capacity(scenario.total_ticks);
        let mut remaining_ticks = scenario.total_ticks;

        // Initialize budgets per theme (converts percentages to counts)
        let mut budgets: HashMap<MarketTheme, usize> = scenario
            .theme_weights
            .iter()
            .map(|(&t, &w)| (t, (scenario.total_ticks as f64 * w) as usize))
            .collect();

        // Adjust for rounding to ensure sum matches exactly (adds residual to first theme)
        let budget_sum: usize = budgets.values().sum();
        if budget_sum < scenario.total_ticks {
            let first_theme = *scenario.theme_weights.keys().next().unwrap();
            *budgets.get_mut(&first_theme).unwrap() += scenario.total_ticks - budget_sum;
        }

        // Determine starting theme (prefer Sideways for a quiet start)
        let mut current_theme = if budgets.get(&MarketTheme::Sideways).cloned().unwrap_or(0) > 0 {
            MarketTheme::Sideways
        } else {
            *budgets.keys().next().expect("No themes available")
        };

        println!(
            "Starting scenario generation for {} ticks...",
            scenario.total_ticks
        );

        while remaining_ticks > 0 {
            // Determine segment length (randomized for more "organic" feel)
            let max_possible = budgets.get(&current_theme).cloned().unwrap_or(0);
            if max_possible == 0 {
                current_theme = self.pick_next_theme(&budgets, &mut rng);
                continue;
            }

            let mut segment_len =
                rng.gen_range(scenario.min_segment_ticks..scenario.max_segment_ticks);
            segment_len = segment_len.min(max_possible).min(remaining_ticks);

            if segment_len == 0 {
                break;
            }

            // Delegate to the theme-specific event generation (handles multi-phase sequences)
            let theme_events = self.generate_theme_events(state, current_theme, segment_len);

            println!(
                " - [{:?}] generated {} ticks ({} remaining for theme)",
                current_theme,
                theme_events.len(),
                budgets[&current_theme] - theme_events.len()
            );

            let actual_len = theme_events.len();
            all_events.extend(theme_events);
            remaining_ticks -= actual_len;
            *budgets.get_mut(&current_theme).unwrap() -= actual_len;

            // Roll for the next theme among those with remaining budget
            current_theme = self.pick_next_theme(&budgets, &mut rng);
        }

        all_events
    }

    /// Selects the next market theme based on availability of the remaining budget.
    ///
    /// This provides the "Markov" behavior by switching between available regimes
    /// in a way that avoids predictable, giant blocks of a single theme.
    fn pick_next_theme(
        &self,
        budgets: &HashMap<MarketTheme, usize>,
        rng: &mut StdRng,
    ) -> MarketTheme {
        let available_themes: Vec<_> = budgets
            .iter()
            .filter(|&(_, &b)| b > 0)
            .map(|(&t, _)| t)
            .collect();

        if available_themes.is_empty() {
            return MarketTheme::Sideways;
        }

        *available_themes.choose(rng).unwrap()
    }

    /// Generates events for a specific high-level theme, potentially through multiple phases.
    ///
    /// If a theme is a `Sequence` (like `FlashCrash`), this method divides the total
    /// segment length among its component phases (e.g., Plunge and Snapback)
    /// and generates each part in order to maintain the expected "shape."
    fn generate_theme_events(
        &self,
        state: &mut MarketState,
        theme: MarketTheme,
        total_ticks: usize,
    ) -> Vec<Event> {
        match theme.get_phases() {
            RegimePhase::Atomic(params) => self.generate_raw_segment(state, total_ticks, params),
            RegimePhase::Sequence(params_list, weights) => {
                let mut sequence_events = Vec::with_capacity(total_ticks);
                let mut ticks_used = 0;

                for (i, &params) in params_list.iter().enumerate() {
                    let phase_ticks = if i == params_list.len() - 1 {
                        total_ticks - ticks_used
                    } else {
                        (total_ticks as f32 * weights[i]) as usize
                    };

                    if phase_ticks > 0 {
                        sequence_events.extend(self.generate_raw_segment(
                            state,
                            phase_ticks,
                            params,
                        ));
                        ticks_used += phase_ticks;
                    }
                }
                sequence_events
            }
        }
    }

    /// Performs the lowest-level market data generation for a single, uniform segment.
    ///
    /// This method configures the underlying `MarketDataGenerator` and converts its
    /// output into the `Event` format required by `hftbacktest`. It is here that
    /// rounding to `tick_size` and time continuity (increments) are enforced.
    fn generate_raw_segment(
        &self,
        state: &mut MarketState,
        num_ticks: usize,
        params: ThemeParams,
    ) -> Vec<Event> {
        let segment_seed = state.base_seed + state.segment_count;
        state.segment_count += 1;

        // Configure the core stochastic generator
        let config = ConfigBuilder::new()
            .starting_price_f64(state.last_price)
            .volatility_f64(params.volatility)
            .trend_f64(params.direction, params.trend)
            .seed(segment_seed)
            .build()
            .expect("Failed to build config");

        let mut generator =
            MarketDataGenerator::with_config(config).expect("Failed to create generator");
        let ticks = generator.generate_ticks(num_ticks);

        let mut events = Vec::with_capacity(ticks.len());
        let mut current_last_price = state.last_price;
        let mut current_last_ts = state.last_timestamp_ms;

        for t in ticks {
            let raw_px = t.price.to_f64().unwrap();
            let rounded_px = (raw_px / self.tick_size).round() * self.tick_size;

            let side_flag = if rounded_px >= current_last_price {
                BUY_EVENT
            } else {
                SELL_EVENT
            };

            // Increment time and update price for the next tick
            current_last_ts += 1;
            current_last_price = rounded_px;

            events.push(Event {
                ev: TRADE_EVENT | EXCH_EVENT | LOCAL_EVENT | side_flag,
                exch_ts: current_last_ts * 1_000_000, // nanoseconds
                local_ts: (current_last_ts + 1) * 1_000_000, // 1ms simulated latency
                px: rounded_px * self.price_scale,
                qty: t.volume.value as f64,
                order_id: 0,
                ival: 0,
                fval: 0.0,
            });
        }

        // Update the global state so the next segment starts exactly where this one ended
        state.last_price = current_last_price;
        state.last_timestamp_ms = current_last_ts;

        events
    }
}
