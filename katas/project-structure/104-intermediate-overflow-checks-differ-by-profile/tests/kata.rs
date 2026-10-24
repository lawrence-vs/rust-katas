// Run both: `cargo test` and `cargo test --release`. Both must pass.
use kata_project_structure_104::{AmountError, add_amounts, total};

#[test]
fn small_amounts_add_up() {
    assert_eq!(add_amounts(100, 50), Ok(150));
}

#[test]
fn the_largest_sum_that_fits_is_ok() {
    assert_eq!(add_amounts(200, 55), Ok(255));
}

#[test]
fn zero_is_the_identity() {
    assert_eq!(add_amounts(0, 0), Ok(0));
    assert_eq!(total(&[]), Ok(0));
}

#[test]
fn total_of_fitting_amounts() {
    assert_eq!(total(&[100, 100, 55]), Ok(255));
}

#[test]
fn adding_past_u8_max_is_an_error() {
    assert_eq!(add_amounts(200, 100), Err(AmountError::Overflow));
}

#[test]
fn one_past_the_edge_is_an_error() {
    assert_eq!(add_amounts(255, 1), Err(AmountError::Overflow));
}

#[test]
fn total_stops_at_the_first_overflow() {
    assert_eq!(total(&[100, 100, 100]), Err(AmountError::Overflow));
}
