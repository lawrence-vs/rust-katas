use kata_money_42::{Money, MoneyError, fill_cost, total};

#[test]
fn money_round_trips_its_cents() {
    assert_eq!(Money::from_cents(12_345).cents(), 12_345);
}

#[test]
fn ten_cents_plus_twenty_cents_is_thirty_cents() {
    let ten = Money::from_cents(10);
    let twenty = Money::from_cents(20);
    assert_eq!(total(&[ten, twenty]), Ok(Money::from_cents(30)));
}

#[test]
fn a_million_dimes_add_up_exactly() {
    let dimes = vec![Money::from_cents(10); 1_000_000];
    assert_eq!(total(&dimes), Ok(Money::from_cents(10_000_000)));
}

#[test]
fn fill_cost_is_price_times_quantity() {
    assert_eq!(
        fill_cost(Money::from_cents(1_999), 3),
        Ok(Money::from_cents(5_997))
    );
}

#[test]
fn overflow_is_an_error_not_a_wrap_or_a_panic() {
    let max = Money::from_cents(u64::MAX);
    assert_eq!(
        total(&[max, Money::from_cents(1)]),
        Err(MoneyError::Overflow)
    );
    assert_eq!(fill_cost(max, 2), Err(MoneyError::Overflow));
}
