use kata_iterators_35::{Fill, Side, bought_qty};

fn buy(qty: u64) -> Fill {
    Fill {
        side: Side::Buy,
        qty,
    }
}

fn sell(qty: u64) -> Fill {
    Fill {
        side: Side::Sell,
        qty,
    }
}

#[test]
fn no_fills_bought_nothing() {
    assert_eq!(bought_qty(&[]), 0);
}

#[test]
fn sums_only_the_buys() {
    assert_eq!(bought_qty(&[buy(5), sell(3), buy(2)]), 7);
}

#[test]
fn first_and_last_fills_count() {
    assert_eq!(bought_qty(&[buy(1), sell(10), sell(10), buy(100)]), 101);
}

#[test]
fn only_sells_bought_nothing() {
    assert_eq!(bought_qty(&[sell(4), sell(6)]), 0);
}
