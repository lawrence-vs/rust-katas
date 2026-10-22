/// Satoshis per whole BTC.
pub const ATOMS_PER_UNIT: u64 = 100_000_000;
/// Basis points in 100%.
pub const BPS_PER_UNIT: u64 = 10_000;

/// The client's side of the trade.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// Quote-currency cents per one whole BTC.
/// CLUE: `.round()` turns 65.00001 cents into what, and who keeps the part cent on a buy?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub u64);

/// An order size in satoshis.
/// CLUE: Do 100 BTC in satoshis x a price in cents x 10_050 fit in a `u64`? What's the next size up?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size(pub u64);

/// An amount in cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cents(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteError {
    Overflow,
    InvalidSpread,
}

/// What the client pays (buy) or receives (sell) for `size` at `mid`, with the
/// house's spread of `spread_bps` taken off mid on each side.
/// CLUE: Integer `/` rounds toward zero. Which side should that favour, and what rounds the other way?
/// CLUE: `f64 as u64` never fails. What does it do with `1e30` or `-650.0`?
pub fn quote(mid: Price, size: Size, spread_bps: u32, side: Side) -> Result<Cents, QuoteError> {
    let factor = match side {
        Side::Buy => 1.0 + spread_bps as f64 / 10_000.0,
        Side::Sell => 1.0 - spread_bps as f64 / 10_000.0,
    };
    let btc = size.0 as f64 / 100_000_000.0;
    Ok(Cents((mid.0 as f64 * factor * btc).round() as u64))
}
