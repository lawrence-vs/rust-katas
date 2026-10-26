use kata_csv_187::split_fields;

#[test]
fn splits_plain_fields() {
    assert_eq!(split_fields("a,b,c"), ["a", "b", "c"]);
}

#[test]
fn keeps_quoted_comma() {
    assert_eq!(
        split_fields(r#"id,"Smith, J",42"#),
        ["id", "Smith, J", "42"]
    );
}
