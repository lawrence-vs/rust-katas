use kata_state_machines_194::{Account, AccountError, Fill, Kind, OrderId, Status};

/// Money never appears or disappears: every cent is available, held or spent.
fn assert_conserved(acct: &Account, deposit: u64) {
    let total = acct
        .available()
        .checked_add(acct.reserved())
        .and_then(|t| t.checked_add(acct.spent()));
    assert_eq!(
        total,
        Some(deposit),
        "available + reserved + spent must stay {deposit}"
    );
}

#[test]
fn placing_a_limit_buy_holds_price_times_quantity() {
    let mut acct = Account::new(10_000);
    let a = acct
        .place(Kind::Limit { price: 250, qty: 8 })
        .expect("enough money");
    assert_eq!(acct.status(a), Some(Status::Open));
    assert_eq!(acct.held(a), Some(2_000));
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (8_000, 2_000, 0)
    );
    assert_conserved(&acct, 10_000);
}

#[test]
fn each_fill_moves_only_the_filled_part_from_held_to_spent() {
    let mut acct = Account::new(10_000);
    let a = acct
        .place(Kind::Limit {
            price: 100,
            qty: 10,
        })
        .expect("enough money");
    let f = acct.fill(a, 3, 100).expect("3 of 10 at the limit");
    assert_eq!(
        f,
        Fill {
            qty: 3,
            cost: 300,
            status: Status::PartiallyFilled
        }
    );
    assert_eq!(
        acct.held(a),
        Some(700),
        "after 3 of 10 at 100, the order holds 700"
    );
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (9_000, 700, 300)
    );
    assert_conserved(&acct, 10_000);

    let f = acct.fill(a, 7, 100).expect("the last 7");
    assert_eq!(
        f,
        Fill {
            qty: 7,
            cost: 700,
            status: Status::Filled
        }
    );
    assert_eq!(acct.held(a), Some(0));
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (9_000, 0, 1_000)
    );
    assert_conserved(&acct, 10_000);
}

#[test]
fn cancel_after_a_partial_fill_releases_only_the_unfilled_part() {
    let mut acct = Account::new(10_000);
    let a = acct
        .place(Kind::Limit {
            price: 100,
            qty: 10,
        })
        .expect("enough money");
    let b = acct
        .place(Kind::Limit { price: 50, qty: 20 })
        .expect("enough money");
    acct.fill(a, 4, 100).expect("4 of 10 at the limit");
    assert_conserved(&acct, 10_000);

    let released = acct
        .cancel(a)
        .expect("a partly filled order can be cancelled");
    assert_eq!(
        released, 600,
        "cancel released {released}, want the unfilled 600"
    );
    assert_eq!(acct.status(a), Some(Status::Cancelled));
    assert_eq!(acct.held(a), Some(0));
    assert_eq!(acct.held(b), Some(1_000), "b's hold is not a's to release");
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (8_600, 1_000, 400)
    );
    assert_conserved(&acct, 10_000);
}

#[test]
fn cancel_of_an_untouched_order_releases_all_of_it() {
    let mut acct = Account::new(5_000);
    let a = acct
        .place(Kind::Limit {
            price: 100,
            qty: 10,
        })
        .expect("enough money");
    assert_eq!(acct.cancel(a), Ok(1_000));
    assert_eq!(acct.status(a), Some(Status::Cancelled));
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (5_000, 0, 0)
    );
    assert_conserved(&acct, 5_000);
}

