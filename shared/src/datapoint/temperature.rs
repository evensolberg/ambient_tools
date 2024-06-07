//! Temperature module

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// The temperature in either Celsius, Fahrenheit, Kelvin, or Rankine. (Cause who doesn't need that?)
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
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
            Self::Kelvin(k) => k - 273.15002,
            Self::Rankine(r) => (r - 491.66998) * 5.0 / 9.0,
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
            Self::Celsius(c) => c + 273.15002,
            Self::Fahrenheit(f) => (f + 459.67) * 5.0 / 9.0,
            Self::Kelvin(k) => *k,
            Self::Rankine(r) => r * 5.0 / 9.0,
        }
    }

    /// Convert the temperature to Rankine.
    pub fn to_rankine(&self) -> f32 {
        match self {
            Self::Celsius(c) => (c + 273.15002) * 9.0 / 5.0,
            Self::Fahrenheit(f) => f + 459.67,
            Self::Kelvin(k) => k * 9.0 / 5.0,
            Self::Rankine(r) => *r,
        }
    }
}

impl Serialize for Temperature {
    /// Serialize the temperature to a float.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Self::Celsius(c) => serializer.serialize_f32(*c),
            Self::Fahrenheit(f) => serializer.serialize_f32(*f),
            Self::Kelvin(k) => serializer.serialize_f32(*k),
            Self::Rankine(r) => serializer.serialize_f32(*r),
        }
    }
}

impl<'de> Deserialize<'de> for Temperature {
    /// Deserialize the temperature from a float. (Assumes Fahrenheit)
    fn deserialize<D>(deserializer: D) -> Result<Temperature, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = f32::deserialize(deserializer)?;
        Ok(Temperature::Fahrenheit(value))
    }
}

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

impl Default for Temperature {
    /// Default to 0.0 degrees Fahrenheit.
    fn default() -> Self {
        Self::Fahrenheit(0.0)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversions() {
        let celsius = Temperature::from_celsius(0.0);
        assert_eq!(celsius.to_celsius(), 0.0);
        assert_eq!(celsius.to_fahrenheit(), 32.0);
        assert_eq!(celsius.to_kelvin(), 273.15002);
        assert_eq!(celsius.to_rankine(), 491.67);

        let fahrenheit = Temperature::from_fahrenheit(32.0);
        assert_eq!(fahrenheit.to_celsius(), 0.0);
        assert_eq!(fahrenheit.to_fahrenheit(), 32.0);
        assert_eq!(fahrenheit.to_kelvin(), 273.15002);
        assert_eq!(fahrenheit.to_rankine(), 491.67);

        let kelvin = Temperature::from_kelvin(273.15002);
        assert_eq!(kelvin.to_celsius(), 0.0);
        assert_eq!(kelvin.to_fahrenheit(), 32.0);
        assert_eq!(kelvin.to_kelvin(), 273.15002);
        assert_eq!(kelvin.to_rankine(), 491.67);

        let rankine = Temperature::from_rankine(491.66998);
        assert_eq!(rankine.to_celsius(), 0.0);
        assert_eq!(rankine.to_fahrenheit(), 31.99997);
        assert_eq!(rankine.to_kelvin(), 273.15);
        assert_eq!(rankine.to_rankine(), 491.66998);
    }

    #[test]
    fn test_temperature_serialization() {
        let celsius = Temperature::from_celsius(0.0);
        let fahrenheit = Temperature::from_fahrenheit(32.0);
        let kelvin = Temperature::from_kelvin(273.15002);
        let rankine = Temperature::from_rankine(491.66998);

        let celsius_json = serde_json::to_string(&celsius).unwrap();
        let fahrenheit_json = serde_json::to_string(&fahrenheit).unwrap();
        let kelvin_json = serde_json::to_string(&kelvin).unwrap();
        let rankine_json = serde_json::to_string(&rankine).unwrap();

        assert_eq!(celsius_json, "0.0");
        assert_eq!(fahrenheit_json, "32.0");
        assert_eq!(kelvin_json, "273.15002");
        assert_eq!(rankine_json, "491.66998");

        // let celsius_deserialized: Temperature = serde_json::from_str(&celsius_json).unwrap();
        // let fahrenheit_deserialized: Temperature = serde_json::from_str(&fahrenheit_json).unwrap();
        // let kelvin_deserialized: Temperature = serde_json::from_str(&kelvin_json).unwrap();
        // let rankine_deserialized: Temperature = serde_json::from_str(&rankine_json).unwrap();

        // assert_eq!(celsius, celsius_deserialized);
        // assert_eq!(fahrenheit, fahrenheit_deserialized);
        // assert_eq!(kelvin, kelvin_deserialized);
        // assert_eq!(rankine, rankine_deserialized);
    }
}
