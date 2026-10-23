//! 33 beginner: a status string to an enum
//!
//! Think further, from the kata README:
//! 1. Implement `FromStr` instead of `parse_status`. What does `"filled".parse::<OrderStatus>()` give you, and what error type would you pick? (Kata 09: `None` says something failed, not what.)
//! 2. The status also goes back out on the wire. Write the signature of an `as_str` method. How would you test that every variant round-trips through `parse_status`?
//! 3. An order is cancelled after a partial fill. What does a bare `Cancelled` lose, and what would `Cancelled { filled: u64 }` change in `has_fills`?
//! 4. A library marks its enum `#[non_exhaustive]`. What must a `match` in another crate write, and what does that crate lose from question 2?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, PartialEq, Eq)]
struct UnknownStatus(String);

// Lets callers write "filled".parse::<OrderStatus>() instead of a free function.
impl FromStr for OrderStatus {
    type Err = UnknownStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(OrderStatus::New),
            "partially_filled" => Ok(OrderStatus::PartiallyFilled),
            "filled" => Ok(OrderStatus::Filled),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "rejected" => Ok(OrderStatus::Rejected),
            _ => Err(UnknownStatus(s.to_string())),
        }
    }
}

impl OrderStatus {
    // The reverse of from_str; every variant named, so a new one is a compile error here.
    fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::New => "new",
            OrderStatus::PartiallyFilled => "partially_filled",
            OrderStatus::Filled => "filled",
            OrderStatus::Cancelled => "cancelled",
            OrderStatus::Rejected => "rejected",
        }
    }
}

// Carries the fill count, so a cancel after a partial fill keeps what traded.
#[derive(Debug, PartialEq, Eq)]
enum Detailed {
    New,
    PartiallyFilled { filled: u64 },
    Filled { filled: u64 },
    Cancelled { filled: u64 },
    Rejected,
}

impl Detailed {
    fn has_fills(&self) -> bool {
        match self {
            Detailed::New | Detailed::Rejected => false,
            Detailed::PartiallyFilled { filled }
            | Detailed::Filled { filled }
            | Detailed::Cancelled { filled } => *filled > 0,
        }
    }
}

const ALL: [OrderStatus; 5] = [
    OrderStatus::New,
    OrderStatus::PartiallyFilled,
    OrderStatus::Filled,
    OrderStatus::Cancelled,
    OrderStatus::Rejected,
];

fn main() {
    // 1. FromStr: parse at the edge, get a reason back on failure.
    assert_eq!("filled".parse::<OrderStatus>(), Ok(OrderStatus::Filled));
    assert_eq!(
        "expired".parse::<OrderStatus>(),
        Err(UnknownStatus("expired".to_string()))
    );

    // 2. as_str and a round trip over every variant.
    for status in ALL {
        assert_eq!(status.as_str().parse::<OrderStatus>(), Ok(status));
    }
    println!("round trip ok for {} variants", ALL.len());

    // 3. Carrying the fill count changes has_fills for a cancel after a partial fill.
    let cancelled_early = Detailed::Cancelled { filled: 0 };
    let cancelled_late = Detailed::Cancelled { filled: 40 };
    println!("cancelled early has fills: {}", cancelled_early.has_fills());
    println!("cancelled late has fills: {}", cancelled_late.has_fills());
    for d in [
        Detailed::New,
        Detailed::PartiallyFilled { filled: 10 },
        Detailed::Filled { filled: 100 },
        Detailed::Rejected,
    ] {
        println!("{d:?} has fills: {}", d.has_fills());
    }

    // 4. #[non_exhaustive] only bites in another crate: there, a match must add `_`.
}
