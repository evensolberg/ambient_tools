//! Wind `WindSpeed` data types.

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// The `WindSpeed` of the weather station.
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub enum WindSpeed {
    /// The `WindSpeed` in miles per hour.
    MPH(f32),

    /// The `WindSpeed` in kilometers per hour.
    KPH(f32),

    /// The `WindSpeed` in meters per second.
    MPS(f32),

    /// The `WindSpeed` in Knots.
    Knots(f32),
}

impl WindSpeed {
    /// Create a new `WindSpeed` from a miles per hour value.
    #[must_use]
    pub fn from_mph(mph: f32) -> Self {
        Self::MPH(mph)
    }

    /// Create a new `WindSpeed` from a kilometers per hour value.
    #[must_use]
    pub fn from_kph(kph: f32) -> Self {
        Self::KPH(kph)
    }

    /// Create a new `WindSpeed` from a meters per second value.
    #[must_use]
    pub fn from_mps(mps: f32) -> Self {
        Self::MPS(mps)
    }

    /// Create a new `WindSpeed` from a knots value.
    #[must_use]
    pub fn from_knots(knots: f32) -> Self {
        Self::Knots(knots)
    }

    /// Convert the `WindSpeed` to miles per hour.
    #[must_use]
    pub fn to_mph(&self) -> f32 {
        match self {
            Self::MPH(m) => *m,
            Self::KPH(k) => k / 1.60934,
            Self::MPS(m) => m * 2.23694,
            Self::Knots(k) => k / 1.15078,
        }
    }

    /// Convert the `WindSpeed` to kilometers per hour.
    #[must_use]
    pub fn to_kph(&self) -> f32 {
        match self {
            Self::MPH(m) => m * 1.60934,
            Self::KPH(k) => *k,
            Self::MPS(m) => m * 3.6,
            Self::Knots(k) => k * 1.852,
        }
    }

    /// Convert the `WindSpeed` to meters per second.
    #[must_use]
    pub fn to_mps(&self) -> f32 {
        match self {
            Self::MPH(m) => m / 2.23694,
            Self::KPH(k) => k / 3.6,
            Self::MPS(m) => *m,
            Self::Knots(k) => k / 1.94384,
        }
    }

    /// Convert the `WindSpeed` to knots.
    #[must_use]
    pub fn to_knots(&self) -> f32 {
        match self {
            Self::Knots(k) => *k,
            _ => self.to_mps() * 1.94384,
        }
    }
}

impl std::fmt::Display for WindSpeed {
    /// Format the `WindSpeed` for display.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::MPH(m) => write!(f, "{m} mph"),
            Self::KPH(k) => write!(f, "{k} kph"),
            Self::MPS(m) => write!(f, "{m} m/s"),
            Self::Knots(k) => write!(f, "{k} knots"),
        }
    }
}

impl Serialize for WindSpeed {
    /// Serialize the `WindSpeed` to a float (mph).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_f32(self.to_mph())
    }
}

impl<'de> Deserialize<'de> for WindSpeed {
    /// Deserialize the `WindSpeed` from a float (assumes mph, matching the Ambient Weather API).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Ok(Self::MPH(value))
    }
}
