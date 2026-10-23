//! 53 intermediate: a balance that never goes negative
//!
//! Think further, from the kata README:
//! 1. Should `proptest-regressions` files be committed in a real project? What does this kata's `failure_persistence: None` trade away?
//! 2. `TryFrom<i64>` or `Balance::new(i64) -> Option<Balance>`: what does each give the caller?
//! 3. How would a `Ledger` of many accounts state "money is conserved" as a property?
//! 4. What would `cargo test --release` do to the starting `deposit`, and would any test notice?
//! 5. How would serde's `#[serde(try_from = "i64")]` keep the invariant at the JSON boundary? (Kata 57 picks this up.)

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Balance(u64);

#[derive(Debug, PartialEq, Eq)]
enum BalanceError {
    Insufficient,
}

impl Balance {
    // Think further 2: a checked constructor that the caller can match on, the same idea as NonZero.
    fn new(cents: i64) -> Option<Balance> {
        u64::try_from(cents).ok().map(Balance)
    }

    fn deposit(self, cents: u64) -> Option<Balance> {
        self.0.checked_add(cents).map(Balance)
    }

    fn withdraw(self, cents: u64) -> Result<Balance, BalanceError> {
        self.0.checked_sub(cents).map(Balance).ok_or(BalanceError::Insufficient)
    }
}

// Think further 3: money is conserved across a transfer; the total before equals the total after.
fn transfer(from: Balance, to: Balance, cents: u64) -> Result<(Balance, Balance), BalanceError> {
    let from = from.withdraw(cents)?;
    let to = to.deposit(cents).ok_or(BalanceError::Insufficient)?;
    Ok((from, to))
}

fn main() {
    println!("new(-5): {:?}", Balance::new(-5));
    println!("new(500): {:?}", Balance::new(500));

    let a = Balance(500);
    let b = Balance(0);
    println!("overdraw: {:?}", transfer(a, b, 501));

    if let Ok((a2, b2)) = transfer(a, b, 200) {
        println!("after transfer: {} + {} = {}", a2.0, b2.0, a2.0 + b2.0);
        println!("conserved: {}", a2.0 + b2.0 == a.0 + b.0);
    }
}
