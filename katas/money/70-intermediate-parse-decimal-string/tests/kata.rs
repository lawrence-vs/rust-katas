use kata_money_70::{ParseError, Price};

#[test]
fn zero_point_five_seven_is_exact() {
    assert_eq!(
        "0.57".parse::<Price>(),
        Ok(Price(5_700)),
        "0.57 is exactly 5700 ten-thousandths"
    );
}

#[test]
fn trims_and_pads_the_fraction() {
    let cases = [
        (" 12.3400 ", 123_400),
        ("12.34", 123_400),
        ("12", 120_000),
        ("0.0001", 1),
        ("007.5", 75_000),
        ("\t19.99\n", 199_900),
        ("0", 0),
    ];
    for (input, minor) in cases {
        assert_eq!(
            input.parse::<Price>(),
            Ok(Price(minor)),
            "{input:?} should be {minor} ten-thousandths"
        );
    }
}

#[test]
fn more_than_four_places_is_rejected() {
    for input in ["1.23456", "1.23450", "0.00001"] {
        assert_eq!(
            input.parse::<Price>(),
            Err(ParseError::TooManyDecimals),
            "{input:?} has a fifth decimal place"
        );
    }
}

#[test]
fn floats_signs_and_separators_are_not_prices() {
    let inputs = [
        "1e3", "NaN", "inf", "-1", "+1", "1.+5", "1 000", "1,5", "0x10", "1.e5",
    ];
    for input in inputs {
        assert_eq!(
            input.parse::<Price>(),
            Err(ParseError::InvalidDigit),
            "{input:?} is not a price"
        );
    }
}

#[test]
fn empty_is_its_own_error() {
    for input in ["", "   "] {
        assert_eq!(
            input.parse::<Price>(),
            Err(ParseError::Empty),
            "{input:?} is empty"
        );
    }
}

#[test]
fn a_dot_needs_digits_on_both_sides() {
    for input in [".", "12.", ".5", "1.2.3"] {
        assert_eq!(
            input.parse::<Price>(),
            Err(ParseError::Malformed),
            "{input:?} is malformed"
        );
    }
}

#[test]
fn the_largest_price_fits_and_one_more_overflows() {
    assert_eq!(
        "1844674407370955.1615".parse::<Price>(),
        Ok(Price(u64::MAX)),
        "the largest price fits"
    );
    for input in [
        "1844674407370955.1616",
        "18446744073709551615",
        "99999999999999999999",
    ] {
        assert_eq!(
            input.parse::<Price>(),
            Err(ParseError::Overflow),
            "{input:?} does not fit in a u64"
        );
    }
}
