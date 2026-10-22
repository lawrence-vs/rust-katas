/// Quote-currency cents per one whole BTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub u64);

/// Milliseconds since the Unix epoch, from the desk's clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millis(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptError {
    Expired,
    PriceMoved,
}

/// The house's quoting policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Desk {
    /// How long a quote stays good, in milliseconds.
    pub ttl_ms: u64,
    /// How far the market mid may move from the quote's reference mid, in basis points.
    pub max_drift_bps: u32,
}

/// A firm price given to a client. Only `Desk::issue` makes one.
/// CLUE: What does this quote remember about when it was given, or what the market was then?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quote {
    price: Price,
}

impl Desk {
    /// Quote `price` to a client, priced off the market mid `reference` at `now`.
    /// CLUE: `_reference` and `_now` are thrown away. Which of them will `accept` need?
    pub fn issue(&self, price: Price, _reference: Price, _now: Millis) -> Quote {
        Quote { price }
    }

    /// The client accepts `quote` at `now`, while the market mid is `market`.
    /// Returns the fill price.
    /// CLUE: A quote at exactly its deadline: fresh or stale? And what does `+` do near `u64::MAX`?
    /// CLUE: 10_000 times a `u64::MAX` price difference: which integer type holds that?
    pub fn accept(&self, quote: &Quote, market: Price, now: Millis) -> Result<Price, AcceptError> {
        let _ = (market, now);
        Ok(quote.price)
    }
}
