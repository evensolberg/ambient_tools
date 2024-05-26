//! Defines the battery status enum for the weather station.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// The battery status of the weather station.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
pub enum BatteryStatus {
    /// Battery is low
    Low = 0,

    /// Battery is OK
    Ok = 1,

    /// Battery status is unknown - we don't know what's going on. We should never see this.
    Unknown = 100,
}

/// Implement the `BatteryStatus` enum.
impl BatteryStatus {
    /// Create a new `BatteryStatus` from a number.
    /// 0 = Low, 1 = OK, any other number = Unknown
    pub fn from_number(num: u8) -> Self {
        match num {
            0 => Self::Low,
            1 => Self::Ok,
            _ => Self::Unknown,
        }
    }

    /// Create a new `BatteryStatus` from a leak detector number.
    /// Use this for sensors that have opposite values for battery status.
    /// 0 = OK, 1 = Low, any other number = Unknown
    pub fn from_number_reversed(num: u8) -> Self {
        match num {
            0 => Self::Ok,
            1 => Self::Low,
            _ => Self::Unknown,
        }
    }

    /// Get the number representation of the battery status.
    pub fn to_number(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Ok => 1,
            Self::Unknown => 100,
        }
    }
}

impl Display for BatteryStatus {
    /// Format the battery status for display.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Ok => write!(f, "OK"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
