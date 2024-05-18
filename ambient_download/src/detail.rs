//! Contains an enum that represents the amount of detail to output.

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum DetailLevel {
    Quiet = 0,
    #[default]
    Normal = 1,
    Detailed = 2,
    Debug = 3,
    Trace = 4,
}

impl DetailLevel {
    /// Returns the detail level from the given number.
    pub const fn from_number(number: u8) -> Self {
        match number {
            0 => Self::Quiet,
            1 => Self::Normal,
            2 => Self::Detailed,
            3 => Self::Debug,
            _ => Self::Trace,
        }
    }
}
