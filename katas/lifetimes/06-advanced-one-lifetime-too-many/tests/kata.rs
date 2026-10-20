use kata_lifetimes_06::{StrSplit, until_char};
#[test]
fn splits() {
    assert_eq!(
        StrSplit::new("a b c", " ").collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
}
#[test]
fn until_char_borrows_only_the_haystack() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
