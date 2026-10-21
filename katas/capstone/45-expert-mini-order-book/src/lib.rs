//! A tiny limit order book: resting orders, matching and cancels.

/// Buy or sell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// The caller's id for an order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OrderId(pub u64);

/// A limit price. One tick is one cent.
/// CLUE: What does `from_ticks(115).ticks()` return? Work out `1.15 * 100.0` in an `f64`.
/// CLUE: A `BTreeMap` key must be `Ord`. Why is `f64` only `PartialOrd`?
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Price(f64);

impl Price {
    pub fn from_ticks(ticks: u64) -> Price {
        Price(ticks as f64 / 100.0)
    }

    pub fn ticks(self) -> u64 {
        (self.0 * 100.0) as u64
    }
}

/// One trade between a resting order (maker) and an incoming one (taker).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fill {
    pub maker: OrderId,
    pub taker: OrderId,
    pub price: Price,
    pub qty: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookError {
    UnknownOrder(OrderId),
}

#[derive(Clone, Copy, Debug)]
struct Resting {
    id: OrderId,
    side: Side,
    price: Price,
    qty: u64,
}

/// CLUE: Every call scans every order. Which std map keeps its keys sorted, so the best price is its first or last entry?
#[derive(Debug, Default)]
pub struct Book {
    orders: Vec<Resting>,
}

impl Book {
    pub fn new() -> Book {
        Book::default()
    }

    /// Matches `qty` against the other side, then rests whatever is left.
    /// CLUE: At one price, who trades first: the oldest order, or whichever the scan meets first?
    pub fn submit(&mut self, id: OrderId, side: Side, price: Price, qty: u64) -> Vec<Fill> {
        let crosses = |r: &Resting| match side {
            Side::Buy => r.side == Side::Sell && r.price <= price,
            Side::Sell => r.side == Side::Buy && r.price >= price,
        };
        if let Some(i) = self.orders.iter().position(crosses) {
            let maker = &mut self.orders[i];
            let traded = maker.qty.min(qty);
            maker.qty -= traded;
            let fill = Fill {
                maker: maker.id,
                taker: id,
                price: maker.price,
                qty: traded,
            };
            if maker.qty == 0 {
                self.orders.remove(i);
            }
            return vec![fill];
        }
        self.orders.push(Resting {
            id,
            side,
            price,
            qty,
        });
        Vec::new()
    }

    /// Removes a resting order and returns the quantity it still had.
    pub fn cancel(&mut self, id: OrderId) -> Result<u64, BookError> {
        let qty = self
            .orders
            .iter()
            .filter(|r| r.id == id)
            .map(|r| r.qty)
            .sum::<u64>();
        self.orders.retain(|r| r.id != id);
        Ok(qty)
    }

    pub fn best_bid(&self) -> Option<(Price, u64)> {
        self.best(Side::Buy)
    }

    pub fn best_ask(&self) -> Option<(Price, u64)> {
        self.best(Side::Sell)
    }

    fn best(&self, side: Side) -> Option<(Price, u64)> {
        let mut best: Option<(Price, u64)> = None;
        for r in self.orders.iter().filter(|r| r.side == side) {
            best = match best {
                None => Some((r.price, r.qty)),
                Some((p, q)) if r.price == p => Some((p, q + r.qty)),
                Some((p, _))
                    if (side == Side::Buy && r.price > p)
                        || (side == Side::Sell && r.price < p) =>
                {
                    Some((r.price, r.qty))
                }
                keep => keep,
            };
        }
        best
    }
}
