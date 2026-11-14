use kata_unsafe_213::parse_qty;

fn call(s: &str) -> (i32, i64) {
    let mut out = 0i64;
    let rc = unsafe { parse_qty(s.as_ptr(), s.len(), &mut out) };
    (rc, out)
}

#[test]
fn parses_a_quantity() {
    assert_eq!(call("-42"), (0, -42));
}

#[test]
fn null_pointer_is_minus_one() {
    let mut out = 0i64;
    assert_eq!(unsafe { parse_qty(std::ptr::null(), 0, &mut out) }, -1);
}

#[test]
fn invalid_utf8_is_minus_two() {
    let bytes = [0xffu8, 0xfe];
    let mut out = 0i64;
    assert_eq!(
        unsafe { parse_qty(bytes.as_ptr(), bytes.len(), &mut out) },
        -2
    );
}

#[test]
fn bad_number_is_minus_three() {
    assert_eq!(call("abc").0, -3);
}
