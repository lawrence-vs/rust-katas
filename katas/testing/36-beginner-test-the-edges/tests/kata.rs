use kata_testing_36::mean_price;

#[test]
fn edges() {
    let cases: &[(&str, &[u64], u64)] = &[
        ("one price", &[100], 100),
        ("two prices", &[100, 200], 150),
        ("rounds down", &[1, 2], 1),
        ("all zero", &[0, 0, 0], 0),
        (
            "total overflows u64",
            &[u64::MAX, u64::MAX - 2],
            u64::MAX - 1,
        ),
        ("many max prices", &[u64::MAX; 4], u64::MAX),
    ];
    for (name, prices, want) in cases {
        assert_eq!(mean_price(prices), *want, "case: {name}");
    }
}

#[test]
#[should_panic(expected = "no prices")]
fn empty_input_panics_with_a_clear_message() {
    mean_price(&[]);
}
