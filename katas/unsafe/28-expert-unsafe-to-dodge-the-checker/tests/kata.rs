use kata_unsafe_28::push_one_then_first;

#[test]
fn appends_one_then_the_first() {
    let mut v = Vec::with_capacity(1);
    v.push(7);
    push_one_then_first(&mut v);
    assert_eq!(v, [7, 1, 7]);
}
