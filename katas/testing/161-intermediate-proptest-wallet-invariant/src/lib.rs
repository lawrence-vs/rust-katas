//! A wallet that sets funds aside for pending payments.

/// A wallet in cents. `free` can be spent or reserved, `reserved` is held for
/// a pending payment, and `total` is stored so a balance read is one field.
/// CLUE: Three numbers, one rule between them. Which methods change two of them, and which change all three?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Wallet {
    free: u64,
    reserved: u64,
    total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletError {
    Insufficient { needed: u64, available: u64 },
    Overflow,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet::default()
    }

    pub fn free(&self) -> u64 {
        self.free
    }

    pub fn reserved(&self) -> u64 {
        self.reserved
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn deposit(&mut self, amount: u64) -> Result<(), WalletError> {
        let total = self
            .total
            .checked_add(amount)
            .ok_or(WalletError::Overflow)?;
        let free = self.free.checked_add(amount).ok_or(WalletError::Overflow)?;
        self.total = total;
        self.free = free;
        Ok(())
    }

    pub fn reserve(&mut self, amount: u64) -> Result<(), WalletError> {
        if amount > self.free {
            return Err(WalletError::Insufficient {
                needed: amount,
                available: self.free,
            });
        }
        self.free -= amount;
        self.reserved += amount;
        Ok(())
    }

    /// CLUE: What does `saturating_sub` do when less is reserved than asked? Where did the missing part come from?
    pub fn release(&mut self, amount: u64) -> Result<(), WalletError> {
        self.reserved = self.reserved.saturating_sub(amount);
        self.free = self.free.saturating_add(amount);
        Ok(())
    }

    /// CLUE: Which pot does a spend come out of, and which pot does the check look at?
    pub fn spend(&mut self, amount: u64) -> Result<(), WalletError> {
        if amount > self.total {
            return Err(WalletError::Insufficient {
                needed: amount,
                available: self.total,
            });
        }
        self.total -= amount;
        self.free = self.free.saturating_sub(amount);
        Ok(())
    }
}
