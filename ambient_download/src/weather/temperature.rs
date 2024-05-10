//! Temperature module

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// The temperature in either Celsius, Fahrenheit, Kelvin, or Rankine. (Cause who doesn't need that?)
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Serialize, Deserialize)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
    Kelvin(f32),
    Rankine(f32),
}

impl Temperature {
    /// Create a new `Temperature` from a Celsius value.
    pub fn from_celsius(celsius: f32) -> Self {
        Self::Celsius(celsius)
    }

    /// Create a new `Temperature` from a Fahrenheit value.
    pub fn from_fahrenheit(fahrenheit: f32) -> Self {
        Self::Fahrenheit(fahrenheit)
    }

    /// Create a new `Temperature` from a Kelvin value.
    pub fn from_kelvin(kelvin: f32) -> Self {
        Self::Kelvin(kelvin)
    }

    /// Create a new `Temperature` from a Rankine value.
    pub fn from_rankine(rankine: f32) -> Self {
        Self::Rankine(rankine)
    }

    /// Convert the temperature to Celsius.
    pub fn to_celsius(&self) -> f32 {
        match self {
            Self::Celsius(c) => *c,
            Self::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
            Self::Kelvin(k) => k - 273.15,
            Self::Rankine(r) => (r - 491.67) * 5.0 / 9.0,
        }
    }

    /// Convert the temperature to Fahrenheit.
    pub fn to_fahrenheit(&self) -> f32 {
        match self {
            Self::Celsius(c) => c * 9.0 / 5.0 + 32.0,
            Self::Fahrenheit(f) => *f,
            Self::Kelvin(k) => k * 9.0 / 5.0 - 459.67,
            Self::Rankine(r) => r - 459.67,
        }
    }

    /// Convert the temperature to Kelvin.
    pub fn to_kelvin(&self) -> f32 {
        match self {
            Self::Celsius(c) => c + 273.15,
            Self::Fahrenheit(f) => (f + 459.67) * 5.0 / 9.0,
            Self::Kelvin(k) => *k,
            Self::Rankine(r) => r * 5.0 / 9.0,
        }
    }

    /// Convert the temperature to Rankine.
    pub fn to_rankine(&self) -> f32 {
        match self {
            Self::Celsius(c) => (c + 273.15) * 9.0 / 5.0,
            Self::Fahrenheit(f) => f + 459.67,
            Self::Kelvin(k) => k * 9.0 / 5.0,
            Self::Rankine(r) => *r,
        }
    }
}

// impl Serialize for Temperature {
//     /// Serialize the temperature to a float.
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::ser::Serializer,
//     {
//         match self {
//             Self::Celsius(c) => serializer.serialize_f32(*c),
//             Self::Fahrenheit(f) => serializer.serialize_f32(*f),
//             Self::Kelvin(k) => serializer.serialize_f32(*k),
//             Self::Rankine(r) => serializer.serialize_f32(*r),
//         }
//     }
// }

// impl<'de> Deserialize<'de> for Temperature {
//     /// Deserialize the temperature from a float.
//     fn deserialize<D>(deserializer: D) -> Result<Temperature, D::Error>
//     where
//         D: serde::de::Deserializer<'de>,
//     {
//         let value = f32::deserialize(deserializer)?;
//         Ok(Temperature::Celsius(value))
//     }
// }

impl Display for Temperature {
    /// Format the temperature for display.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Celsius(c) => write!(f, "{:.1}°C", c),
            Self::Fahrenheit(r) => write!(f, "{:.1}°F", r),
            Self::Kelvin(k) => write!(f, "{:.1}K", k),
            Self::Rankine(r) => write!(f, "{:.1}°R", r),
        }
    }
}
