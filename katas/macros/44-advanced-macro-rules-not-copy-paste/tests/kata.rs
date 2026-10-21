use kata_macros_44::{Amount, Fee, Notional, Price, Qty, Sats, label, total};

// No `use` in here: the macro has to find `Amount` by itself.
mod outside {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Rebate(u64);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Margin(u64);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Collateral(u64);

    kata_macros_44::impl_amount!(Rebate => "rebate cents");

    kata_macros_44::impl_amount! {
        Margin => "margin cents",
        Collateral => "collateral cents",
    }
}

use outside::{Collateral, Margin, Rebate};

#[test]
fn every_unit_labels_itself() {
    assert_eq!(label(Price::new(101)), "101 ticks");
    assert_eq!(label(Qty::new(3)), "3 lots");
    assert_eq!(label(Notional::new(2500)), "2500 cents");
    assert_eq!(label(Fee::new(25)), "25 bps");
    assert_eq!(label(Sats::new(1000)), "1000 sats");
}

#[test]
fn every_unit_adds() {
    assert_eq!(
        Price::new(2).checked_add(Price::new(3)),
        Some(Price::new(5))
    );
    assert_eq!(Fee::new(2).checked_add(Fee::new(3)), Some(Fee::new(5)));
    assert_eq!(Sats::new(2).checked_add(Sats::new(3)), Some(Sats::new(5)));
}

#[test]
fn every_unit_reports_overflow() {
    assert_eq!(Price::new(u64::MAX).checked_add(Price::new(1)), None);
    assert_eq!(Qty::new(u64::MAX).checked_add(Qty::new(1)), None);
    assert_eq!(Notional::new(u64::MAX).checked_add(Notional::new(1)), None);
    assert_eq!(Fee::new(u64::MAX).checked_add(Fee::new(1)), None);
    assert_eq!(Sats::new(u64::MAX).checked_add(Sats::new(1)), None);
}

#[test]
fn total_sums_or_reports_overflow() {
    assert_eq!(
        total(&[Qty::new(1), Qty::new(2), Qty::new(4)]),
        Some(Qty::new(7))
    );
    assert_eq!(total::<Price>(&[]), Some(Price::new(0)));
    assert_eq!(total(&[Sats::new(u64::MAX), Sats::new(1)]), None);
}

#[test]
fn another_crate_can_use_the_macro() {
    assert_eq!(label(Rebate::new(7)), "7 rebate cents");
    assert_eq!(label(Margin::new(8)), "8 margin cents");
    assert_eq!(label(Collateral::new(9)), "9 collateral cents");
    assert_eq!(Margin::new(u64::MAX).checked_add(Margin::new(1)), None);
    assert_eq!(
        total(&[Collateral::new(1), Collateral::new(2)]),
        Some(Collateral::new(3))
    );
}

#[test]
fn one_impl_not_five() {
    let src = include_str!("../src/lib.rs");
    assert_eq!(
        src.matches("Amount for").count(),
        1,
        "write the impl once, inside the macro, and invoke the macro for the five types"
    );
}
