use kata_rayon_208::{Order, by_price_desc};

#[test]
fn equal_prices_keep_arrival_order() {
    let orders: Vec<Order> = (0..10_000)
        .map(|i| Order {
            id: i,
            price: i % 10,
        })
        .collect();

    let mut expected = orders.clone();
    expected.sort_by(|a, b| b.price.cmp(&a.price));

    assert_eq!(by_price_desc(orders), expected);
}
