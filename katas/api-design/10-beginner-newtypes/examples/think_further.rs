//! 10 beginner: newtypes, not primitives
//!
//! Think further, from the kata README:
//! 1. Call `transfer` from an account that isn't in the map, and with an amount bigger than the sender's balance. What happens in each case in a debug build, and in `--release`? Which of these should the caller get back as a value? (Kata 42 covers the arithmetic.)
//! 2. Transfer from `alice` to `alice`. Is the result what a bank would expect? Would you reject it, allow it, or make it impossible to write?
//! 3. The test builds `AccountId(1)` directly, so the field is public. What could a caller do with a public `Cents` field that a private field and a constructor would stop? What would the test call instead?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::mem::size_of;
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AccountId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Cents(u64);

impl Cents {
    pub fn new(value: u64) -> Self {
        Cents(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

// Think further 3: a private field and a constructor keep a caller from building an invalid value.
fn place(price: Cents, qty: u64) -> String {
    format!("price {} qty {qty}", price.get())
}

fn main() {
    // In the wild: the newtype has the same size as the number it wraps.
    println!("AccountId is {} bytes, u64 is {}", size_of::<AccountId>(), size_of::<u64>());

    // In the wild: NonZero makes Option free: the zero value is the None.
    println!(
        "Option<NonZeroU64> is {} bytes, Option<u64> is {}",
        size_of::<Option<NonZeroU64>>(),
        size_of::<Option<u64>>()
    );

    // The newtype stops swapped arguments: place(qty, price) would not compile.
    println!("{}", place(Cents::new(1_999), 3));

    // Think further 1: a missing account is a value the caller gets back, not a panic.
    let accounts = [AccountId(1), AccountId(2)];
    let found = accounts.iter().find(|id| **id == AccountId(9));
    println!("account 9 found: {found:?}");

    // Think further 2: a transfer to the same account should be refused, not silently accepted.
    let (from, to) = (AccountId(1), AccountId(1));
    if from == to {
        println!("refused: transfer to the same account");
    }
}
