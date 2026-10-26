//! A cash account on an exchange. Placing a buy holds money, each fill spends
//! part of the hold, and a cancel gives back what the order still holds.

use std::collections::HashMap;
use std::fmt;

/// Money in minor units (cents). Never a float.
pub type Cents = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

/// `New` is an order that holds nothing yet. `place` takes the hold and moves it
/// to `Open` in the same step, so no stored order is ever `New`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Buy `qty` units at exactly `price` cents each.
    Limit { price: Cents, qty: u64 },
    /// Buy at any price until `budget` cents are used.
    Market { budget: Cents },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill {
    pub qty: u64,
    pub cost: Cents,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountError {
    Insufficient {
        needed: Cents,
        available: Cents,
    },
    UnknownOrder(OrderId),
    NotActive {
        id: OrderId,
        status: Status,
    },
    WrongPrice {
        id: OrderId,
        limit: Cents,
        got: Cents,
    },
    OverFill {
        id: OrderId,
        remaining: u64,
        got: u64,
    },
    ZeroQuantity,
    ZeroPrice,
    Overflow,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountError::Insufficient { needed, available } => {
                write!(f, "need {needed} cents, only {available} available")
            }
            AccountError::UnknownOrder(id) => write!(f, "no order {}", id.0),
            AccountError::NotActive { id, status } => {
                write!(f, "order {} is {status:?}, not open", id.0)
            }
            AccountError::WrongPrice { id, limit, got } => {
                write!(f, "order {} is a limit at {limit}, fill at {got}", id.0)
            }
            AccountError::OverFill { id, remaining, got } => {
                write!(f, "order {} has {remaining} left, fill of {got}", id.0)
            }
            AccountError::ZeroQuantity => write!(f, "quantity or budget is zero"),
            AccountError::ZeroPrice => write!(f, "price is zero"),
            AccountError::Overflow => write!(f, "amount overflows u64"),
        }
    }
}

impl std::error::Error for AccountError {}

#[derive(Debug, Clone)]
struct Order {
    kind: Kind,
    /// CLUE: Set when the order is placed. After 3 of 10 fill, what should it say?
    held: Cents,
    filled_qty: u64,
    status: Status,
}

#[derive(Debug, Default)]
pub struct Account {
    available: Cents,
    reserved: Cents,
    spent: Cents,
    next_id: u64,
    orders: HashMap<OrderId, Order>,
}

impl Account {
    pub fn new(deposit: Cents) -> Account {
        Account {
            available: deposit,
            ..Account::default()
        }
    }

    pub fn available(&self) -> Cents {
        self.available
    }

    pub fn reserved(&self) -> Cents {
        self.reserved
    }

    pub fn spent(&self) -> Cents {
        self.spent
    }

    pub fn status(&self, id: OrderId) -> Option<Status> {
        self.orders.get(&id).map(|o| o.status)
    }

    /// What this order still holds.
    pub fn held(&self, id: OrderId) -> Option<Cents> {
        self.orders.get(&id).map(|o| o.held)
    }

    pub fn place(&mut self, kind: Kind) -> Result<OrderId, AccountError> {
        let hold = match kind {
            Kind::Limit { price, qty } => {
                if qty == 0 {
                    return Err(AccountError::ZeroQuantity);
                }
                if price == 0 {
                    return Err(AccountError::ZeroPrice);
                }
                price.checked_mul(qty).ok_or(AccountError::Overflow)?
            }
            Kind::Market { budget } => {
                if budget == 0 {
                    return Err(AccountError::ZeroQuantity);
                }
                budget
            }
        };
        let available = self
            .available
            .checked_sub(hold)
            .ok_or(AccountError::Insufficient {
                needed: hold,
                available: self.available,
            })?;
        let reserved = self
            .reserved
            .checked_add(hold)
            .ok_or(AccountError::Overflow)?;
        let id = OrderId(self.next_id);
        let next_id = self.next_id.checked_add(1).ok_or(AccountError::Overflow)?;
        // New -> Open: the hold is taken in the same step the order is accepted.
        self.available = available;
        self.reserved = reserved;
        self.next_id = next_id;
        self.orders.insert(
            id,
            Order {
                kind,
                held: hold,
                filled_qty: 0,
                status: Status::Open,
            },
        );
        Ok(id)
    }

    pub fn fill(&mut self, id: OrderId, qty: u64, price: Cents) -> Result<Fill, AccountError> {
        if qty == 0 {
            return Err(AccountError::ZeroQuantity);
        }
        if price == 0 {
            return Err(AccountError::ZeroPrice);
        }
        let order = self
            .orders
            .get_mut(&id)
            .ok_or(AccountError::UnknownOrder(id))?;
        if !matches!(order.status, Status::Open | Status::PartiallyFilled) {
            return Err(AccountError::NotActive {
                id,
                status: order.status,
            });
        }
        let take = match order.kind {
            Kind::Limit {
                price: limit,
                qty: total,
            } => {
                if price != limit {
                    return Err(AccountError::WrongPrice {
                        id,
                        limit,
                        got: price,
                    });
                }
                let remaining = total
                    .checked_sub(order.filled_qty)
                    .ok_or(AccountError::Overflow)?;
                if qty > remaining {
                    return Err(AccountError::OverFill {
                        id,
                        remaining,
                        got: qty,
                    });
                }
                qty
            }
            // A market buy takes only as many units as its hold still pays for.
            Kind::Market { .. } => qty.min(
                order
                    .held
                    .checked_div(price)
                    .ok_or(AccountError::ZeroPrice)?,
            ),
        };
        let cost = take.checked_mul(price).ok_or(AccountError::Overflow)?;
        let filled_qty = order
            .filled_qty
            .checked_add(take)
            .ok_or(AccountError::Overflow)?;
        let done = match order.kind {
            Kind::Limit { qty: total, .. } => filled_qty == total,
            Kind::Market { .. } => take < qty || cost == order.held,
        };
        // CLUE: `reserved` drops by `cost` here. Which other number should drop with it?
        let mut reserved = self
            .reserved
            .checked_sub(cost)
            .ok_or(AccountError::Overflow)?;
        let spent = self.spent.checked_add(cost).ok_or(AccountError::Overflow)?;
        let mut available = self.available;
        let mut status = Status::PartiallyFilled;
        if done {
            // A finished order gives back whatever it still holds.
            reserved = reserved
                .checked_sub(order.held)
                .ok_or(AccountError::Overflow)?;
            available = available
                .checked_add(order.held)
                .ok_or(AccountError::Overflow)?;
            status = Status::Filled;
        }
        self.available = available;
        self.reserved = reserved;
        self.spent = spent;
        order.filled_qty = filled_qty;
        order.status = status;
        if done {
            order.held = 0;
        }
        Ok(Fill {
            qty: take,
            cost,
            status,
        })
    }

    pub fn cancel(&mut self, id: OrderId) -> Result<Cents, AccountError> {
        let order = self
            .orders
            .get_mut(&id)
            .ok_or(AccountError::UnknownOrder(id))?;
        let release = match order.status {
            Status::Open => order.held,
            // CLUE: Part filled is not all filled. Where is the rest of the hold now?
            Status::PartiallyFilled => 0,
            status => return Err(AccountError::NotActive { id, status }),
        };
        let reserved = self
            .reserved
            .checked_sub(release)
            .ok_or(AccountError::Overflow)?;
        let available = self
            .available
            .checked_add(release)
            .ok_or(AccountError::Overflow)?;
        self.reserved = reserved;
        self.available = available;
        order.held = 0;
        order.status = Status::Cancelled;
        Ok(release)
    }
}
