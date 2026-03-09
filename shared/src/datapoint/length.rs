use serde::{Deserialize, Serialize};

use std::fmt::Formatter;

/// `THe` length unit of measure
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub enum Length {
    /// The length in inches
    Inches(f32),

    /// The length in millimeters
    Millimeters(f32),
}

impl Length {
    /// Create a new `Length` from an inch value.
    #[must_use]
    pub fn from_inches(inches: f32) -> Self {
        Self::Inches(inches)
    }

    /// Create a new `Length` from a millimeter value.
    #[must_use]
    pub fn from_millimeters(millimeters: f32) -> Self {
        Self::Millimeters(millimeters)
    }

    /// Convert the length to inches.
    #[must_use]
    pub fn to_inches(&self) -> f32 {
        match self {
            Self::Inches(i) => *i,
            Self::Millimeters(m) => m / 25.4,
        }
    }

    /// Convert the length to millimeters.
    #[must_use]
    pub fn to_millimeters(&self) -> f32 {
        match self {
            Self::Inches(i) => i * 25.4,
            Self::Millimeters(m) => *m,
        }
    }
}

impl std::fmt::Display for Length {
    /// Format the length for display.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Inches(i) => write!(f, "{i} in"),
            Self::Millimeters(mm) => write!(f, "{mm} mm"),
        }
    }
}

impl Serialize for Length {
    /// Serialize the length to a float (inches).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_f32(self.to_inches())
    }
}

impl<'de> Deserialize<'de> for Length {
    /// Deserialize the length from a float (assumes inches, matching the Ambient Weather API).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Ok(Self::Inches(value))
    }
}
