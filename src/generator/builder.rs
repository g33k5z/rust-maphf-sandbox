use crate::types::MarketTheme;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scenario {
    pub total_ticks: usize,
    pub theme_weights: HashMap<MarketTheme, f64>,
    pub seed: u64,
    pub min_segment_ticks: usize,
    pub max_segment_ticks: usize,
}

pub struct ScenarioBuilder {
    total_ticks: usize,
    theme_weights: HashMap<MarketTheme, f64>,
    seed: Option<u64>,
    min_segment_ticks: usize,
    max_segment_ticks: usize,
}

impl ScenarioBuilder {
    pub fn new(total_ticks: usize) -> Self {
        Self {
            total_ticks,
            theme_weights: HashMap::new(),
            seed: None,
            min_segment_ticks: 5_000,
            max_segment_ticks: 20_000,
        }
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn segment_range(mut self, min: usize, max: usize) -> Self {
        self.min_segment_ticks = min;
        self.max_segment_ticks = max;
        self
    }

    pub fn add_theme(mut self, theme: MarketTheme, weight: f64) -> Self {
        self.theme_weights.insert(theme, weight);
        self
    }

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
