use std::error::Error;

use kata_errors_09::{User, UserError, load_user};

#[test]
fn loads_a_user() {
    let user = load_user("ada, 36").unwrap();
    assert_eq!(
        user,
        User {
            name: "ada".into(),
            age: 36
        }
    );
}

#[test]
fn says_which_field_is_missing() {
    let err = load_user("ada").unwrap_err();
    assert!(matches!(err, UserError::MissingField("age")));
    assert_eq!(err.to_string(), "missing field: age");
}

#[test]
fn keeps_the_parse_error_as_source() {
    let err = load_user("ada, old").unwrap_err();
    assert!(matches!(err, UserError::BadAge { .. }));
    assert_eq!(err.to_string(), "bad age \"old\"");
    assert!(err.source().is_some());
}
