//! Directions for the wind.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// The direction of the wind.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum WindDirection {
    /// North
    N(f32) = 0,

    /// North-Northeast
    NNE(f32) = 1,

    /// Northeast
    NE(f32) = 2,

    /// East-Northeast
    ENE(f32) = 3,

    /// East
    E(f32) = 4,

    /// East-Southeast
    ESE(f32) = 5,

    /// Southeast
    SE(f32) = 6,

    /// South-Southeast
    SSE(f32) = 7,

    /// South
    S(f32) = 8,

    /// South-Southwest
    SSW(f32) = 9,

    /// Southwest
    SW(f32) = 10,

    /// West-Southwest
    WSW(f32) = 11,

    /// West
    W(f32) = 12,

    /// West-Northwest
    WNW(f32) = 13,

    /// Northwest
    NW(f32) = 14,

    /// North-Northwest
    NNW(f32) = 15,

    /// Unknown direction
    Unknown(f32) = 100,
}

impl WindDirection {
    /// Get the direction of the wind from degrees (0-360).
    #[must_use]
    pub fn from_degrees(degrees: f32) -> Self {
        if degrees < 0.0 {
            return Self::N(degrees + 360.0);
        }

        match degrees % 360.0 {
            0.0..=11.25 => Self::N(degrees % 360.0),
            11.25..=33.75 => Self::NNE(degrees % 360.0),
            33.75..=56.25 => Self::NE(degrees % 360.0),
            56.25..=78.75 => Self::ENE(degrees % 360.0),
            78.75..=101.25 => Self::E(degrees % 360.0),
            101.25..=123.75 => Self::ESE(degrees % 360.0),
            123.75..=146.25 => Self::SE(degrees % 360.0),
            146.25..=168.75 => Self::SSE(degrees % 360.0),
            168.75..=191.25 => Self::S(degrees % 360.0),
            191.25..=213.75 => Self::SSW(degrees % 360.0),
            213.75..=236.25 => Self::SW(degrees % 360.0),
            236.25..=258.75 => Self::WSW(degrees % 360.0),
            258.75..=281.25 => Self::W(degrees % 360.0),
            281.25..=303.75 => Self::WNW(degrees % 360.0),
            303.75..=326.25 => Self::NW(degrees % 360.0),
            326.25..=348.75 => Self::NNW(degrees % 360.0),
            _ => Self::Unknown(degrees % 360.0), // Modulus to keep the degrees between 0 and 360.
        }
    }

    /// Create a new `Direction` from a number.
    #[must_use]
    pub fn from_number(num: u8) -> Self {
        match num {
            0 => Self::N(0.0),
            1 => Self::NNE(22.5),
            2 => Self::NE(45.0),
            3 => Self::ENE(67.5),
            4 => Self::E(90.0),
            5 => Self::ESE(112.5),
            6 => Self::SE(135.0),
            7 => Self::SSE(157.5),
            8 => Self::S(180.0),
            9 => Self::SSW(202.5),
            10 => Self::SW(225.0),
            11 => Self::WSW(247.5),
            12 => Self::W(270.0),
            13 => Self::WNW(292.5),
            14 => Self::NW(315.0),
            15 => Self::NNW(337.5),
            _ => Self::Unknown(0.0),
        }
    }

    /// Get the number representation of the direction.
    #[must_use]
    pub fn to_number(&self) -> u8 {
        match self {
            Self::N(_) => 0,
            Self::NNE(_) => 1,
            Self::NE(_) => 2,
            Self::ENE(_) => 3,
            Self::E(_) => 4,
            Self::ESE(_) => 5,
            Self::SE(_) => 6,
            Self::SSE(_) => 7,
            Self::S(_) => 8,
            Self::SSW(_) => 9,
            Self::SW(_) => 10,
            Self::WSW(_) => 11,
            Self::W(_) => 12,
            Self::WNW(_) => 13,
            Self::NW(_) => 14,
            Self::NNW(_) => 15,
            Self::Unknown(_) => 100,
        }
    }

