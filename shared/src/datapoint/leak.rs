//! Defines the status codes for the leak detector of the weather station.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// The leak detector status of the weather station.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
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
    pub fn from_number(num: u8) -> Self {
        match num {
            0 => Self::OK,
            1 => Self::Leak,
            2 => Self::Offline,
            _ => Self::Unknown,
        }
    }

    /// Get the number representation of the leak detector status.
    #[must_use]
    pub fn to_number(&self) -> u8 {
        match self {
            Self::OK => 0,
            Self::Leak => 1,
            Self::Offline => 2,
            Self::Unknown => 100,
        }
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
