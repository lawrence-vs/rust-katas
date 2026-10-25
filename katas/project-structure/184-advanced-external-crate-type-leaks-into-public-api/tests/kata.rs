use kata_project_structure_184::{Price, PriceError, parse_price, total};
use std::any::type_name;
use std::error::Error;

const LEAK: &str = "the public API names another crate's type";

fn p(s: &str) -> Price {
    parse_price(s).expect("a valid price")
}

/// Lines a downstream crate can see: lines starting with `pub ` or `impl`, and every line of a `pub enum`.
fn public_lines() -> Vec<&'static str> {
    let mut lines = Vec::new();
    let mut in_pub_enum = false;
    for line in include_str!("../src/lib.rs").lines() {
        let t = line.trim_start();
        if t.starts_with("pub enum ") {
            in_pub_enum = true;
        }
        if in_pub_enum || t.starts_with("pub ") || t.starts_with("impl") {
            lines.push(line);
        }
        if in_pub_enum && line == "}" {
            in_pub_enum = false;
        }
    }
    lines
}

#[test]
fn a_price_parses_and_prints_as_written() {
    assert_eq!(p("1.50").to_string(), "1.50");
    assert_eq!(p("0").to_string(), "0");
}

#[test]
fn a_negative_price_is_an_error() {
    assert!(matches!(parse_price("-0.01"), Err(PriceError::Negative)));
    assert_eq!(
        parse_price("-0.01").unwrap_err().to_string(),
        "negative price"
    );
}

#[test]
fn a_total_adds_prices() {
    assert_eq!(
        total(&[p("1.50"), p("2.25")]).expect("fits").to_string(),
        "3.75"
    );
    assert_eq!(total(&[]).expect("empty").to_string(), "0");
}

#[test]
fn a_total_too_big_is_an_error() {
    assert!(matches!(
        total(&[p("79228162514264337593543950335"), p("1")]),
        Err(PriceError::Overflow)
    ));
}

#[test]
fn price_is_this_crates_own_type() {
    assert!(
        type_name::<Price>().starts_with("kata_project_structure_184::"),
        "{LEAK}: Price is {}",
        type_name::<Price>()
    );
}

#[test]
fn no_public_item_names_rust_decimal() {
    for line in public_lines() {
        assert!(
            !line.contains("rust_decimal") && !line.contains("Decimal"),
            "{LEAK}: {line}"
        );
    }
}

#[test]
fn bad_input_is_invalid_with_the_cause_as_source() {
    for input in ["abc", "", "1.2.3"] {
        let err = parse_price(input).unwrap_err();
        assert!(matches!(err, PriceError::Invalid { .. }), "{input:?}");
        assert_eq!(err.to_string(), format!("invalid price {input:?}"));
        let cause = err.source().map(ToString::to_string);
        assert!(
            cause.as_deref().is_some_and(|c| !c.is_empty()),
            "the parser's error is kept as source()"
        );
        assert_ne!(cause.as_deref(), Some(err.to_string().as_str()));
    }
}
