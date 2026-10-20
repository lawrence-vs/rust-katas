use std::collections::HashMap;

use kata_api_design_10::{AccountId, Cents, transfer};

#[test]
fn moves_money_between_typed_accounts() {
    let alice = AccountId(1);
    let bob = AccountId(2);
    let mut balances = HashMap::from([(alice, Cents(1_000)), (bob, Cents(0))]);
    transfer(&mut balances, alice, bob, Cents(300));
    assert_eq!(balances[&alice], Cents(700));
    assert_eq!(balances[&bob], Cents(300));
}

#[test]
fn newtypes_cost_nothing() {
    assert_eq!(size_of::<AccountId>(), size_of::<u64>());
    assert_eq!(size_of::<Cents>(), size_of::<u64>());
}
