/// `from` atoms a hop's rate is quoted per.
pub const RATE_SCALE: u64 = 100_000_000;
/// Basis points in 100%.
pub const BPS_PER_UNIT: u32 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Asset {
    Btc,
    Usdt,
    Zar,
}

/// An amount in the smallest unit of some asset (satoshis, cents).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Atoms(pub u64);

/// One market: sell `from`, receive `to`. At most one per (from, to) pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hop {
    pub from: Asset,
    pub to: Asset,
    /// `to` atoms per `RATE_SCALE` `from` atoms.
    pub rate: u64,
    /// CLUE: Every hop takes its fee. Where does the float loop take any?
    pub fee_bps: u32,
    /// The smallest `from` amount this market accepts.
    /// CLUE: Is the minimum checked against what you sent, or against what reaches this hop after the last hop's fee?
    pub min_in: Atoms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Direct,
    Via(Asset),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill {
    pub route: Route,
    pub out: Atoms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteError {
    NoRoute,
    BelowMinimum,
    Overflow,
    InvalidFee,
}

/// Convert `amount` of `from` into `to` by the route that gives the most.
///
/// CLUE: The intermediate USDT sits in a real account. Can it hold part of a cent?
/// CLUE: `f64 as u64` never fails. What does it do with 3.4e30?
pub fn best_route(
    markets: &[Hop],
    from: Asset,
    to: Asset,
    amount: Atoms,
) -> Result<Fill, RouteError> {
    let mut best: Option<Fill> = None;
    for (route, hops) in candidates(markets, from, to) {
        let mut x = amount.0 as f64;
        for hop in hops {
            x = x * hop.rate as f64 / RATE_SCALE as f64;
        }
        let out = Atoms(x.round() as u64);
        if best.is_none_or(|b| out.0 > b.out.0) {
            best = Some(Fill { route, out });
        }
    }
    best.ok_or(RouteError::NoRoute)
}

/// Direct first, then one route per intermediate asset in `markets` order.
fn candidates(markets: &[Hop], from: Asset, to: Asset) -> Vec<(Route, Vec<&Hop>)> {
    let find = |a: Asset, b: Asset| markets.iter().find(|h| h.from == a && h.to == b);
    let mut out = Vec::new();
    if let Some(d) = find(from, to) {
        out.push((Route::Direct, vec![d]));
    }
    for first in markets
        .iter()
        .filter(|h| h.from == from && h.to != to && h.to != from)
    {
        if let Some(second) = find(first.to, to) {
            out.push((Route::Via(first.to), vec![first, second]));
        }
    }
    out
}
