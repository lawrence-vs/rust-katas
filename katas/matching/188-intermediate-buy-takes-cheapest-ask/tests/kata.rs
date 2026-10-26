use std::collections::{BTreeMap, VecDeque};

use kata_matching_188::{Fill, Order, match_buy};

#[test]
fn buy_takes_cheapest_ask_first() {
    let mut asks = BTreeMap::new();
    asks.insert(
        101,
        VecDeque::from([Order {
            id: 1,
            price: 101,
            qty: 1,
        }]),
    );
    asks.insert(
        103,
        VecDeque::from([Order {
            id: 2,
            price: 103,
            qty: 1,
        }]),
    );
    let mut buy = Order {
        id: 9,
        price: 105,
        qty: 2,
    };

    let fills = match_buy(&mut asks, &mut buy);

    assert_eq!(
        fills,
        [
            Fill {
                maker: 1,
                taker: 9,
                price: 101,
                qty: 1
            },
            Fill {
                maker: 2,
                taker: 9,
                price: 103,
                qty: 1
            },
        ]
    );
    assert!(asks.is_empty());
}
