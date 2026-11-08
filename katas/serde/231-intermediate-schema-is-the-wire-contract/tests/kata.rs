use kata_serde_231::Order;

#[test]
fn wire_key_is_volume() {
    let json = serde_json::to_value(Order {
        symbol: "BTC".into(),
        volume: 2,
    })
    .unwrap();
    assert!(json.get("volume").is_some(), "wire key changed");
}

#[test]
fn schema_property_is_volume() {
    let schema = serde_json::to_value(schemars::schema_for!(Order)).unwrap();
    assert!(
        schema["properties"].get("volume").is_some(),
        "schema property missing"
    );
}
