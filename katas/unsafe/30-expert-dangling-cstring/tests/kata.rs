use kata_unsafe_30::c_len;

#[test]
fn counts_bytes() {
    assert_eq!(c_len("hello"), Some(5));
}

#[test]
fn interior_nul_is_none() {
    assert_eq!(c_len("a\0b"), None);
}
