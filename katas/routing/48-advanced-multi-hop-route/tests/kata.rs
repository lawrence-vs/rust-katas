use kata_routing_48::{Asset, Atoms, Fill, Hop, Route, RouteError, best_route};

fn market(from: Asset, to: Asset, rate: u64, fee_bps: u32, min_in: u64) -> Hop {
    Hop {
        from,
        to,
        rate,
        fee_bps,
        min_in: Atoms(min_in),
    }
}

fn std_markets(direct_fee: u32) -> Vec<Hop> {
    vec![
        market(Asset::Btc, Asset::Zar, 120_000_000, direct_fee, 0),
        market(Asset::Btc, Asset::Usdt, 6_500_000, 20, 0),
        market(Asset::Usdt, Asset::Zar, 1_850_000_000, 20, 0),
    ]
}

#[test]
fn fees_decide_the_route() {
    let r = best_route(&std_markets(10), Asset::Btc, Asset::Zar, Atoms(100_000_000));
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Direct,
            out: Atoms(119_880_000)
        }),
        "fees decide the route"
    );
}

#[test]
fn each_hop_takes_its_fee() {
    let r = best_route(
        &std_markets(100),
        Asset::Btc,
        Asset::Zar,
        Atoms(100_000_000),
    );
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Via(Asset::Usdt),
            out: Atoms(119_769_481)
        }),
        "each hop takes its fee"
    );
}

#[test]
fn each_hop_rounds_down_to_whole_atoms() {
    let markets = vec![
        market(Asset::Btc, Asset::Usdt, 6_500_000, 20, 0),
        market(Asset::Usdt, Asset::Zar, 1_850_000_000, 20, 0),
    ];
    let r = best_route(&markets, Asset::Btc, Asset::Zar, Atoms(1_001));
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Via(Asset::Usdt),
            out: Atoms(1_181)
        }),
        "each hop rounds down to whole atoms"
    );
}

#[test]
fn the_fee_is_rounded_up_on_the_whole_atom_fill() {
    let markets = vec![market(Asset::Btc, Asset::Usdt, 1_000_000, 500, 0)];
    let r = best_route(&markets, Asset::Btc, Asset::Usdt, Atoms(1_099));
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Direct,
            out: Atoms(9)
        }),
        "the fee rounds up on the whole-atom fill"
    );
}

#[test]
fn a_hop_below_its_minimum_is_skipped() {
    let mut markets = std_markets(100);
    markets[2].min_in = Atoms(6_490_000);
    let r = best_route(&markets, Asset::Btc, Asset::Zar, Atoms(100_000_000));
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Direct,
            out: Atoms(118_800_000)
        }),
        "a hop below its minimum can't be used"
    );
}

#[test]
fn too_small_for_every_route() {
    let mut markets = std_markets(100);
    markets[0].min_in = Atoms(10_000);
    markets[1].min_in = Atoms(10_000);
    let r = best_route(&markets, Asset::Btc, Asset::Zar, Atoms(1_000));
    assert_eq!(
        r,
        Err(RouteError::BelowMinimum),
        "too small for every route"
    );
}

#[test]
fn no_market_means_no_route() {
    let r = best_route(&std_markets(10), Asset::Zar, Asset::Btc, Atoms(1_000));
    assert_eq!(r, Err(RouteError::NoRoute), "no market, no route");
}

#[test]
fn a_tie_goes_to_the_direct_route() {
    let markets = vec![
        market(Asset::Btc, Asset::Zar, 120_000_000, 0, 0),
        market(Asset::Btc, Asset::Usdt, 5_000_000, 0, 0),
        market(Asset::Usdt, Asset::Zar, 2_400_000_000, 0, 0),
    ];
    let r = best_route(&markets, Asset::Btc, Asset::Zar, Atoms(100_000_000));
    assert_eq!(
        r,
        Ok(Fill {
            route: Route::Direct,
            out: Atoms(120_000_000)
        }),
        "a tie goes to the direct route"
    );
}

#[test]
fn overflow_is_an_error() {
    let markets = vec![market(Asset::Btc, Asset::Usdt, u64::MAX, 0, 0)];
    let r = best_route(&markets, Asset::Btc, Asset::Usdt, Atoms(u64::MAX));
    assert_eq!(
        r,
        Err(RouteError::Overflow),
        "overflow is an error, not u64::MAX"
    );
}

#[test]
fn a_fee_over_100_percent_is_rejected() {
    let markets = vec![market(Asset::Btc, Asset::Usdt, 6_500_000, 10_001, 0)];
    let r = best_route(&markets, Asset::Btc, Asset::Usdt, Atoms(100_000_000));
    assert_eq!(
        r,
        Err(RouteError::InvalidFee),
        "a fee over 10000 bps is invalid"
    );
}
