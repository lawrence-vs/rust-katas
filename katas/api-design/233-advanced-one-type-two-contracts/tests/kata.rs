use kata_api_design_233::{Order, openapi};

#[test]
fn openapi_declares_the_rust_fields() {
    let doc = openapi();
    let props = doc["components"]["schemas"]["Order"]["properties"]
        .as_object()
        .expect("Order schema has properties");
    assert!(
        props.contains_key("volume"),
        "openapi and rust disagree on field names"
    );
}

#[test]
fn proto_matches_the_rust_type() {
    let proto = include_str!("../contract/orders.proto");
    let mut in_order = false;
    let mut proto_fields: Vec<String> = Vec::new();
    for line in proto.lines().map(str::trim) {
        if line.starts_with("message Order") {
            in_order = true;
        } else if line.starts_with('}') {
            in_order = false;
        } else if in_order && let Some((decl, _)) = line.split_once('=') {
            proto_fields.extend(decl.split_whitespace().last().map(str::to_string));
        }
    }
    proto_fields.sort();
    let order = Order {
        symbol: "BTC".into(),
        volume: 2,
    };
    let mut rust_keys: Vec<String> = serde_json::to_value(order)
        .expect("order serializes")
        .as_object()
        .expect("order is an object")
        .keys()
        .cloned()
        .collect();
    rust_keys.sort();
    assert_eq!(
        proto_fields, rust_keys,
        "proto and rust disagree on field names"
    );
}

#[test]
fn openapi_is_3_1() {
    assert_eq!(openapi()["openapi"], "3.1.0");
}
