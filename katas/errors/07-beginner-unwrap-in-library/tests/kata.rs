use kata_errors_07::parse_port;

#[test]
fn good_port() {
    assert_eq!(parse_port(" 8080 "), Ok(8080));
}

#[test]
fn bad_input_is_an_error_not_a_panic() {
    assert!(parse_port("http").is_err());
    assert!(parse_port("70000").is_err());
}
