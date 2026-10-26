use kata_collections_194::{PriceLevel, distinct_levels};

#[test]
fn counts_distinct_levels() {
    let levels = vec![
        PriceLevel {
            price: 101.5,
            qty: 2,
        },
        PriceLevel {
            price: 101.5,
            qty: 2,
        },
        PriceLevel {
            price: 102.0,
            qty: 1,
        },
    ];
    assert_eq!(distinct_levels(levels), 2);
}
