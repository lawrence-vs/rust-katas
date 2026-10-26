use kata_collections_195::Asks;

#[test]
fn pops_lowest_price_first() {
    let mut asks = Asks::new();
    for price in [103, 101, 105] {
        asks.push(price);
    }
    assert_eq!(asks.pop_best(), Some(101));
    assert_eq!(asks.pop_best(), Some(103));
    assert_eq!(asks.pop_best(), Some(105));
    assert_eq!(asks.pop_best(), None);
}
