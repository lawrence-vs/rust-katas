use kata_traits_13::largest;

#[test]
fn integers() {
    assert_eq!(largest(&[3, 9, 2]), Some(&9));
}

#[test]
fn strings() {
    assert_eq!(largest(&["pear", "apple"]), Some(&"pear"));
}

#[test]
fn floats() {
    assert_eq!(largest(&[0.5, 2.5, 1.0]), Some(&2.5));
}

#[test]
fn empty() {
    assert_eq!(largest::<i32>(&[]), None);
}
