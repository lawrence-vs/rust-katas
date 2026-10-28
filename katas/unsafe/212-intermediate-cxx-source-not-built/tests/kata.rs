use kata_unsafe_212::add;

#[test]
fn adds_in_cpp() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn handles_negatives() {
    assert_eq!(add(-7, 3), -4);
}
