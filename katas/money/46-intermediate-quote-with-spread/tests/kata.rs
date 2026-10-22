use kata_money_46::{Cents, Price, QuoteError, Side, Size, quote};

#[test]
fn a_buy_rounds_up_in_the_houses_favour() {
    assert_eq!(
        quote(Price(6_500_001), Size(1_000), 0, Side::Buy),
        Ok(Cents(66)),
        "a buy rounds up, in the house's favour"
    );
}

#[test]
fn a_sell_rounds_down_in_the_houses_favour() {
    assert_eq!(
        quote(Price(6_500_001), Size(1_500), 0, Side::Sell),
        Ok(Cents(97)),
        "a sell rounds down, in the house's favour"
    );
}

#[test]
fn the_spread_is_added_on_a_buy_and_taken_off_a_sell() {
    assert_eq!(
        quote(Price(6_500_000), Size(100_000_000), 50, Side::Buy),
        Ok(Cents(6_532_500)),
        "the spread is applied off mid"
    );
    assert_eq!(
        quote(Price(6_500_000), Size(100_000_000), 50, Side::Sell),
        Ok(Cents(6_467_500)),
        "the spread is applied off mid"
    );
}

#[test]
fn a_large_order_does_not_overflow() {
    assert_eq!(
        quote(Price(6_500_000), Size(10_000_000_000), 50, Side::Buy),
        Ok(Cents(653_250_000)),
        "a 100 BTC quote fits in cents"
    );
}

#[test]
fn overflow_is_an_error_not_a_saturated_max() {
    assert_eq!(
        quote(Price(u64::MAX), Size(u64::MAX), 0, Side::Buy),
        Err(QuoteError::Overflow),
        "overflow is an error, not u64::MAX"
    );
    assert_eq!(
        quote(Price(u64::MAX), Size(200_000_000), 0, Side::Buy),
        Err(QuoteError::Overflow),
        "overflow is an error, not u64::MAX"
    );
}

#[test]
fn a_spread_over_100_percent_is_rejected() {
    assert_eq!(
        quote(Price(6_500_000), Size(100_000_000), 10_001, Side::Sell),
        Err(QuoteError::InvalidSpread),
        "a spread over 10000 bps is invalid"
    );
}
