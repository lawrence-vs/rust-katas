//! A limit order book whose price levels are queues of orders.

use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OrderId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fill {
    pub maker: OrderId,
    pub taker: OrderId,
    pub price: u64,
    pub qty: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookError {
    UnknownOrder(OrderId),
}

#[derive(Clone, Copy, Debug)]
struct Order {
    id: OrderId,
    qty: u64,
}

/// CLUE: `cancel` finds its order by walking the level. A level can hold 100k orders. What would let it jump straight to one?
type Level = VecDeque<Order>;

/// CLUE: `slab::Slab` hands out a `usize` key per insert and reuses freed keys. What do `prev` and `next` hold if they are keys, not pointers?
#[derive(Debug, Default)]
pub struct Book {
    bids: BTreeMap<u64, Level>,
    asks: BTreeMap<u64, Level>,
    index: HashMap<OrderId, (Side, u64)>,
}

impl Book {
    pub fn new() -> Book {
        Book::default()
    }

    /// Match against the other side (best price first, oldest first), rest the remainder.
    pub fn submit(&mut self, id: OrderId, side: Side, price: u64, mut qty: u64) -> Vec<Fill> {
        let mut fills = Vec::new();
        while qty > 0 {
            let best = match side {
                Side::Buy => self.asks.first_entry(),
                Side::Sell => self.bids.last_entry(),
            };
            let Some(mut entry) = best else { break };
            let level_price = *entry.key();
            let crosses = match side {
                Side::Buy => level_price <= price,
                Side::Sell => level_price >= price,
            };
            if !crosses {
                break;
            }
            let level = entry.get_mut();
            while qty > 0 {
                let Some(maker) = level.front_mut() else {
                    break;
                };
                let traded = maker.qty.min(qty);
                maker.qty -= traded;
                qty -= traded;
                fills.push(Fill {
                    maker: maker.id,
                    taker: id,
                    price: level_price,
                    qty: traded,
                });
                if maker.qty == 0 {
                    self.index.remove(&maker.id);
                    level.pop_front();
                }
            }
            if level.is_empty() {
                entry.remove();
            }
        }
        if qty > 0 {
            let levels = match side {
                Side::Buy => &mut self.bids,
                Side::Sell => &mut self.asks,
            };
            levels
                .entry(price)
                .or_default()
                .push_back(Order { id, qty });
            self.index.insert(id, (side, price));
        }
        fills
    }

    /// Remove a resting order, return its remaining qty. Unknown, filled or already cancelled id: Err.
    ///
    /// CLUE: A freed slab key is reused by the next order. After a cancel, what must the id index forget, so an old id cannot hit the new owner of the slot?
    pub fn cancel(&mut self, id: OrderId) -> Result<u64, BookError> {
        let unknown = BookError::UnknownOrder(id);
        let &(side, price) = self.index.get(&id).ok_or(unknown)?;
        let levels = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        let level = levels.get_mut(&price).ok_or(unknown)?;
        let pos = level.iter().position(|o| o.id == id).ok_or(unknown)?;
        let order = level.remove(pos).ok_or(unknown)?;
        if level.is_empty() {
            levels.remove(&price);
        }
        self.index.remove(&id);
        Ok(order.qty)
    }

    /// (price, total qty at level) of the highest bid.
    pub fn best_bid(&self) -> Option<(u64, u64)> {
        self.bids
            .last_key_value()
            .map(|(price, level)| (*price, depth(level)))
    }

    /// (price, total qty at level) of the lowest ask.
    pub fn best_ask(&self) -> Option<(u64, u64)> {
        self.asks
            .first_key_value()
            .map(|(price, level)| (*price, depth(level)))
    }

    /// Number of resting orders.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

fn depth(level: &Level) -> u64 {
    level.iter().map(|o| o.qty).sum()
}
