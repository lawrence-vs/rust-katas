/// Which way a fill went.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// One executed trade: `qty` units on one `side`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill {
    pub side: Side,
    pub qty: u64,
}

/// Total quantity bought across `fills`.
/// CLUE: How many places in this function go wrong if `i` is one too big, or starts one too late?
/// CLUE: `fills.iter()` hands you each `&Fill` exactly once. Which bound is left for you to write?
/// CLUE: Which iterator methods keep some items, turn each into another value, and add them up?
pub fn bought_qty(fills: &[Fill]) -> u64 {
    let mut total = 0;
    let mut i = 0;
    while i <= fills.len() {
        if fills[i].side == Side::Buy {
            total += fills[i].qty;
        }
        i += 1;
    }
    total
}
