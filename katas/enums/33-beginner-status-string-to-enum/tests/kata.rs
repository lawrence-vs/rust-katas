use kata_enums_33::{OrderStatus, parse_status};

#[test]
fn parses_the_wire_names() {
    assert_eq!(parse_status("new"), Some(OrderStatus::New));
    assert_eq!(
        parse_status("partially_filled"),
        Some(OrderStatus::PartiallyFilled)
    );
    assert_eq!(parse_status("filled"), Some(OrderStatus::Filled));
    assert_eq!(parse_status("cancelled"), Some(OrderStatus::Cancelled));
    assert_eq!(parse_status("rejected"), Some(OrderStatus::Rejected));
}

#[test]
fn unknown_or_misspelt_is_none() {
    assert_eq!(parse_status("partially-filled"), None);
    assert_eq!(parse_status("Filled"), None);
    assert_eq!(parse_status("rejectd"), None);
    assert_eq!(parse_status(""), None);
}

#[test]
fn only_new_and_partially_filled_are_open() {
    assert!(OrderStatus::New.is_open());
    assert!(OrderStatus::PartiallyFilled.is_open());
    assert!(!OrderStatus::Filled.is_open());
    assert!(!OrderStatus::Cancelled.is_open());
    assert!(!OrderStatus::Rejected.is_open());
}

#[test]
fn partially_filled_and_filled_have_fills() {
    assert!(OrderStatus::PartiallyFilled.has_fills());
    assert!(OrderStatus::Filled.has_fills());
    assert!(!OrderStatus::New.has_fills());
    assert!(!OrderStatus::Cancelled.has_fills());
    assert!(!OrderStatus::Rejected.has_fills());
}
