use kata_money_168::{Line, MoneyError, format_amount, line_total, parse_line, statement_total};
use rust_decimal::Decimal;

fn d(s: &str) -> Decimal {
    s.parse().expect("test literal")
}

fn line(price: &str, qty: &str) -> Line {
    Line {
        unit_price: d(price),
        qty: d(qty),
    }
}

#[test]
fn amounts_print_with_exactly_four_places() {
    let table = [
        ("1.5", "1.5000"),
        ("1.50", "1.5000"),
        ("100", "100.0000"),
        ("0", "0.0000"),
        ("0.00", "0.0000"),
        ("-2.5", "-2.5000"),
        ("1.2345", "1.2345"),
    ];
    for (input, want) in table {
        assert_eq!(format_amount(d(input)), want, "{input} prints as {want}");
    }
}

#[test]
fn extra_places_round_half_to_even() {
    let table = [
        ("0.12346", "0.1235"),
        ("0.12345", "0.1234"),
        ("0.12355", "0.1236"),
        ("-0.12355", "-0.1236"),
        ("2.00005", "2.0000"),
    ];
    for (input, want) in table {
        assert_eq!(
            format_amount(d(input)),
            want,
            "{input} rounds half-to-even to {want}"
        );
    }
}

#[test]
fn equal_amounts_print_the_same() {
    assert_eq!(d("1.5"), d("1.50"), "1.5 and 1.50 are the same value");
    assert_ne!(
        d("1.5").scale(),
        d("1.50").scale(),
        "but they carry different scales"
    );
    assert_eq!(
        format_amount(d("1.5")),
        format_amount(d("1.50")),
        "equal amounts print the same"
    );
}

#[test]
fn the_largest_amounts_print_padded() {
    assert_eq!(
        format_amount(Decimal::MAX),
        "79228162514264337593543950335.0000",
        "Decimal::MAX prints padded"
    );
    assert_eq!(
        format_amount(Decimal::MIN),
        "-79228162514264337593543950335.0000",
        "Decimal::MIN prints padded"
    );
    println!("{}", format_amount(Decimal::MAX));
}

#[test]
fn line_total_multiplies() {
    assert_eq!(line_total(&line("1.10", "3")), Ok(d("3.30")));
    assert_eq!(line_total(&line("0.25", "4")), Ok(d("1")));
    assert_eq!(
        line_total(&Line {
            unit_price: Decimal::MAX,
            qty: d("1")
        }),
        Ok(Decimal::MAX)
    );
}

#[test]
fn a_line_that_overflows_is_an_error() {
    assert_eq!(
        line_total(&Line {
            unit_price: Decimal::MAX,
            qty: d("2")
        }),
        Err(MoneyError::Overflow),
        "overflow is an error, not a panic"
    );
    assert_eq!(
        line_total(&Line {
            unit_price: Decimal::MIN,
            qty: d("2")
        }),
        Err(MoneyError::Overflow),
        "overflow is an error, not a panic"
    );
}

#[test]
fn statement_total_sums_lines() {
    assert_eq!(
        statement_total(&[]),
        Ok(Decimal::ZERO),
        "no lines total zero"
    );
    assert_eq!(
        statement_total(&[line("1.10", "3"), line("0.25", "4"), line("19.99", "1")]),
        Ok(d("24.29"))
    );
}

#[test]
fn a_statement_that_overflows_is_an_error() {
    let max = Line {
        unit_price: Decimal::MAX,
        qty: d("1"),
    };
    assert_eq!(
        statement_total(&[max]),
        Ok(Decimal::MAX),
        "the largest amount alone fits"
    );
    assert_eq!(
        statement_total(&[max, line("-1", "1")]),
        Ok(d("79228162514264337593543950334"))
    );
    assert_eq!(
        statement_total(&[max, line("1", "1")]),
        Err(MoneyError::Overflow),
        "overflow is an error, not a panic"
    );
    assert_eq!(
        statement_total(&[max, max]),
        Err(MoneyError::Overflow),
        "overflow is an error, not a panic"
    );
}

#[test]
fn amounts_arrive_as_strings() {
    assert_eq!(
        parse_line(r#"{"unit_price":"0.1","qty":"3"}"#),
        Ok(line("0.1", "3"))
    );
    assert_eq!(
        parse_line(r#"{"unit_price":"12345678901234567.89","qty":"1"}"#),
        Ok(line("12345678901234567.89", "1")),
        "a string keeps every digit"
    );
}

#[test]
fn a_json_number_amount_is_rejected() {
    let bad = [
        r#"{"unit_price":0.1,"qty":"3"}"#,
        r#"{"unit_price":12345678901234567.89,"qty":"1"}"#,
        r#"{"unit_price":"0.1","qty":3}"#,
        r#"{"unit_price":"abc","qty":"1"}"#,
        r#"{"unit_price":"0.1"}"#,
        r#"{"unit_price":"1","qty":"1","fee":"0"}"#,
    ];
    for json in bad {
        assert_eq!(
            parse_line(json),
            Err(MoneyError::Parse),
            "{json} is rejected"
        );
    }
}
