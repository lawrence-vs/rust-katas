use kata_ownership_01::replace;

#[test]
fn replace_writes_through_the_reference() {
    let mut x = 1;
    assert_eq!(replace(&mut x, 5), 1);
    assert_eq!(x, 5);
}
