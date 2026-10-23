/// MoneyError is what a checked money operation returns when it cannot give a correct amount.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoneyError {
    Overflow,
}

/// Money is a whole number of cents, because binary floating point cannot hold 0.1 exactly.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Money {
    cents: u64,
}

impl Money {
    pub fn from_cents(cents: u64) -> Self {
        Money { cents }
    }

    pub fn cents(&self) -> u64 {
        self.cents
    }

    pub fn add(&self, val: Money) -> Result<Money, MoneyError> {
        self.cents
            .checked_add(val.cents)
            .ok_or(MoneyError::Overflow)
            .map(Money::from_cents)
    }

    pub fn mul(&self, val: u64) -> Result<Money, MoneyError> {
        self.cents
            .checked_mul(val)
            .ok_or(MoneyError::Overflow)
            .map(Money::from_cents)
    }
}

/// Sums a list of prices.
/// CLUE: `u64` has `checked_add`. What does it return that `+` doesn't?
pub fn total(prices: &[Money]) -> Result<Money, MoneyError> {
    prices
        .iter()
        .try_fold(Money::from_cents(0), |acc, i| acc.add(*i))
}

/// The cost of filling `qty` units at `price`.
pub fn fill_cost(price: Money, qty: u64) -> Result<Money, MoneyError> {
    price.mul(qty)
}
