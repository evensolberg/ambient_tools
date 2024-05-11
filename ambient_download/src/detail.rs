//! Contains an enum that represents the amount of detail to output.

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
    pub fn from_number(number: u8) -> Self {
        match number {
            0 => DetailLevel::Quiet,
            1 => DetailLevel::Normal,
            2 => DetailLevel::Detailed,
            3 => DetailLevel::Debug,
            4 => DetailLevel::Trace,
            _ => DetailLevel::Trace,
        }
    }
}
