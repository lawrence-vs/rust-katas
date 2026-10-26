use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookError {
    UnknownId,
    DuplicateId,
    ZeroQty,
    WouldCross,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Order {
    id: u64,
    qty: u64,
    seq: u64,
}

/// Price-time priority: each price holds a queue, oldest order at the front.
#[derive(Debug, Default)]
pub struct Book {
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
    index: HashMap<u64, (Side, u64)>,
    next_seq: u64,
}

impl Book {
    pub fn new() -> Book {
        Book::default()
    }

    pub fn place(&mut self, id: u64, side: Side, price: u64, qty: u64) -> Result<u64, BookError> {
        if qty == 0 {
            return Err(BookError::ZeroQty);
        }
        if self.index.contains_key(&id) {
            return Err(BookError::DuplicateId);
        }
        if self.would_cross(side, price) {
            return Err(BookError::WouldCross);
        }
        let seq = self.next_seq();
        self.side_mut(side)
            .entry(price)
            .or_default()
            .push_back(Order { id, qty, seq });
        self.index.insert(id, (side, price));
        Ok(seq)
    }

    /// Change an order's price or quantity. Returns the sequence number it holds afterwards.
    /// CLUE: A size decrease frees room ahead of nobody. What does an increase take from the orders behind it?
    /// CLUE: What number says who arrived first? Does it change when the order does?
    /// CLUE: Which end of the `VecDeque` is the back of the queue?
    pub fn amend(&mut self, id: u64, new_price: u64, new_qty: u64) -> Result<u64, BookError> {
        if new_qty == 0 {
            return Err(BookError::ZeroQty);
        }
        let Some(&(side, old_price)) = self.index.get(&id) else {
            return Err(BookError::UnknownId);
        };
        if self.would_cross(side, new_price) {
            return Err(BookError::WouldCross);
        }
        let Some((pos, mut order)) = self.take(side, old_price, id) else {
            return Err(BookError::UnknownId);
        };
        let seq = order.seq;
        order.qty = new_qty;
        if new_price == old_price {
            self.side_mut(side)
                .entry(old_price)
                .or_default()
                .insert(pos, order);
        } else {
            self.side_mut(side)
                .entry(new_price)
                .or_default()
                .push_front(order);
            self.index.insert(id, (side, new_price));
        }
        Ok(seq)
    }

    pub fn cancel(&mut self, id: u64) -> Result<(), BookError> {
        let Some((side, price)) = self.index.remove(&id) else {
            return Err(BookError::UnknownId);
        };
        self.take(side, price, id);
        Ok(())
    }

    /// The orders at one price, front of the queue first, as (id, qty, seq).
    pub fn queue(&self, side: Side, price: u64) -> Vec<(u64, u64, u64)> {
        self.side(side)
            .get(&price)
            .map(|level| level.iter().map(|o| (o.id, o.qty, o.seq)).collect())
            .unwrap_or_default()
    }

    pub fn best_bid(&self) -> Option<u64> {
        self.bids.last_key_value().map(|(&price, _)| price)
    }

    pub fn best_ask(&self) -> Option<u64> {
        self.asks.first_key_value().map(|(&price, _)| price)
    }

    fn side(&self, side: Side) -> &BTreeMap<u64, VecDeque<Order>> {
        match side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        }
    }

    fn side_mut(&mut self, side: Side) -> &mut BTreeMap<u64, VecDeque<Order>> {
        match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        }
    }

    fn would_cross(&self, side: Side, price: u64) -> bool {
        match side {
            Side::Buy => self.best_ask().is_some_and(|ask| price >= ask),
            Side::Sell => self.best_bid().is_some_and(|bid| price <= bid),
        }
    }

    fn next_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// Remove an order from its level, returning its position. Drops the level when it empties.
    fn take(&mut self, side: Side, price: u64, id: u64) -> Option<(usize, Order)> {
        let levels = self.side_mut(side);
        let level = levels.get_mut(&price)?;
        let pos = level.iter().position(|o| o.id == id)?;
        let order = level.remove(pos)?;
        if level.is_empty() {
            levels.remove(&price);
        }
        Some((pos, order))
    }
}
