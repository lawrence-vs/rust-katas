use kata_serde_206::Fill;

#[test]
fn accepts_known_fields() {
    let fill: Fill = serde_json::from_str(r#"{"id":1,"qty":2}"#).unwrap();
    assert_eq!(fill, Fill { id: 1, qty: 2 });
}

#[test]
fn rejects_missing_qty() {
    assert!(serde_json::from_str::<Fill>(r#"{"id":1}"#).is_err());
}
