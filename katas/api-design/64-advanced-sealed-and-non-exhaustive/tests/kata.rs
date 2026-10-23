use kata_api_design_64::{Instrument, Perp, RejectReason, Spot, round_price};

fn message(r: RejectReason) -> &'static str {
    match r {
        RejectReason::InsufficientFunds => "top up",
        RejectReason::PriceOutOfBand => "move your price",
        RejectReason::MarketClosed => "try later",
        _ => "rejected",
    }
}

#[test]
fn downstream_match_keeps_a_fallback_arm() {
    assert_eq!(message(RejectReason::InsufficientFunds), "top up");
    assert_eq!(message(RejectReason::MarketClosed), "try later");
}

#[test]
fn only_market_closed_is_retryable() {
    assert!(RejectReason::MarketClosed.retryable());
    assert!(!RejectReason::InsufficientFunds.retryable());
    assert!(!RejectReason::PriceOutOfBand.retryable());
}

#[test]
fn a_sealed_trait_is_still_a_bound() {
    fn describe<I: Instrument>(i: &I) -> String {
        format!("{} tick {}", i.symbol(), i.tick_size())
    }
    assert_eq!(describe(&Spot::new("BTC", "USD")), "BTC-USD tick 1");
    assert_eq!(round_price(&Perp::new("BTC"), 1234), 1230);
}

#[test]
fn a_sealed_trait_is_still_a_trait_object() {
    let all: Vec<Box<dyn Instrument>> = vec![
        Box::new(Spot::new("ETH", "USD")),
        Box::new(Perp::new("ETH")),
    ];
    let symbols: Vec<String> = all.iter().map(|i| i.symbol()).collect();
    assert_eq!(symbols, ["ETH-USD", "ETH-PERP"]);
    assert_eq!(round_price(all[1].as_ref(), 99), 95);
}
