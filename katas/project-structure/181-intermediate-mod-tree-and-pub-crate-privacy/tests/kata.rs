use kata_project_structure_181::cart::Cart;
use kata_project_structure_181::percent::{Percent, PercentError};

#[test]
fn new_accepts_zero_to_one_hundred() {
    assert_eq!(Percent::new(0).map(Percent::get), Ok(0));
    assert_eq!(Percent::new(20).map(Percent::get), Ok(20));
    assert_eq!(Percent::new(100).map(Percent::get), Ok(100));
}

#[test]
fn new_rejects_over_one_hundred() {
    assert_eq!(Percent::new(101), Err(PercentError::OverHundred(101)));
    assert_eq!(Percent::new(150), Err(PercentError::OverHundred(150)));
    assert_eq!(
        PercentError::OverHundred(150).to_string(),
        "150% is over 100%"
    );
}

#[test]
fn of_rounds_down() {
    let p = Percent::new(15).unwrap();
    assert_eq!(p.of(999), 149);
    assert_eq!(p.of(0), 0);
    assert_eq!(Percent::new(100).unwrap().of(u64::MAX), u64::MAX);
}

#[test]
fn cart_total_applies_the_discount() {
    let mut cart = Cart::new();
    cart.add(250, 4);
    cart.add(1_000, 1);
    assert_eq!(cart.total(), Some(2_000));
    cart.set_discount(Percent::new(25).unwrap());
    assert_eq!(cart.total(), Some(1_500));
}

#[test]
fn cart_total_is_none_on_overflow() {
    let mut big = Cart::new();
    big.add(u64::MAX, 2);
    assert_eq!(big.total(), None);

    let mut sum = Cart::new();
    sum.add(u64::MAX, 1);
    sum.add(1, 1);
    assert_eq!(sum.total(), None);
}

#[test]
fn an_empty_cart_costs_nothing() {
    assert_eq!(Cart::new().total(), Some(0));
}
