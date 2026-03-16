//! Defines the status codes for the leak detector of the weather station.

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};

/// The leak detector status of the weather station.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Serialize)]
pub enum LeakDetector {
    /// Leak detector is OK - no leaks detected.
    OK = 0,

    /// Leak detector is detecting a leak.
    Leak = 1,

    /// Leak detector is offline.
    Offline = 2,

    /// Leak detector status is unknown - we don't know what's going on. We should never see this.
    Unknown = 100,
}

impl LeakDetector {
    /// Create a new `LeakDetector` from a number.
    #[must_use]
    pub const fn from_number(num: u8) -> Self {
        match num {
            0 => Self::OK,
            1 => Self::Leak,
            2 => Self::Offline,
            _ => Self::Unknown,
        }
    }

    /// Get the number representation of the leak detector status.
    #[must_use]
    pub const fn to_number(&self) -> u8 {
        match self {
            Self::OK => 0,
            Self::Leak => 1,
            Self::Offline => 2,
            Self::Unknown => 100,
        }
    }
}

impl<'de> Deserialize<'de> for LeakDetector {
    /// Accept an integer (0/1/2) or a string ("0"/"1"/"2") in addition to variant names.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let v = serde_json::Value::deserialize(deserializer)?;
        let n: u8 = match &v {
            serde_json::Value::Number(n) => u8::try_from(n.as_u64().unwrap_or(100)).unwrap_or(100),
            serde_json::Value::String(s) => s.parse().unwrap_or(100),
            _ => {
                return Err(D::Error::custom(format!(
                    "unexpected leak detector value: {v}"
                )))
            }
        };
        Ok(Self::from_number(n))
    }
}

impl Display for LeakDetector {
    /// Format the leak detector status for display.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::OK => write!(f, "OK"),
            Self::Leak => write!(f, "Leak"),
            Self::Offline => write!(f, "Offline"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
