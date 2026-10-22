//! An account balance in cents that must never go below zero.

/// An account balance in cents.
/// CLUE: The field is `pub`. What stops a caller writing `Balance(-5)`?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError {
    Negative(i64),
    Insufficient { balance: Balance, requested: u64 },
    Overflow,
}

impl Balance {
    pub const ZERO: Balance = Balance(0);

    /// CLUE: What is `u64::MAX as i64`? Can a deposit make a balance smaller?
    pub fn deposit(self, amount: u64) -> Result<Balance, BalanceError> {
        Ok(Balance(self.0 + amount as i64))
    }

    /// CLUE: Which check runs before the subtraction? Which constructor should the result go through?
    pub fn withdraw(self, amount: u64) -> Result<Balance, BalanceError> {
        Ok(Balance(self.0 - amount as i64))
    }
}

impl TryFrom<i64> for Balance {
    type Error = BalanceError;
    fn try_from(cents: i64) -> Result<Self, Self::Error> {
        Ok(Balance(cents))
    }
}

impl From<Balance> for i128 {
    fn from(b: Balance) -> i128 {
        i128::from(b.0)
    }
}
