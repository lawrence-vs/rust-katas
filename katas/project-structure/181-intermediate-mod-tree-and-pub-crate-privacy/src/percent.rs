use std::fmt;

use crate::money::share;

/// CLUE: Percent::new checks the value. What other way into a Percent does a caller in another crate have today?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percent {
    pub value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PercentError {
    OverHundred(u8),
}

impl fmt::Display for PercentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PercentError::OverHundred(v) => write!(f, "{v}% is over 100%"),
        }
    }
}

impl std::error::Error for PercentError {}

impl Percent {
    pub fn new(value: u8) -> Result<Percent, PercentError> {
        if value > 100 {
            return Err(PercentError::OverHundred(value));
        }
        Ok(Percent { value })
    }

    pub fn get(self) -> u8 {
        self.value
    }

    pub fn of(self, cents: u64) -> u64 {
        share(cents, self.value)
    }
}
