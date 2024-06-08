//! Defines the units of measurement used in the system.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The system of units used in the system.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SystemOfUnits {
    /// The SI system of units.
    SI,

    /// The Imperial system of units.
    #[default]
    Imperial,

    /// Some other system of units, likely a mix of SI and Imperial.
    Other,
}

impl Display for SystemOfUnits {
    /// Format the length for display.
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::SI => write!(f, "SI"),
            Self::Imperial => write!(f, "Imperial"),
            Self::Other => write!(f, "Other"),
        }
    }
}
