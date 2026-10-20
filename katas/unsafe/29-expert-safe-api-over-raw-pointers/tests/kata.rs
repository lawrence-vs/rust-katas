use kata_unsafe_29::Buffer;

#[test]
fn safe_reads_and_writes() {
    let mut b = Buffer::new(4);
    assert!(b.set(1, 9));
    assert_eq!(b.get(1), Some(9));
    assert_eq!(b.get(0), Some(0));
}

#[test]
fn out_of_bounds_is_none_not_ub() {
    let mut b = Buffer::new(4);
    assert!(!b.set(4, 1));
    assert_eq!(b.get(4), None);
}
