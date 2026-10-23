//! A small exchange client SDK.

/// Why the venue rejected an order.
///
/// A crate using this SDK must not be able to match every variant without a
/// fallback arm, so a new reason in a minor release doesn't break its build:
///
/// ```compile_fail
/// use kata_api_design_64::RejectReason;
///
/// fn code(r: RejectReason) -> u16 {
///     match r {
///         RejectReason::InsufficientFunds => 1,
///         RejectReason::PriceOutOfBand => 2,
///         RejectReason::MarketClosed => 3,
///     }
/// }
/// ```
/// CLUE: Doc tests are compiled as a separate crate. What can another crate do with this enum today that this crate can't take back later?
/// CLUE: Which attribute makes a match outside this crate need a `_` arm, while matches inside it stay exhaustive?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    InsufficientFunds,
    PriceOutOfBand,
    MarketClosed,
}

impl RejectReason {
    pub fn retryable(self) -> bool {
        match self {
            RejectReason::InsufficientFunds | RejectReason::PriceOutOfBand => false,
            RejectReason::MarketClosed => true,
        }
    }
}

/// Something you can place an order on. Only this crate's instruments
/// implement it: `round_price` relies on `tick_size() > 0`, and a later
/// release will add methods.
///
/// ```compile_fail
/// struct Mine;
///
/// impl kata_api_design_64::Instrument for Mine {
///     fn symbol(&self) -> String {
///         "MINE".to_string()
///     }
///     fn tick_size(&self) -> u64 {
///         0
///     }
/// }
/// ```
/// CLUE: A trait another crate can't name can't be implemented there. What if `Instrument` required one?
pub trait Instrument {
    fn symbol(&self) -> String;
    fn tick_size(&self) -> u64;
}

pub struct Spot {
    base: String,
    quote: String,
}

impl Spot {
    pub fn new(base: &str, quote: &str) -> Self {
        Spot {
            base: base.to_string(),
            quote: quote.to_string(),
        }
    }
}

impl Instrument for Spot {
    fn symbol(&self) -> String {
        format!("{}-{}", self.base, self.quote)
    }

    fn tick_size(&self) -> u64 {
        1
    }
}

pub struct Perp {
    underlying: String,
}

impl Perp {
    pub fn new(underlying: &str) -> Self {
        Perp {
            underlying: underlying.to_string(),
        }
    }
}

impl Instrument for Perp {
    fn symbol(&self) -> String {
        format!("{}-PERP", self.underlying)
    }

    fn tick_size(&self) -> u64 {
        5
    }
}

/// Rounds `price` down to the instrument's tick.
pub fn round_price<I: Instrument + ?Sized>(i: &I, price: u64) -> u64 {
    price - price % i.tick_size()
}
