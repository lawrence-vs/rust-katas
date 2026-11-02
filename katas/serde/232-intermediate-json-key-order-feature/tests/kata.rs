#[test]
fn echo_keeps_key_order() {
    let s = r#"{"b":1,"a":2}"#;
    assert_eq!(kata_serde_232::echo(s).unwrap(), s, "key order changed");
}

#[test]
fn echo_single_key() {
    let s = r#"{"a":1}"#;
    assert_eq!(kata_serde_232::echo(s).unwrap(), s);
}
