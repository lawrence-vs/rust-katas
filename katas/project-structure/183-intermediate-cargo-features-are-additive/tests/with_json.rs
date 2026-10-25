#![cfg(feature = "json")]
use kata_project_structure_183::{slug, slugs_csv, slugs_json};

#[test]
fn slugs_json_is_a_json_array() {
    assert_eq!(
        slugs_json(&["Hello World", "Rust 2024"]),
        r#"["hello-world","rust-2024"]"#
    );
}

#[test]
fn slugs_json_of_nothing_is_an_empty_array() {
    assert_eq!(slugs_json(&[]), "[]");
}

#[test]
fn json_does_not_remove_or_change_anything() {
    assert_eq!(slugs_csv(&["A b", "C"]), "a-b,c");
    assert_eq!(slug("Caf\u{e9} Menu"), "caf-menu");
}
