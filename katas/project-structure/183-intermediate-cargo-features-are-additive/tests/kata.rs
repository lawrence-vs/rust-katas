// Run all of: cargo test, --no-default-features, --features unicode, --features json, --all-features.
use kata_project_structure_183::{slug, slugs_csv};

#[test]
fn slug_lowercases_and_joins_words() {
    assert_eq!(slug("  Hello, World!  "), "hello-world");
}

#[test]
fn slug_keeps_digits() {
    assert_eq!(slug("Rust 2024"), "rust-2024");
}

#[test]
fn slug_of_empty_or_punctuation_is_empty() {
    assert_eq!(slug(""), "");
    assert_eq!(slug("!!!"), "");
}

#[test]
fn slug_is_the_same_in_every_build() {
    assert_eq!(
        slug("Caf\u{e9} Menu"),
        "caf-menu",
        "slug changed with a feature on; a feature must only add"
    );
}

#[test]
fn slug_drops_non_ascii() {
    assert_eq!(slug("Cr\u{e8}me Br\u{fb}l\u{e9}e"), "cr-me-br-l-e");
}

#[test]
fn slugs_csv_joins_with_commas() {
    assert_eq!(
        slugs_csv(&["Hello World", "Rust 2024"]),
        "hello-world,rust-2024"
    );
}

#[test]
fn slugs_csv_of_nothing_is_empty() {
    assert_eq!(slugs_csv(&[]), "");
}
