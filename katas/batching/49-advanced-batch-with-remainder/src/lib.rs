/// A currency pair, e.g. `Pair("BTC-USD")`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair(pub &'static str);

/// A client order id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(pub u64);

/// A quantity in whole lots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Qty(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub pair: Pair,
    pub qty: Qty,
}

/// Lots given to one client order out of a provider fill. Never zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill {
    pub order: OrderId,
    pub qty: Qty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchError {
    /// An order for zero lots.
    ZeroQty,
    /// This order would push the pair's pending total past `u64::MAX`.
    PairOverflow { pair: Pair },
}

/// The liquidity provider the house hedges with.
pub trait LiquidityProvider {
    /// Buys up to `qty` of `pair`; returns how much was filled (at most `qty`, maybe zero).
    ///
    /// CLUE: Every call is a round trip, a fee and a rate-limit slot. How many orders on one pair could share a call?
    fn fill(&mut self, pair: Pair, qty: Qty) -> Qty;
}

pub struct Batcher<L> {
    lp: L,
    /// CLUE: Which orders go out together, and which of them was here first?
    pending: Vec<Order>,
}

impl<L: LiquidityProvider> Batcher<L> {
    pub fn new(lp: L) -> Self {
        Batcher {
            lp,
            pending: Vec::new(),
        }
    }

    pub fn lp(&self) -> &L {
        &self.lp
    }

    /// Queues an order for the next flush.
    ///
    /// CLUE: A flush adds up a pair's orders. What is the biggest total a `u64` holds?
    pub fn submit(&mut self, order: Order) -> Result<(), BatchError> {
        if order.qty.0 == 0 {
            return Err(BatchError::ZeroQty);
        }
        self.pending.push(order);
        Ok(())
    }

    /// Unfilled orders for `pair`, oldest first, with what each is still owed.
    pub fn pending(&self, pair: Pair) -> Vec<(OrderId, Qty)> {
        self.pending
            .iter()
            .filter(|o| o.pair == pair)
            .map(|o| (o.id, o.qty))
            .collect()
    }

    /// Sends pending orders to the provider and returns the fills: pairs in ascending order, oldest order first within a pair.
    ///
    /// CLUE: The provider filled less than you asked. Whose lots are the difference, and where do they live now?
    pub fn flush(&mut self) -> Vec<Fill> {
        let mut fills = Vec::new();
        for order in self.pending.drain(..) {
            let filled = self.lp.fill(order.pair, order.qty);
            fills.push(Fill {
                order: order.id,
                qty: filled,
            });
        }
        fills
    }
}
