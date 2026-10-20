use kata_ownership_04::total_len;
#[test]
fn borrows_instead_of_taking() {
    let names = vec![String::from("ada"), String::from("grace")];
    assert_eq!(total_len(&names), 8);
    assert_eq!(names.len(), 2);
}
