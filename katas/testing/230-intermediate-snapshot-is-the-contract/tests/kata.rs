#[test]
fn receipt_matches_the_contract() {
    insta::assert_snapshot!(kata_testing_230::receipt("BTC", 2, 1250));
}
