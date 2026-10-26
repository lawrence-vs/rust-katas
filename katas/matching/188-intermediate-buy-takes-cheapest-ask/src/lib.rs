use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub id: u64,
    pub price: u64,
    pub qty: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fill {
    pub maker: u64,
    pub taker: u64,
    pub price: u64,
    pub qty: u64,
}

/// Match a buy against resting asks: cheapest price first, then oldest order at that price.
/// CLUE: The asks are keyed by price in ascending order. A buyer wants the lowest ask. Which end of the map is that?
pub fn match_buy(asks: &mut BTreeMap<u64, VecDeque<Order>>, buy: &mut Order) -> Vec<Fill> {
    let mut fills = Vec::new();
    for (&price, level) in asks.iter_mut().rev() {
        if price > buy.price {
            break;
        }
        while buy.qty > 0 {
            let Some(maker) = level.front_mut() else {
                break;
            };
            let qty = maker.qty.min(buy.qty);
            fills.push(Fill {
                maker: maker.id,
                taker: buy.id,
                price,
                qty,
            });
            maker.qty -= qty;
            buy.qty -= qty;
            if maker.qty == 0 {
                level.pop_front();
            }
        }
    }
    asks.retain(|_, level| !level.is_empty());
    fills
}
