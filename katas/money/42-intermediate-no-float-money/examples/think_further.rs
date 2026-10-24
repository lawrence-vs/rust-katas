//! 42 intermediate: no float money
//!
//! Think further, from the kata README:
//! 1. BTC trades in satoshis (8 decimal places) and JPY has no minor unit. What does `Money` need to know so that `from_cents` stays honest across currencies, and what happens if you add two amounts with different scales?
//! 2. Three fills total 1000 cents over 3 units. What is the average price? Which way do you round, and where does the remainder go?
//! 3. A P&L goes negative. Do you switch to `i64`, or keep `u64` and add a `checked_sub` that returns an `InsufficientFunds` error? Which one stops an account from being overdrawn?
//! 4. A price arrives on the wire as the string `"0.10"`. How do you parse it into cents without ever going through `f64`?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(u64);

#[derive(Debug, PartialEq, Eq)]
pub enum MoneyError {
    Overflow,
    InsufficientFunds,
    BadAmount(String),
}

impl Money {
    pub fn from_cents(cents: u64) -> Self {
        Money(cents)
    }

    pub fn cents(self) -> u64 {
        self.0
    }

    // checked_add turns overflow into an error the caller must handle.
    pub fn checked_add(self, other: Money) -> Result<Money, MoneyError> {
        self.0
            .checked_add(other.0)
            .map(Money)
            .ok_or(MoneyError::Overflow)
    }

    // Subtracting more than the balance is an error, so an account can't go negative.
    pub fn checked_sub(self, other: Money) -> Result<Money, MoneyError> {
        self.0
            .checked_sub(other.0)
            .map(Money)
            .ok_or(MoneyError::InsufficientFunds)
    }

    pub fn checked_mul(self, qty: u64) -> Result<Money, MoneyError> {
        self.0
            .checked_mul(qty)
            .map(Money)
            .ok_or(MoneyError::Overflow)
    }
}

// Think further 2: integer division floors, so the remainder must go somewhere on purpose.
fn split_evenly(total: u64, units: u64) -> Vec<u64> {
    if units == 0 {
        return Vec::new();
    }
    let base = total / units;
    let remainder = total % units;
    (0..units)
        .map(|i| base + u64::from(i < remainder))
        .collect()
}

// Think further 4: parse "1.5" into cents with integers only, never through f64.
fn parse_dollars(s: &str) -> Result<Money, MoneyError> {
    let bad = || MoneyError::BadAmount(s.to_string());
    let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
    if frac.len() > 2 {
        return Err(bad());
    }
    let whole: u64 = whole.parse().map_err(|_| bad())?;
    // Pad the fraction on the right: "5" means 50 cents, "" means 0.
    let cents: u64 = format!("{frac:0<2}").parse().map_err(|_| bad())?;
    whole
        .checked_mul(100)
        .and_then(|w| w.checked_add(cents))
        .map(Money)
        .ok_or(MoneyError::Overflow)
}

fn main() {
    // Think further 3: a balance that would go negative.
    let balance = Money::from_cents(500);
    println!(
        "sub 1000 from 500: {:?}",
        balance.checked_sub(Money::from_cents(1000))
    );

    // Overflow is an error, not a wrap or a panic.
    println!(
        "max + 1: {:?}",
        Money::from_cents(u64::MAX).checked_add(Money::from_cents(1))
    );
    println!(
        "1999 cents x 3: {:?}",
        Money::from_cents(1999).checked_mul(3)
    );

    // Think further 2: three fills of 1000 cents across 3 units.
    println!("split 1000 into 3: {:?}", split_evenly(1000, 3));

    // Think further 4: strings in, cents out.
    for s in ["0.10", "1.5", "10", "0.001", "-1.00", ".5"] {
        println!("{s:>6} => {:?}", parse_dollars(s));
    }

    // In the wild: std's Duration uses the same checked pattern for whole seconds plus nanoseconds.
    println!(
        "duration overflow: {:?}",
        Duration::from_secs(u64::MAX).checked_add(Duration::from_secs(1))
    );
}
