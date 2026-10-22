use kata_serde_57::{Side, parse_order};

fn rejected(what: &str, json: &str) -> String {
    match parse_order(json) {
        Ok(order) => panic!("{what} was accepted: {order:?}"),
        Err(e) => e.to_string(),
    }
}

#[test]
fn a_valid_order_parses() {
    let order =
        parse_order(r#"{"client_order_id":"c-1","side":"buy","price":1999,"quantity":3}"#).unwrap();
    assert_eq!(order.price().cents(), 1999);
    assert_eq!(order.quantity().get(), 3);
    assert_eq!(order.side(), Side::Buy);
    assert!(!order.post_only());
    assert_eq!(order.client_order_id(), "c-1");
}

#[test]
fn post_only_is_kept() {
    let order = parse_order(
        r#"{"client_order_id":"c-1","side":"sell","price":1999,"quantity":3,"post_only":true}"#,
    )
    .unwrap();
    assert!(order.post_only());
}

#[test]
fn a_missing_quantity_is_an_error() {
    let msg = rejected(
        "a missing quantity",
        r#"{"client_order_id":"c-1","side":"buy","price":1999}"#,
    );
    assert!(msg.contains("missing field `quantity`"), "{msg}");
}

#[test]
fn a_negative_quantity_is_rejected_with_the_field_named() {
    let msg = rejected(
        "a negative quantity",
        r#"{"client_order_id":"c-1","side":"buy","price":1999,"quantity":-5}"#,
    );
    assert!(msg.contains("quantity must be positive"), "{msg}");
}

#[test]
fn a_zero_quantity_is_rejected() {
    let msg = rejected(
        "a zero quantity",
        r#"{"client_order_id":"c-1","side":"buy","price":1999,"quantity":0}"#,
    );
    assert!(msg.contains("quantity must be positive"), "{msg}");
}

#[test]
fn a_fractional_cent_is_rejected() {
    let msg = rejected(
        "a fractional cent",
        r#"{"client_order_id":"c-1","side":"buy","price":1999.5,"quantity":3}"#,
    );
    assert!(msg.contains("floating point"), "{msg}");
}

#[test]
fn a_negative_price_is_rejected_with_the_field_named() {
    let msg = rejected(
        "a negative price",
        r#"{"client_order_id":"c-1","side":"buy","price":-1,"quantity":3}"#,
    );
    assert!(msg.contains("price must be positive"), "{msg}");
}

#[test]
fn a_zero_price_is_rejected() {
    let msg = rejected(
        "a zero price",
        r#"{"client_order_id":"c-1","side":"buy","price":0,"quantity":3}"#,
    );
    assert!(msg.contains("price must be positive"), "{msg}");
}

#[test]
fn a_misspelt_field_is_rejected_not_ignored() {
    let msg = rejected(
        "a misspelt postOnly",
        r#"{"client_order_id":"c-1","side":"buy","price":1999,"quantity":3,"postOnly":true}"#,
    );
    assert!(msg.contains("unknown field `postOnly`"), "{msg}");
}

#[test]
fn an_extra_field_is_rejected() {
    let msg = rejected(
        "an extra leverage field",
        r#"{"client_order_id":"c-1","side":"buy","price":1999,"quantity":3,"leverage":100}"#,
    );
    assert!(msg.contains("unknown field `leverage`"), "{msg}");
}
