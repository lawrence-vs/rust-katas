use kata_drop_39::{Gateway, SubmitError};
use std::panic::{self, AssertUnwindSafe};

#[test]
fn good_orders_go_through() {
    let gw = Gateway::new(1);
    assert_eq!(gw.submit(" 25 "), Ok(25));
    assert_eq!(gw.submit("10"), Ok(10));
    assert_eq!(gw.in_flight(), 0);
}

#[test]
fn a_bad_quantity_frees_its_slot() {
    let gw = Gateway::new(1);
    assert!(matches!(gw.submit("ten"), Err(SubmitError::BadQuantity(_))));
    assert_eq!(gw.in_flight(), 0, "the `?` path kept the slot");
    assert_eq!(gw.submit("10"), Ok(10));
}

#[test]
fn a_zero_quantity_frees_its_slot() {
    let gw = Gateway::new(1);
    assert_eq!(gw.submit("0"), Err(SubmitError::ZeroQuantity));
    assert_eq!(gw.in_flight(), 0, "the early return kept the slot");
    assert_eq!(gw.submit("10"), Ok(10));
}

#[test]
fn a_held_slot_counts_until_dropped() {
    let gw = Gateway::new(2);
    let first = gw.reserve().unwrap();
    let _second = gw.reserve().unwrap();
    assert_eq!(gw.submit("10"), Err(SubmitError::Busy));
    drop(first);
    assert_eq!(gw.in_flight(), 1, "dropping a slot gives it back");
    assert_eq!(gw.submit("10"), Ok(10));
}

#[test]
fn a_panic_while_holding_a_slot_frees_it() {
    let gw = Gateway::new(1);
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let _slot = gw.reserve().unwrap();
        panic!("venue adapter bug");
    }));
    assert!(result.is_err());
    assert_eq!(gw.in_flight(), 0, "unwinding drops the slot");
}
