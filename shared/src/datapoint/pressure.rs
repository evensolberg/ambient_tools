//! Air pressure units.

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// The `AirPressure` of the weather station.
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
#[allow(non_camel_case_types)] // Because these are standardised units of measure.
pub enum AirPressure {
    /// The `AirPressure` in inches of mercury.
    inHg(f32),

    /// The `AirPressure` in millibars.
    mb(f32),

    /// The `AirPressure` in kilopascals.
    hPa(f32),

    /// The `AirPressure` in pounds per square inch.
    PSI(f32),

    /// The `AirPressure` in atmospheres.
    Atmospheres(f32),

    /// The `AirPressure` in millimeters of mercury.
    mmHg(f32),
}

impl AirPressure {
    /// Create a new `AirPressure` in inches of mercury.
    #[must_use]
    pub fn from_in_hg(p: f32) -> Self {
        Self::inHg(p)
    }

    /// Create a new `AirPressure` in millibars.
    #[must_use]
    pub fn from_mb(p: f32) -> Self {
        Self::mb(p)
    }

    /// Create a new `AirPressure` in kilopascals.
    #[must_use]
    pub fn from_hpa(p: f32) -> Self {
        Self::hPa(p)
    }

    /// Create a new `AirPressure` in pounds per square inch.
    #[must_use]
    pub fn from_psi(p: f32) -> Self {
        Self::PSI(p)
    }

    /// Create a new `AirPressure` in atmospheres.
    #[must_use]
    pub fn from_atmospheres(p: f32) -> Self {
        Self::Atmospheres(p)
    }

    /// Create a new `AirPressure` in millimeters of mercury.
    #[must_use]
    pub fn from_mmhg(p: f32) -> Self {
        Self::mmHg(p)
    }

    /// Convert the `AirPressure` to inches of mercury.
    #[must_use]
    pub fn to_inhg(&self) -> f32 {
        match self {
            Self::inHg(p) => *p,
            Self::mb(p) => p / 33.8639,
            Self::hPa(p) => p / 33.8639,
            Self::PSI(p) => p * 0.491_154,
            Self::Atmospheres(p) => p * 29.9213,
            Self::mmHg(p) => p / 25.4,
        }
    }

    /// Convert the `AirPressure` to millibars.
    #[must_use]
    pub fn to_mb(&self) -> f32 {
        match self {
            Self::inHg(p) => p * 33.8639,
            Self::mb(p) => *p,
            Self::hPa(p) => *p,
            Self::PSI(p) => p * 68.9476,
            Self::Atmospheres(p) => p * 1013.25,
            Self::mmHg(p) => p * 1.33322,
        }
    }

    /// Convert the `AirPressure` to kilopascals.
    #[must_use]
    pub fn to_hpa(&self) -> f32 {
        match self {
            Self::inHg(p) => p * 33.8639,
            Self::mb(p) => *p,
            Self::hPa(p) => *p,
            Self::PSI(p) => p * 6.89476,
            Self::Atmospheres(p) => p * 101.325,
            Self::mmHg(p) => p * 1.33322,
        }
    }

    /// Convert the `AirPressure` to pounds per square inch.
    #[must_use]
    pub fn to_psi(&self) -> f32 {
        match self {
            Self::inHg(p) => p * 2.03602,
            Self::mb(p) => p / 68.9476,
            Self::hPa(p) => p / 6.89476,
            Self::PSI(p) => *p,
            Self::Atmospheres(p) => p * 14.6959,
            Self::mmHg(p) => p / 51.7149,
        }
    }

    /// Convert the `AirPressure` to atmospheres.
    #[must_use]
    pub fn to_atmospheres(&self) -> f32 {
        match self {
            Self::inHg(p) => p * 0.033_421_1,
            Self::mb(p) => p / 1013.25,
            Self::hPa(p) => p / 101.325,
            Self::PSI(p) => p / 14.6959,
            Self::Atmospheres(p) => *p,
            Self::mmHg(p) => p / 760.0,
        }
    }

    /// Convert the `AirPressure` to millimeters of mercury.
    #[must_use]
    pub fn to_mmhg(&self) -> f32 {
        match self {
            Self::inHg(p) => p * 25.4,
            Self::mb(p) => p / 1.33322,
            Self::hPa(p) => p / 1.33322,
            Self::PSI(p) => p * 51.7149,
            Self::Atmospheres(p) => p * 760.0,
            Self::mmHg(p) => *p,
        }
    }

    /// Convert the `AirPressure` to the specified unit.
    #[must_use]
    pub fn to(&self, unit: Self) -> f32 {
        match unit {
            Self::inHg(_) => self.to_inhg(),
            Self::mb(_) => self.to_mb(),
            Self::hPa(_) => self.to_hpa(),
            Self::PSI(_) => self.to_psi(),
            Self::Atmospheres(_) => self.to_atmospheres(),
            Self::mmHg(_) => self.to_mmhg(),
        }
    }

    /// Convert the `AirPressure` from the specified unit.
    #[must_use]
    pub fn from(unit: Self, value: f32) -> Self {
        match unit {
            Self::inHg(_) => Self::inHg(value),
            Self::mb(_) => Self::mb(value),
            Self::hPa(_) => Self::hPa(value),
            Self::PSI(_) => Self::PSI(value),
            Self::Atmospheres(_) => Self::Atmospheres(value),
            Self::mmHg(_) => Self::mmHg(value),
        }
    }
}

impl Default for AirPressure {
    /// Create a new `AirPressure` with a value of 0.0 in inches of mercury.
    fn default() -> Self {
        Self::inHg(0.0)
    }
}

impl Serialize for AirPressure {
    /// Serialize the `AirPressure` to a float (inHg).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_f32(self.to_inhg())
    }
}

impl<'de> Deserialize<'de> for AirPressure {
    /// Deserialize the `AirPressure` from a float (assumes inHg, matching the Ambient Weather API).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Ok(Self::inHg(value))
    }
}

impl std::fmt::Display for AirPressure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::inHg(p) => write!(f, "{p} inHg"),
            Self::mb(p) => write!(f, "{p} mb"),
            Self::hPa(p) => write!(f, "{p} hPa"),
            Self::PSI(p) => write!(f, "{p} PSI"),
            Self::Atmospheres(p) => write!(f, "{p} Atmospheres"),
            Self::mmHg(p) => write!(f, "{p} mmHg"),
        }
    }
}
