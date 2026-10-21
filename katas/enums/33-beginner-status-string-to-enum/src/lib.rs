/// Whether an order with this status can still trade.
/// CLUE: `"rejected"`, `"Rejected"` and `"rejectd"` all return `false` here. Which branch decides each one, and would you have noticed?
/// CLUE: Next month someone adds `"expired"`. Which line of this file makes them come back and update this function?
pub fn is_open(status: &str) -> bool {
    if status == "new" || status == "partially_filled" {
        true
    } else if status == "filled" || status == "cancelled" || status == "rejected" {
        false
    } else {
        false
    }
}

/// Whether any quantity of the order has traded.
/// CLUE: Compare the strings here with the ones in `is_open`. Which one is spelt differently, and why did nothing complain?
pub fn has_fills(status: &str) -> bool {
    if status == "partially-filled" || status == "filled" {
        true
    } else {
        false
    }
}
