use kata_traits_63::*;

fn venues() -> Vec<Box<dyn Venue>> {
    vec![
        Box::new(FixedVenue::new(
            "north",
            &[("BTC-USD", 6_500_000), ("ETH-USD", 310_000)],
        )),
        Box::new(Halted::new("east")),
        Box::new(FixedVenue::new("south", &[("BTC-USD", 6_499_900)])),
    ]
}

#[test]
fn cheapest_picks_the_lowest_ask_across_venue_types() {
    let v = venues();
    assert_eq!(cheapest(&v, "BTC-USD"), Some(("south", 6_499_900)));
}

#[test]
fn a_pair_only_one_venue_lists() {
    let v = venues();
    assert_eq!(cheapest(&v, "ETH-USD"), Some(("north", 310_000)));
}

#[test]
fn no_venue_lists_the_pair() {
    let v = venues();
    assert_eq!(cheapest(&v, "SOL-USD"), None);
}

#[test]
fn a_tie_goes_to_the_first_venue() {
    let v: Vec<Box<dyn Venue>> = vec![
        Box::new(FixedVenue::new("north", &[("BTC-USD", 100)])),
        Box::new(FixedVenue::new("south", &[("BTC-USD", 100)])),
    ];
    assert_eq!(cheapest(&v, "BTC-USD"), Some(("north", 100)));
}

#[test]
fn best_asks_still_works_on_a_concrete_venue() {
    let north = FixedVenue::new("north", &[("BTC-USD", 6_500_000)]);
    assert_eq!(
        north.best_asks(["BTC-USD", "SOL-USD"]),
        vec![Some(6_500_000), None]
    );
    let wanted = [String::from("SOL-USD"), String::from("BTC-USD")];
    assert_eq!(
        north.best_asks(wanted.iter().map(String::as_str)),
        vec![None, Some(6_500_000)]
    );
}
