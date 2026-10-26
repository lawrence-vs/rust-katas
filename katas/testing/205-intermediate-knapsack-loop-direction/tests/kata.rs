use kata_testing_205::{Item, max_value};

fn item(weight: usize, value: u64) -> Item {
    Item { weight, value }
}

#[test]
fn cases() {
    let cases = [
        ("no items", vec![], 10, 0),
        ("capacity zero", vec![item(1, 10)], 0, 0),
        ("item heavier than capacity", vec![item(5, 10)], 4, 0),
        ("one item counted once", vec![item(2, 3)], 4, 3),
        ("exact fit, two items", vec![item(5, 7), item(5, 8)], 10, 15),
        ("pair beats repeats", vec![item(2, 3), item(3, 5)], 6, 8),
        (
            "classic",
            vec![item(5, 10), item(4, 40), item(6, 30), item(3, 50)],
            10,
            90,
        ),
    ];
    for (name, items, capacity, want) in &cases {
        assert_eq!(max_value(items, *capacity), *want, "case: {name}");
    }
}
