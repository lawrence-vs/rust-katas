/// A count of whole minor units: ticks, lots, cents. Never a float.
/// CLUE: Diff the five impls below. Apart from the type and the unit, should any line differ?
/// CLUE: `tests/kata.rs` calls `kata_macros_44::impl_amount!` from outside this crate, once with one type and once with a list. Which attribute exports a `macro_rules!`, and which repetition takes a list with an optional trailing comma?
pub trait Amount: Copy {
    /// The unit, as `label` prints it: `"ticks"`, `"lots"`.
    const UNIT: &'static str;

    fn new(minor: u64) -> Self;

    fn minor(self) -> u64;

    /// `None` if the sum doesn't fit in a `u64`.
    fn checked_add(self, other: Self) -> Option<Self>;
}

/// `"101 ticks"`.
pub fn label<A: Amount>(amount: A) -> String {
    format!("{} {}", amount.minor(), A::UNIT)
}

/// The sum of `amounts`, or `None` on overflow. An empty slice sums to zero.
pub fn total<A: Amount>(amounts: &[A]) -> Option<A> {
    amounts
        .iter()
        .try_fold(A::new(0), |sum, &amount| sum.checked_add(amount))
}

/// A price in exchange ticks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Price(u64);

/// A quantity in lots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Qty(u64);

/// Price times quantity, in quote-currency cents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Notional(u64);

/// A fee rate in basis points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fee(u64);

/// A bitcoin amount in satoshis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sats(u64);

/// CLUE: A macro expands where it is called. In the test's `mod outside`, `Amount` isn't in scope. Which path reaches this crate's trait from anywhere?
impl Amount for Price {
    const UNIT: &'static str = "ticks";

    fn new(minor: u64) -> Self {
        Self(minor)
    }

    fn minor(self) -> u64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl Amount for Qty {
    const UNIT: &'static str = "lots";

    fn new(minor: u64) -> Self {
        Self(minor)
    }

    fn minor(self) -> u64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl Amount for Notional {
    const UNIT: &'static str = "cents";

    fn new(minor: u64) -> Self {
        Self(minor)
    }

    fn minor(self) -> u64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl Amount for Fee {
    const UNIT: &'static str = "lots";

    fn new(minor: u64) -> Self {
        Self(minor)
    }

    fn minor(self) -> u64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }
}

impl Amount for Sats {
    const UNIT: &'static str = "sats";

    fn new(minor: u64) -> Self {
        Self(minor)
    }

    fn minor(self) -> u64 {
        self.0
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self(self.0 + other.0))
    }
}