    /// Get the compass direction of the wind.
    #[must_use]
    pub fn to_compass(&self) -> &str {
        match self {
            Self::N(_) => "N",
            Self::NNE(_) => "NNE",
            Self::NE(_) => "NE",
            Self::ENE(_) => "ENE",
            Self::E(_) => "E",
            Self::ESE(_) => "ESE",
            Self::SE(_) => "SE",
            Self::SSE(_) => "SSE",
            Self::S(_) => "S",
            Self::SSW(_) => "SSW",
            Self::SW(_) => "SW",
            Self::WSW(_) => "WSW",
            Self::W(_) => "W",
            Self::WNW(_) => "WNW",
            Self::NW(_) => "NW",
            Self::NNW(_) => "NNW",
            Self::Unknown(_) => "Unknown",
        }
    }

    /// Get the long form compass direction of the wind.
    #[must_use]
    pub fn to_compass_long(&self) -> &str {
        match self {
            Self::N(_) => "North",
            Self::NNE(_) => "North-Northeast",
            Self::NE(_) => "Northeast",
            Self::ENE(_) => "East-Northeast",
            Self::E(_) => "East",
            Self::ESE(_) => "East-Southeast",
            Self::SE(_) => "Southeast",
            Self::SSE(_) => "South-Southeast",
            Self::S(_) => "South",
            Self::SSW(_) => "South-Southwest",
            Self::SW(_) => "Southwest",
            Self::WSW(_) => "West-Southwest",
            Self::W(_) => "West",
            Self::WNW(_) => "West-Northwest",
            Self::NW(_) => "Northwest",
            Self::NNW(_) => "North-Northwest",
            Self::Unknown(_) => "Unknown",
        }
    }

    /// Get the compass direction of the wind in degrees.
    #[must_use]
    pub fn to_degrees(&self) -> f32 {
        match self {
            Self::N(d) => *d,
            Self::NNE(d) => *d,
            Self::NE(d) => *d,
            Self::ENE(d) => *d,
            Self::E(d) => *d,
            Self::ESE(d) => *d,
            Self::SE(d) => *d,
            Self::SSE(d) => *d,
            Self::S(d) => *d,
            Self::SSW(d) => *d,
            Self::SW(d) => *d,
            Self::WSW(d) => *d,
            Self::W(d) => *d,
            Self::WNW(d) => *d,
            Self::NW(d) => *d,
            Self::NNW(d) => *d,
            Self::Unknown(d) => *d,
        }
    }

    /// Get the opposite direction of the wind.
    #[must_use]
    pub fn opposite(&self) -> Self {
        match self {
            Self::N(d) => Self::S(*d + 180.0),
            Self::NNE(d) => Self::SSW(*d + 180.0),
            Self::NE(d) => Self::SW(*d + 180.0),
            Self::ENE(d) => Self::WSW(*d + 180.0),
            Self::E(d) => Self::W(*d + 180.0),
            Self::ESE(d) => Self::WNW(*d + 180.0),
            Self::SE(d) => Self::NW(*d + 180.0),
            Self::SSE(d) => Self::NNW(*d + 180.0),
            Self::S(d) => Self::N(*d - 180.0),
            Self::SSW(d) => Self::NNE(*d - 180.0),
            Self::SW(d) => Self::NE(*d - 180.0),
            Self::WSW(d) => Self::ENE(*d - 180.0),
            Self::W(d) => Self::E(*d - 180.0),
            Self::WNW(d) => Self::ESE(*d - 180.0),
            Self::NW(d) => Self::SE(*d - 180.0),
            Self::NNW(d) => Self::SSE(*d - 180.0),
            Self::Unknown(d) => Self::Unknown(*d),
        }
    }
}