#[test]
fn a_finished_order_cannot_be_cancelled_or_filled() {
    let mut acct = Account::new(5_000);
    let a = acct
        .place(Kind::Limit { price: 100, qty: 2 })
        .expect("enough money");
    assert_eq!(acct.cancel(a), Ok(200));
    assert_eq!(
        acct.cancel(a),
        Err(AccountError::NotActive {
            id: a,
            status: Status::Cancelled
        })
    );
    assert_eq!(
        acct.fill(a, 1, 100),
        Err(AccountError::NotActive {
            id: a,
            status: Status::Cancelled
        })
    );
    assert_eq!(acct.available(), 5_000, "a second cancel releases nothing");

    let b = acct
        .place(Kind::Limit { price: 100, qty: 2 })
        .expect("enough money");
    assert_eq!(
        acct.fill(b, 2, 100),
        Ok(Fill {
            qty: 2,
            cost: 200,
            status: Status::Filled
        })
    );
    assert_eq!(
        acct.cancel(b),
        Err(AccountError::NotActive {
            id: b,
            status: Status::Filled
        })
    );
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (4_800, 0, 200)
    );
    assert_conserved(&acct, 5_000);
}

#[test]
fn a_market_buy_stops_when_its_budget_is_used_and_releases_the_rest() {
    let mut acct = Account::new(10_000);
    let m = acct
        .place(Kind::Market { budget: 1_000 })
        .expect("enough money");
    assert_eq!((acct.available(), acct.reserved()), (9_000, 1_000));

    let f = acct.fill(m, 2, 300).expect("600 of the 1_000 budget");
    assert_eq!(
        f,
        Fill {
            qty: 2,
            cost: 600,
            status: Status::PartiallyFilled
        }
    );
    assert_eq!(acct.held(m), Some(400));
    assert_conserved(&acct, 10_000);

    let f = acct.fill(m, 5, 150).expect("the budget pays for 2 of 5");
    assert_eq!(
        f,
        Fill {
            qty: 2,
            cost: 300,
            status: Status::Filled
        }
    );
    assert_eq!(acct.held(m), Some(0));
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (9_100, 0, 900)
    );
    assert_conserved(&acct, 10_000);

    assert_eq!(
        acct.fill(m, 1, 10),
        Err(AccountError::NotActive {
            id: m,
            status: Status::Filled
        })
    );
}

#[test]
fn a_refused_call_changes_nothing() {
    let mut acct = Account::new(1_000);
    assert_eq!(
        acct.place(Kind::Limit {
            price: 100,
            qty: 11
        }),
        Err(AccountError::Insufficient {
            needed: 1_100,
            available: 1_000
        })
    );
    assert_eq!(
        acct.place(Kind::Limit {
            price: u64::MAX,
            qty: 2
        }),
        Err(AccountError::Overflow)
    );
    assert_eq!(
        acct.place(Kind::Limit { price: 100, qty: 0 }),
        Err(AccountError::ZeroQuantity)
    );
    assert_eq!(
        acct.place(Kind::Limit { price: 0, qty: 1 }),
        Err(AccountError::ZeroPrice)
    );
    assert_eq!(
        acct.place(Kind::Market { budget: 0 }),
        Err(AccountError::ZeroQuantity)
    );

    let a = acct
        .place(Kind::Limit { price: 100, qty: 5 })
        .expect("enough money");
    assert_eq!(
        acct.fill(a, 1, 99),
        Err(AccountError::WrongPrice {
            id: a,
            limit: 100,
            got: 99
        })
    );
    assert_eq!(
        acct.fill(a, 6, 100),
        Err(AccountError::OverFill {
            id: a,
            remaining: 5,
            got: 6
        })
    );
    assert_eq!(acct.fill(a, 0, 100), Err(AccountError::ZeroQuantity));
    assert_eq!(acct.fill(a, 1, 0), Err(AccountError::ZeroPrice));
    assert_eq!(
        acct.fill(OrderId(99), 1, 100),
        Err(AccountError::UnknownOrder(OrderId(99)))
    );
    assert_eq!(
        acct.cancel(OrderId(99)),
        Err(AccountError::UnknownOrder(OrderId(99)))
    );

    assert_eq!(acct.status(a), Some(Status::Open));
    assert_eq!(acct.held(a), Some(500));
    assert_eq!(
        (acct.available(), acct.reserved(), acct.spent()),
        (500, 500, 0)
    );
    assert_conserved(&acct, 1_000);
}
