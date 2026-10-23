/// Whether an order with this status can still trade.
/// CLUE: `"rejected"`, `"Rejected"` and `"rejectd"` all return `false` here. Which branch decides each one, and would you have noticed?
/// CLUE: Next month someone adds `"expired"`. Which line of this file makes them come back and update this function?

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

impl OrderStatus {
    pub fn is_open(&self) -> bool {
        match self {
            OrderStatus::New | OrderStatus::PartiallyFilled => true,
            OrderStatus::Filled | OrderStatus::Cancelled | OrderStatus::Rejected => false,
        }
    }
    /// Whether any quantity of the order has traded.
    /// CLUE: Compare the strings here with the ones in `is_open`. Which one is spelt differently, and why did nothing complain?
    pub fn has_fills(&self) -> bool {
        match self {
            OrderStatus::Filled | OrderStatus::PartiallyFilled => true,
            OrderStatus::Cancelled | OrderStatus::Rejected | OrderStatus::New => false,
        }
    }
}

pub fn parse_status(status: &str) -> Option<OrderStatus> {
    match status {
        "new" => Some(OrderStatus::New),
        "partially_filled" => Some(OrderStatus::PartiallyFilled),
        "filled" => Some(OrderStatus::Filled),
        "cancelled" => Some(OrderStatus::Cancelled),
        "rejected" => Some(OrderStatus::Rejected),
        _ => None,
    }
}
