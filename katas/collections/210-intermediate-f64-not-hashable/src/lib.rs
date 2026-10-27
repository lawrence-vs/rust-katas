use std::collections::HashSet;

/// One price level on the book. Levels are de-duplicated by price and quantity.
/// CLUE: Which traits does `HashSet` need on its element, and can `f64` provide each one?
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PriceLevel {
    pub price: f64,
    pub qty: u64,
}

/// Count the distinct price levels in a batch.
pub fn distinct_levels(levels: Vec<PriceLevel>) -> usize {
    let set: HashSet<PriceLevel> = levels.into_iter().collect();
    set.len()
}
