use kata_lifetimes_186::field_or;

#[test]
fn picks_field_or_fallback() {
    let row = String::from("id,Smith,42");
    let fallback = String::from("none");
    assert_eq!(field_or(&row, 1, &fallback), "Smith");
    assert_eq!(field_or(&row, 9, &fallback), "none");
}
