//! # Market Behavior Definitions
//!
//! This module defines the parameters and phase sequences that the generator uses
//! to create complex market "shapes" (V-Shape, U-Shape, etc.).
//!
//! Themes like `FlashCrash` are decomposed into `RegimePhase` sequences (Plunge then Snapback).
//! Each phase uses a specific set of `ThemeParams` for volatility, trend, and direction.

use crate::types::{MarketTheme, TrendDirection};

/// # Behavior Parameters
///
/// Defines the specific stochastic characteristics (volatility, trend) that
/// drive the low-level `MarketDataGenerator` in each market segment.
#[derive(Debug, Clone, Copy)]
pub struct ThemeParams {
    /// Volatility factor (e.g., 0.005 for 0.5% standard deviation).
    pub volatility: f64,
    /// Absolute trend factor for the direction.
    pub trend: f64,
    /// The direction of the trend (Bullish, Bearish, or Sideways).
    pub direction: TrendDirection,
}

impl ThemeParams {
    /// Medium-volatility, low-drift upward behavior.
    pub const BULLISH: Self = Self {
        volatility: 0.005,
        trend: 0.0002,
        direction: TrendDirection::Bullish,
    };

    /// High-volatility, medium-drift downward behavior.
    pub const BEARISH: Self = Self {
        volatility: 0.008,
        trend: 0.0005,
        direction: TrendDirection::Bearish,
    };

    /// Very low volatility, zero-drift behavior.
    pub const SIDEWAYS: Self = Self {
        volatility: 0.001,
        trend: 0.0,
        direction: TrendDirection::Sideways,
    };

    /// Extreme-volatility, aggressive downward behavior for flash crashes.
    pub const FLASH_CRASH_PLUNGE: Self = Self {
        volatility: 0.08,
        trend: 0.03,
        direction: TrendDirection::Bearish,
    };

    /// Extreme-volatility, aggressive upward recovery for flash crashes.
    pub const FLASH_CRASH_SNAPBACK: Self = Self {
        volatility: 0.06,
        trend: 0.025,
        direction: TrendDirection::Bullish,
    };

    /// Moderate-volatility, steady sell-off for market corrections.
    pub const CORRECTION_SLIDE: Self = Self {
        volatility: 0.015,
        trend: 0.004,
        direction: TrendDirection::Bearish,
    };

    /// Low-volatility, stagnant period at the bottom of a correction.
    pub const CORRECTION_BOTTOM: Self = Self {
        volatility: 0.002,
        trend: 0.0,
        direction: TrendDirection::Sideways,
    };

    /// Moderate-volatility recovery from a correction.
    pub const CORRECTION_RECOVER: Self = Self {
        volatility: 0.012,
        trend: 0.003,
        direction: TrendDirection::Bullish,
    };
}

/// # Internal Regime Phases
///
/// Determines how the `MarkovGenerator` breaks down a theme into segments.
/// Simple themes (Bullish, Sideways) are `Atomic`, while complex shapes (FlashCrash)
/// are defined as a `Sequence` of phases with relative duration weights.
#[derive(Debug, Clone, Copy)]
pub enum RegimePhase {
    /// A single behavior that stays consistent for the entire segment.
    Atomic(ThemeParams),
    /// A sequence of behaviors (e.g., [Plunge, Snapback]) with relative duration weights.
    Sequence(&'static [ThemeParams], &'static [f32]),
}

impl MarketTheme {
    /// Maps a high-level `MarketTheme` to its internal `RegimePhase` definition.
    ///
    /// This method is the "recipe book" for the generator, translating a theme
    /// like `Correction` into its three-part U-shape: [Slide, Bottom, Recover].
    pub fn get_phases(&self) -> RegimePhase {
        match self {
            MarketTheme::Bullish => RegimePhase::Atomic(ThemeParams::BULLISH),
            MarketTheme::Bearish => RegimePhase::Atomic(ThemeParams::BEARISH),
            MarketTheme::Sideways => RegimePhase::Atomic(ThemeParams::SIDEWAYS),
            MarketTheme::FlashCrash => RegimePhase::Sequence(
                &[
                    ThemeParams::FLASH_CRASH_PLUNGE,
                    ThemeParams::FLASH_CRASH_SNAPBACK,
                ],
                &[0.4, 0.6], // 40% crash duration, 60% recovery duration
            ),
            MarketTheme::Correction => RegimePhase::Sequence(
                &[
                    ThemeParams::CORRECTION_SLIDE,
                    ThemeParams::CORRECTION_BOTTOM,
                    ThemeParams::CORRECTION_RECOVER,
                ],
                &[0.3, 0.4, 0.3], // 30% slide, 40% bottom, 30% recovery duration
            ),
        }
    }
}
