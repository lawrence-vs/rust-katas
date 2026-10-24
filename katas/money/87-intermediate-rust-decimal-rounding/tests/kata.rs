use kata_money_87::{MoneyError, fee, settle};
use rust_decimal::Decimal;

fn d(s: &str) -> Decimal {
    s.parse().expect("test literal")
}

#[test]
fn a_settlement_tie_rounds_to_even() {
    assert_eq!(
        settle("0.125", "1"),
        Ok(d("0.12")),
        "a 0.125 tie settles to the even cent, 0.12"
    );
}

#[test]
fn a_fee_tie_rounds_half_up() {
    assert_eq!(
        fee("100.5", 100),
        Ok(d("1.01")),
        "a 1.005 fee rounds half-up to 1.01"
    );
}

#[test]
fn settlement_rounds_half_to_even() {
    let table = [
        ("0.125", "1", "0.12"),
        ("2.5", "0.05", "0.12"),
        ("0.135", "1", "0.14"),
        ("0.375", "1", "0.38"),
        ("0.1251", "1", "0.13"),
        ("0.1249", "1", "0.12"),
        ("19.99", "3", "59.97"),
        ("0.1", "3", "0.30"),
    ];
    for (price, qty, want) in table {
        assert_eq!(
            settle(price, qty),
            Ok(d(want)),
            "{price} x {qty} settles to {want}"
        );
    }
}

#[test]
fn fees_round_half_up() {
    let table = [
        ("100.5", 100, "1.01"),
        ("0.5", 100, "0.01"),
        ("250", 25, "0.63"),
        ("100.3", 100, "1.00"),
        ("100.7", 100, "1.01"),
        ("1000000", 30, "3000"),
    ];
    for (notional, bps, want) in table {
        assert_eq!(
            fee(notional, bps),
            Ok(d(want)),
            "{bps} bps of {notional} is {want}"
        );
    }
}

#[test]
fn only_plain_decimals_parse() {
    for s in ["", "abc", "NaN", "inf", "1,5"] {
        assert_eq!(fee(s, 10), Err(MoneyError::Parse), "{s:?} is not a decimal");
        assert_eq!(
            settle(s, "1"),
            Err(MoneyError::Parse),
            "{s:?} is not a decimal"
        );
        assert_eq!(
            settle("1", s),
            Err(MoneyError::Parse),
            "{s:?} is not a decimal"
        );
    }
}

#[test]
fn overflow_is_an_error() {
    let max = "79228162514264337593543950335";
    assert_eq!(
        settle(max, "2"),
        Err(MoneyError::Overflow),
        "overflow is an error, not a clamped number"
    );
    assert_eq!(
        fee(max, 10_000),
        Err(MoneyError::Overflow),
        "overflow is an error, not a clamped number"
    );
}
