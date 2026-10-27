use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub id: u64,
    pub price: u64,
}

/// Orders by price, highest first. Orders at the same price keep their arrival order.
/// CLUE: Which rayon sort keeps equal keys in their input order, and which one makes no promise?
pub fn by_price_desc(mut orders: Vec<Order>) -> Vec<Order> {
    orders.par_sort_unstable_by(|a, b| b.price.cmp(&a.price));
    orders
}
