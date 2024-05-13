//! Air pressure units.

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// The AirPressure of the weather station.
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Serialize, Deserialize)]
pub enum AirPressure {
    /// The AirPressure in inches of mercury.
    inHg(f32),

    /// The AirPressure in millibars.
    mb(f32),

    /// The AirPressure in kilopascals.
    hPa(f32),

    /// The AirPressure in pounds per square inch.
    PSI(f32),

    /// The AirPressure in atmospheres.
    Atmospheres(f32),

    /// The AirPressure in millimeters of mercury.
    mmHg(f32),
}

impl AirPressure {
    /// Create a new AirPressure in inches of mercury.
    pub fn from_in_hg(p: f32) -> Self {
        AirPressure::inHg(p)
    }

    /// Create a new AirPressure in millibars.
    pub fn from_mb(p: f32) -> Self {
        AirPressure::mb(p)
    }

    /// Create a new AirPressure in kilopascals.
    pub fn from_hpa(p: f32) -> Self {
        AirPressure::hPa(p)
    }

    /// Create a new AirPressure in pounds per square inch.
    pub fn from_psi(p: f32) -> Self {
        AirPressure::PSI(p)
    }

    /// Create a new AirPressure in atmospheres.
    pub fn from_atmospheres(p: f32) -> Self {
        AirPressure::Atmospheres(p)
    }

    /// Create a new AirPressure in millimeters of mercury.
    pub fn from_mmhg(p: f32) -> Self {
        AirPressure::mmHg(p)
    }

    /// Convert the AirPressure to inches of mercury.
    pub fn to_inhg(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => *p,
            AirPressure::mb(p) => p / 33.8639,
            AirPressure::hPa(p) => p / 33.8639,
            AirPressure::PSI(p) => p * 0.491154,
            AirPressure::Atmospheres(p) => p * 29.9213,
            AirPressure::mmHg(p) => p / 25.4,
        }
    }

    /// Convert the AirPressure to millibars.
    pub fn to_mb(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => p * 33.8639,
            AirPressure::mb(p) => *p,
            AirPressure::hPa(p) => *p,
            AirPressure::PSI(p) => p * 68.9476,
            AirPressure::Atmospheres(p) => p * 1013.25,
            AirPressure::mmHg(p) => p * 1.33322,
        }
    }

    /// Convert the AirPressure to kilopascals.
    pub fn to_hpa(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => p * 33.8639,
            AirPressure::mb(p) => *p,
            AirPressure::hPa(p) => *p,
            AirPressure::PSI(p) => p * 6.89476,
            AirPressure::Atmospheres(p) => p * 101.325,
            AirPressure::mmHg(p) => p * 1.33322,
        }
    }

    /// Convert the AirPressure to pounds per square inch.
    pub fn to_psi(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => p * 2.03602,
            AirPressure::mb(p) => p / 68.9476,
            AirPressure::hPa(p) => p / 6.89476,
            AirPressure::PSI(p) => *p,
            AirPressure::Atmospheres(p) => p * 14.6959,
            AirPressure::mmHg(p) => p / 51.7149,
        }
    }

    /// Convert the AirPressure to atmospheres.
    pub fn to_atmospheres(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => p * 0.0334211,
            AirPressure::mb(p) => p / 1013.25,
            AirPressure::hPa(p) => p / 101.325,
            AirPressure::PSI(p) => p / 14.6959,
            AirPressure::Atmospheres(p) => *p,
            AirPressure::mmHg(p) => p / 760.0,
        }
    }

    /// Convert the AirPressure to millimeters of mercury.
    pub fn to_mmhg(&self) -> f32 {
        match self {
            AirPressure::inHg(p) => p * 25.4,
            AirPressure::mb(p) => p / 1.33322,
            AirPressure::hPa(p) => p / 1.33322,
            AirPressure::PSI(p) => p * 51.7149,
            AirPressure::Atmospheres(p) => p * 760.0,
            AirPressure::mmHg(p) => *p,
        }
    }

    /// Convert the AirPressure to the specified unit.
    pub fn to(&self, unit: AirPressure) -> f32 {
        match unit {
            AirPressure::inHg(_) => self.to_inhg(),
            AirPressure::mb(_) => self.to_mb(),
            AirPressure::hPa(_) => self.to_hpa(),
            AirPressure::PSI(_) => self.to_psi(),
            AirPressure::Atmospheres(_) => self.to_atmospheres(),
            AirPressure::mmHg(_) => self.to_mmhg(),
        }
    }

    /// Convert the AirPressure from the specified unit.
    pub fn from(unit: AirPressure, value: f32) -> Self {
        match unit {
            AirPressure::inHg(_) => AirPressure::inHg(value),
            AirPressure::mb(_) => AirPressure::mb(value),
            AirPressure::hPa(_) => AirPressure::hPa(value),
            AirPressure::PSI(_) => AirPressure::PSI(value),
            AirPressure::Atmospheres(_) => AirPressure::Atmospheres(value),
            AirPressure::mmHg(_) => AirPressure::mmHg(value),
        }
    }
}

impl Default for AirPressure {
    /// Create a new AirPressure with a value of 0.0 in inches of mercury.
    fn default() -> Self {
        Self::inHg(0.0)
    }
}

impl std::fmt::Display for AirPressure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AirPressure::inHg(p) => write!(f, "{} inHg", p),
            AirPressure::mb(p) => write!(f, "{} mb", p),
            AirPressure::hPa(p) => write!(f, "{} hPa", p),
            AirPressure::PSI(p) => write!(f, "{} PSI", p),
            AirPressure::Atmospheres(p) => write!(f, "{} Atmospheres", p),
            AirPressure::mmHg(p) => write!(f, "{} mmHg", p),
        }
    }
}
