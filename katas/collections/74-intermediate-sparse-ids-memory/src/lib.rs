use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerError {
    /// The new balance doesn't fit in an i64.
    Overflow { account: AccountId },
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerError::Overflow { account } => {
                write!(f, "balance overflow on account {}", account.0)
            }
        }
    }
}

impl std::error::Error for LedgerError {}

/// CLUE: How many bytes does this `Vec` hold after one `apply` to account 4,000,000,000?
/// How many accounts does it hold?
pub struct Ledger {
    balances: Vec<Option<i64>>,
}

impl Ledger {
    /// `accounts` is a hint: how many distinct accounts the caller expects.
    ///
    /// CLUE: The hint says how many *accounts*, not how large an id. Which collection
    /// can use that number directly?
    pub fn with_capacity(accounts: usize) -> Self {
        Ledger {
            balances: Vec::with_capacity(accounts),
        }
    }

    /// Adds `delta` (minor units, may be negative) to the account, creating it at 0
    /// on first sight. Returns the new balance. On overflow the balance is unchanged.
    ///
    /// CLUE: `resize` makes room for every id up to this one. Which of those ids will
    /// ever be used?
    pub fn apply(&mut self, account: AccountId, delta: i64) -> Result<i64, LedgerError> {
        let slot = account.0 as usize;
        if slot >= self.balances.len() {
            self.balances.resize(slot + 1, None);
        }
        let cell = self
            .balances
            .get_mut(slot)
            .ok_or(LedgerError::Overflow { account })?;
        let new = cell
            .unwrap_or(0)
            .checked_add(delta)
            .ok_or(LedgerError::Overflow { account })?;
        *cell = Some(new);
        Ok(new)
    }

    /// None for an account never seen.
    pub fn balance(&self, account: AccountId) -> Option<i64> {
        self.balances.get(account.0 as usize).copied().flatten()
    }

    /// Number of distinct accounts seen.
    pub fn len(&self) -> usize {
        self.balances.iter().filter(|b| b.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
