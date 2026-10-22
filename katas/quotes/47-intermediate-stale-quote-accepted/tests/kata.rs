use kata_quotes_47::{AcceptError, Desk, Millis, Price, Quote};

const DESK: Desk = Desk {
    ttl_ms: 5_000,
    max_drift_bps: 10,
};
const MID: Price = Price(6_500_000); // $65,000.00
const ASK: Price = Price(6_532_500);
const ISSUED: Millis = Millis(1_000_000);

fn quote() -> Quote {
    DESK.issue(ASK, MID, ISSUED)
}

#[test]
fn a_fresh_quote_fills_at_its_price() {
    assert_eq!(
        DESK.accept(&quote(), MID, Millis(1_004_999)),
        Ok(ASK),
        "a fresh quote fills at its price"
    );
}

#[test]
fn a_quote_accepted_after_its_deadline_is_rejected() {
    assert_eq!(
        DESK.accept(&quote(), MID, Millis(1_005_000)),
        Err(AcceptError::Expired),
        "a quote accepted after its deadline is rejected"
    );
    assert_eq!(
        DESK.accept(&quote(), MID, Millis(1_060_000)),
        Err(AcceptError::Expired),
        "a quote accepted after its deadline is rejected"
    );
}

#[test]
fn a_quote_is_rejected_when_the_market_moved_past_tolerance() {
    // 10 bps of 6_500_000 cents is 6_500 cents.
    assert_eq!(
        DESK.accept(&quote(), Price(6_506_501), Millis(1_001_000)),
        Err(AcceptError::PriceMoved),
        "a quote is rejected when the market moved past tolerance"
    );
    assert_eq!(
        DESK.accept(&quote(), Price(6_493_499), Millis(1_001_000)),
        Err(AcceptError::PriceMoved),
        "a quote is rejected when the market moved past tolerance"
    );
}

#[test]
fn a_move_within_tolerance_still_fills() {
    assert_eq!(
        DESK.accept(&quote(), Price(6_506_500), Millis(1_001_000)),
        Ok(ASK),
        "a move inside tolerance still fills"
    );
    assert_eq!(
        DESK.accept(&quote(), Price(6_493_500), Millis(1_001_000)),
        Ok(ASK),
        "a move inside tolerance still fills"
    );
}

#[test]
fn expiry_is_checked_before_drift() {
    assert_eq!(
        DESK.accept(&quote(), Price(7_000_000), Millis(1_005_000)),
        Err(AcceptError::Expired),
        "an expired quote is Expired, whatever the market did"
    );
}

#[test]
fn a_deadline_past_u64_max_does_not_wrap_into_the_past() {
    let desk = Desk {
        ttl_ms: u64::MAX,
        max_drift_bps: 10,
    };
    let q = desk.issue(ASK, MID, Millis(1));
    assert_eq!(
        desk.accept(&q, MID, Millis(u64::MAX)),
        Ok(ASK),
        "a deadline past u64::MAX does not wrap into the past"
    );
}

#[test]
fn a_huge_price_does_not_overflow_the_drift_check() {
    let q = DESK.issue(Price(u64::MAX), Price(u64::MAX), ISSUED);
    assert_eq!(
        DESK.accept(&q, Price(u64::MAX), Millis(1_001_000)),
        Ok(Price(u64::MAX)),
        "a huge price does not overflow the drift check"
    );
    assert_eq!(
        DESK.accept(&q, Price(0), Millis(1_001_000)),
        Err(AcceptError::PriceMoved),
        "a huge price does not overflow the drift check"
    );
}
