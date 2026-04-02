use crate::generator::builder::Scenario;
use crate::generator::theme::{RegimePhase, ThemeParams};
use crate::types::{MarketState, MarketTheme};
use hftbacktest::types::{Event, BUY_EVENT, EXCH_EVENT, LOCAL_EVENT, SELL_EVENT, TRADE_EVENT};
use market_data_source::{ConfigBuilder, MarketDataGenerator};
use rand::prelude::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;

pub struct MarkovGenerator {
    tick_size: f64,
    price_scale: f64,
}

impl MarkovGenerator {
    pub fn new(tick_size: f64, price_scale: f64) -> Self {
        Self {
            tick_size,
            price_scale,
        }
    }

    pub fn generate(&self, scenario: &Scenario, state: &mut MarketState) -> Vec<Event> {
        let mut rng = StdRng::seed_from_u64(scenario.seed);
        let mut all_events = Vec::with_capacity(scenario.total_ticks);
        let mut remaining_ticks = scenario.total_ticks;

        // Initialize budgets per theme
        let mut budgets: HashMap<MarketTheme, usize> = scenario
            .theme_weights
            .iter()
            .map(|(&t, &w)| (t, (scenario.total_ticks as f64 * w) as usize))
            .collect();

        // Adjust for rounding to ensure sum matches exactly
        let budget_sum: usize = budgets.values().sum();
        if budget_sum < scenario.total_ticks {
            let first_theme = *scenario.theme_weights.keys().next().unwrap();
            *budgets.get_mut(&first_theme).unwrap() += scenario.total_ticks - budget_sum;
        }

        // Current theme (start with Sideways if available, otherwise any)
        let mut current_theme = if budgets.contains_key(&MarketTheme::Sideways) {
            MarketTheme::Sideways
        } else {
            *budgets.keys().next().expect("No themes available")
        };

        println!(
            "Starting scenario generation for {} ticks...",
            scenario.total_ticks
        );

        while remaining_ticks > 0 {
            // Determine segment length
            let max_possible = budgets.get(&current_theme).cloned().unwrap_or(0);
            if max_possible == 0 {
                // Theme budget exhausted, pick a new one
                current_theme = self.pick_next_theme(&budgets, &mut rng);
                continue;
            }

            let mut segment_len =
                rng.gen_range(scenario.min_segment_ticks..scenario.max_segment_ticks);
            segment_len = segment_len.min(max_possible).min(remaining_ticks);

            if segment_len == 0 {
                break;
            }

            // Generate events for the theme
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

            // Transition to next theme
            current_theme = self.pick_next_theme(&budgets, &mut rng);
        }

        all_events
    }

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
            return MarketTheme::Sideways; // Should not happen given logic
        }

        // Uniform selection among available for simplicity (Budgeted Markov)
        *available_themes.choose(rng).unwrap()
    }

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

    fn generate_raw_segment(
        &self,
        state: &mut MarketState,
        num_ticks: usize,
        params: ThemeParams,
    ) -> Vec<Event> {
        let segment_seed = state.base_seed + state.segment_count;
        state.segment_count += 1;

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

            current_last_ts += 1;
            current_last_price = rounded_px;

            events.push(Event {
                ev: TRADE_EVENT | EXCH_EVENT | LOCAL_EVENT | side_flag,
                exch_ts: current_last_ts * 1_000_000,
                local_ts: (current_last_ts + 1) * 1_000_000,
                px: rounded_px * self.price_scale,
                qty: t.volume.value as f64,
                order_id: 0,
                ival: 0,
                fval: 0.0,
            });
        }

        state.last_price = current_last_price;
        state.last_timestamp_ms = current_last_ts;

        events
    }
}
