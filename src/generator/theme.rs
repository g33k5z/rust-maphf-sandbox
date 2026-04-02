use crate::types::{MarketTheme, TrendDirection};

#[derive(Debug, Clone, Copy)]
pub struct ThemeParams {
    pub volatility: f64,
    pub trend: f64,
    pub direction: TrendDirection,
}

impl ThemeParams {
    pub const BULLISH: Self = Self {
        volatility: 0.005,
        trend: 0.0002,
        direction: TrendDirection::Bullish,
    };

    pub const BEARISH: Self = Self {
        volatility: 0.008,
        trend: 0.0005,
        direction: TrendDirection::Bearish,
    };

    pub const SIDEWAYS: Self = Self {
        volatility: 0.001,
        trend: 0.0,
        direction: TrendDirection::Sideways,
    };

    pub const FLASH_CRASH_PLUNGE: Self = Self {
        volatility: 0.08,
        trend: 0.03,
        direction: TrendDirection::Bearish,
    };

    pub const FLASH_CRASH_SNAPBACK: Self = Self {
        volatility: 0.06,
        trend: 0.025,
        direction: TrendDirection::Bullish,
    };

    pub const CORRECTION_SLIDE: Self = Self {
        volatility: 0.015,
        trend: 0.004,
        direction: TrendDirection::Bearish,
    };

    pub const CORRECTION_BOTTOM: Self = Self {
        volatility: 0.002,
        trend: 0.0,
        direction: TrendDirection::Sideways,
    };

    pub const CORRECTION_RECOVER: Self = Self {
        volatility: 0.012,
        trend: 0.003,
        direction: TrendDirection::Bullish,
    };
}

/// Represents the internal segments of a theme.
#[derive(Debug, Clone, Copy)]
pub enum RegimePhase {
    Atomic(ThemeParams),
    Sequence(&'static [ThemeParams], &'static [f32]), // Params + relative duration weights
}

impl MarketTheme {
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
                &[0.4, 0.6], // 40% crash, 60% recovery (longer)
            ),
            MarketTheme::Correction => RegimePhase::Sequence(
                &[
                    ThemeParams::CORRECTION_SLIDE,
                    ThemeParams::CORRECTION_BOTTOM,
                    ThemeParams::CORRECTION_RECOVER,
                ],
                &[0.3, 0.4, 0.3], // 30% slide, 40% bottom, 30% recovery (U-Shape)
            ),
        }
    }
}