impl Display for WindDirection {
    /// Format the direction for display.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::N(d) => write!(f, "{d:.0}°N"),
            Self::NNE(d) => write!(f, "{d:.0}°NNE"),
            Self::NE(d) => write!(f, "{d:.0}°NE"),
            Self::ENE(d) => write!(f, "{d:.0}°ENE"),
            Self::E(d) => write!(f, "{d:.0}°E"),
            Self::ESE(d) => write!(f, "{d:.0}°ESE"),
            Self::SE(d) => write!(f, "{d:.0}°SE"),
            Self::SSE(d) => write!(f, "{d:.0}°SSE"),
            Self::S(d) => write!(f, "{d:.0}°S"),
            Self::SSW(d) => write!(f, "{d:.0}°SSW"),
            Self::SW(d) => write!(f, "{d:.0}°SW"),
            Self::WSW(d) => write!(f, "{d:.0}°WSW"),
            Self::W(d) => write!(f, "{d:.0}°W"),
            Self::WNW(d) => write!(f, "{d:.0}°WNW"),
            Self::NW(d) => write!(f, "{d:.0}°NW"),
            Self::NNW(d) => write!(f, "{d:.0}°NNW"),
            Self::Unknown(d) => write!(f, "Unknown ({d:.0}°)"),
        }
    }
}

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    /// Test the `from_degrees` function.
    fn test_from_degrees() {
        assert_eq!(WindDirection::from_degrees(0.0), WindDirection::N(0.0));
        assert_eq!(WindDirection::from_degrees(10.0), WindDirection::N(10.0));
        assert_eq!(WindDirection::from_degrees(12.5), WindDirection::NNE(12.5));
        assert_eq!(WindDirection::from_degrees(22.5), WindDirection::NNE(22.5));
        assert_eq!(WindDirection::from_degrees(45.0), WindDirection::NE(45.0));
        assert_eq!(WindDirection::from_degrees(67.5), WindDirection::ENE(67.5));
        assert_eq!(WindDirection::from_degrees(90.0), WindDirection::E(90.0));
        assert_eq!(
            WindDirection::from_degrees(112.5),
            WindDirection::ESE(112.5)
        );
        assert_eq!(WindDirection::from_degrees(135.0), WindDirection::SE(135.0));
        assert_eq!(
            WindDirection::from_degrees(157.5),
            WindDirection::SSE(157.5)
        );
        assert_eq!(WindDirection::from_degrees(180.0), WindDirection::S(180.0));
        assert_eq!(
            WindDirection::from_degrees(202.5),
            WindDirection::SSW(202.5)
        );
        assert_eq!(WindDirection::from_degrees(225.0), WindDirection::SW(225.0));
        assert_eq!(
            WindDirection::from_degrees(247.5),
            WindDirection::WSW(247.5)
        );
        assert_eq!(WindDirection::from_degrees(270.0), WindDirection::W(270.0));
        assert_eq!(
            WindDirection::from_degrees(292.5),
            WindDirection::WNW(292.5)
        );
        assert_eq!(WindDirection::from_degrees(315.0), WindDirection::NW(315.0));
        assert_eq!(
            WindDirection::from_degrees(337.5),
            WindDirection::NNW(337.5)
        );
        assert_eq!(WindDirection::from_degrees(360.0), WindDirection::N(0.0));
        assert_eq!(WindDirection::from_degrees(-22.5), WindDirection::N(337.5));
    }

    #[test]
    /// Test the `from_number` function.
    fn test_from_number() {
        assert_eq!(WindDirection::from_number(0), WindDirection::N(0.0));
        assert_eq!(WindDirection::from_number(1), WindDirection::NNE(22.5));
        assert_eq!(WindDirection::from_number(2), WindDirection::NE(45.0));
        assert_eq!(WindDirection::from_number(3), WindDirection::ENE(67.5));
        assert_eq!(WindDirection::from_number(4), WindDirection::E(90.0));
        assert_eq!(WindDirection::from_number(5), WindDirection::ESE(112.5));
        assert_eq!(WindDirection::from_number(6), WindDirection::SE(135.0));
        assert_eq!(WindDirection::from_number(7), WindDirection::SSE(157.5));
        assert_eq!(WindDirection::from_number(8), WindDirection::S(180.0));
        assert_eq!(WindDirection::from_number(9), WindDirection::SSW(202.5));
        assert_eq!(WindDirection::from_number(10), WindDirection::SW(225.0));
        assert_eq!(WindDirection::from_number(11), WindDirection::WSW(247.5));
        assert_eq!(WindDirection::from_number(12), WindDirection::W(270.0));
        assert_eq!(WindDirection::from_number(13), WindDirection::WNW(292.5));
        assert_eq!(WindDirection::from_number(14), WindDirection::NW(315.0));
        assert_eq!(WindDirection::from_number(15), WindDirection::NNW(337.5));
        assert_eq!(WindDirection::from_number(16), WindDirection::Unknown(0.0));
    }

    #[test]
    /// Test the `to_number` function.
    fn test_to_number() {
        assert_eq!(WindDirection::N(0.0).to_number(), 0);
        assert_eq!(WindDirection::NNE(22.5).to_number(), 1);
        assert_eq!(WindDirection::NE(45.0).to_number(), 2);
        assert_eq!(WindDirection::ENE(67.5).to_number(), 3);
        assert_eq!(WindDirection::E(90.0).to_number(), 4);
        assert_eq!(WindDirection::ESE(112.5).to_number(), 5);
        assert_eq!(WindDirection::SE(135.0).to_number(), 6);
        assert_eq!(WindDirection::SSE(157.5).to_number(), 7);
        assert_eq!(WindDirection::S(180.0).to_number(), 8);
        assert_eq!(WindDirection::SSW(202.5).to_number(), 9);
        assert_eq!(WindDirection::SW(225.0).to_number(), 10);
        assert_eq!(WindDirection::WSW(247.5).to_number(), 11);
        assert_eq!(WindDirection::W(270.0).to_number(), 12);
        assert_eq!(WindDirection::WNW(292.5).to_number(), 13);
        assert_eq!(WindDirection::NW(315.0).to_number(), 14);
        assert_eq!(WindDirection::NNW(337.5).to_number(), 15);
        assert_eq!(WindDirection::Unknown(0.0).to_number(), 100);
    }

    #[test]
    /// Test the `to_compass` function.
    fn test_to_compass() {
        assert_eq!(WindDirection::N(0.0).to_compass(), "N");
        assert_eq!(WindDirection::NNE(22.5).to_compass(), "NNE");
        assert_eq!(WindDirection::NE(45.0).to_compass(), "NE");
        assert_eq!(WindDirection::ENE(67.5).to_compass(), "ENE");
        assert_eq!(WindDirection::E(90.0).to_compass(), "E");
        assert_eq!(WindDirection::ESE(112.5).to_compass(), "ESE");
        assert_eq!(WindDirection::SE(135.0).to_compass(), "SE");
        assert_eq!(WindDirection::SSE(157.5).to_compass(), "SSE");
        assert_eq!(WindDirection::S(180.0).to_compass(), "S");
        assert_eq!(WindDirection::SSW(202.5).to_compass(), "SSW");
        assert_eq!(WindDirection::SW(225.0).to_compass(), "SW");
        assert_eq!(WindDirection::WSW(247.5).to_compass(), "WSW");
        assert_eq!(WindDirection::W(270.0).to_compass(), "W");
        assert_eq!(WindDirection::WNW(292.5).to_compass(), "WNW");
        assert_eq!(WindDirection::NW(315.0).to_compass(), "NW");
        assert_eq!(WindDirection::NNW(337.5).to_compass(), "NNW");
        assert_eq!(WindDirection::Unknown(0.0).to_compass(), "Unknown");
    }

    #[test]
    /// Test the `to_compass_long` function.
    fn test_to_compass_long() {
        assert_eq!(WindDirection::N(0.0).to_compass_long(), "North");
        assert_eq!(
            WindDirection::NNE(22.5).to_compass_long(),
            "North-Northeast"
        );
        assert_eq!(WindDirection::NE(45.0).to_compass_long(), "Northeast");
        assert_eq!(WindDirection::ENE(67.5).to_compass_long(), "East-Northeast");
        assert_eq!(WindDirection::E(90.0).to_compass_long(), "East");
        assert_eq!(
            WindDirection::ESE(112.5).to_compass_long(),
            "East-Southeast"
        );
        assert_eq!(WindDirection::SE(135.0).to_compass_long(), "Southeast");
        assert_eq!(
            WindDirection::SSE(157.5).to_compass_long(),
            "South-Southeast"
        );
        assert_eq!(WindDirection::S(180.0).to_compass_long(), "South");
        assert_eq!(
            WindDirection::SSW(202.5).to_compass_long(),
            "South-Southwest"
        );
        assert_eq!(WindDirection::SW(225.0).to_compass_long(), "Southwest");
        assert_eq!(
            WindDirection::WSW(247.5).to_compass_long(),
            "West-Southwest"
        );
        assert_eq!(WindDirection::W(270.0).to_compass_long(), "West");
        assert_eq!(
            WindDirection::WNW(292.5).to_compass_long(),
            "West-Northwest"
        );
        assert_eq!(WindDirection::NW(315.0).to_compass_long(), "Northwest");
        assert_eq!(
            WindDirection::NNW(337.5).to_compass_long(),
            "North-Northwest"
        );
        assert_eq!(WindDirection::Unknown(0.0).to_compass_long(), "Unknown");
    }

    #[test]
    /// Test the `to_degrees` function.
    fn test_to_degrees() {
        assert_eq!(WindDirection::N(0.0).to_degrees(), 0.0);
        assert_eq!(WindDirection::N(10.0).to_degrees(), 10.0);
        assert_eq!(WindDirection::NNE(22.5).to_degrees(), 22.5);
        assert_eq!(WindDirection::NE(45.0).to_degrees(), 45.0);
        assert_eq!(WindDirection::ENE(67.5).to_degrees(), 67.5);
        assert_eq!(WindDirection::E(90.0).to_degrees(), 90.0);
        assert_eq!(WindDirection::ESE(112.5).to_degrees(), 112.5);
        assert_eq!(WindDirection::SE(135.0).to_degrees(), 135.0);
        assert_eq!(WindDirection::SSE(157.5).to_degrees(), 157.5);
        assert_eq!(WindDirection::S(180.0).to_degrees(), 180.0);
        assert_eq!(WindDirection::SSW(202.5).to_degrees(), 202.5);
        assert_eq!(WindDirection::SW(225.0).to_degrees(), 225.0);
        assert_eq!(WindDirection::WSW(247.5).to_degrees(), 247.5);
        assert_eq!(WindDirection::W(270.0).to_degrees(), 270.0);
        assert_eq!(WindDirection::WNW(292.5).to_degrees(), 292.5);
        assert_eq!(WindDirection::NW(315.0).to_degrees(), 315.0);
        assert_eq!(WindDirection::NNW(337.5).to_degrees(), 337.5);
        assert_eq!(WindDirection::Unknown(0.0).to_degrees(), 0.0);
    }

    #[test]
    /// Test the `opposite` function.
    fn test_opposite() {
        assert_eq!(WindDirection::N(0.0).opposite(), WindDirection::S(180.0));
        assert_eq!(
            WindDirection::NNE(22.5).opposite(),
            WindDirection::SSW(202.5)
        );
        assert_eq!(WindDirection::NE(45.0).opposite(), WindDirection::SW(225.0));
        assert_eq!(
            WindDirection::ENE(67.5).opposite(),
            WindDirection::WSW(247.5)
        );
        assert_eq!(WindDirection::E(90.0).opposite(), WindDirection::W(270.0));
        assert_eq!(
            WindDirection::ESE(112.5).opposite(),
            WindDirection::WNW(292.5)
        );
        assert_eq!(
            WindDirection::SE(135.0).opposite(),
            WindDirection::NW(315.0)
        );
        assert_eq!(
            WindDirection::SSE(157.5).opposite(),
            WindDirection::NNW(337.5)
        );
        assert_eq!(WindDirection::S(180.0).opposite(), WindDirection::N(0.0));
        assert_eq!(
            WindDirection::SSW(202.5).opposite(),
            WindDirection::NNE(22.5)
        );
        assert_eq!(WindDirection::SW(225.0).opposite(), WindDirection::NE(45.0));
        assert_eq!(
            WindDirection::WSW(247.5).opposite(),
            WindDirection::ENE(67.5)
        );
        assert_eq!(WindDirection::W(270.0).opposite(), WindDirection::E(90.0));
        assert_eq!(
            WindDirection::WNW(292.5).opposite(),
            WindDirection::ESE(112.5)
        );
        assert_eq!(
            WindDirection::NW(315.0).opposite(),
            WindDirection::SE(135.0)
        );
        assert_eq!(
            WindDirection::NNW(337.5).opposite(),
            WindDirection::SSE(157.5)
        );
        assert_eq!(
            WindDirection::Unknown(0.0).opposite(),
            WindDirection::Unknown(0.0)
        );
    }

    /// Test serialization and deserialization of the `WindDirection` enum.
    #[test]
    fn test_wind_direction_serde() {
        let dir = WindDirection::N(0.0);
        let serialized = serde_json::to_string(&dir).unwrap();
        assert_eq!(serialized, r#"{"N":0.0}"#);

        let deserialized: WindDirection = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, dir);

        let serialized2 = r#"{"NNE":22.5}"#;
        let deserialized2: WindDirection = serde_json::from_str(serialized2).unwrap();
        assert_eq!(deserialized2, WindDirection::NNE(22.5));
    }
}
