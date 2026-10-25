#![cfg(feature = "unicode")]
use kata_project_structure_183::{slug, slug_unicode};

#[test]
fn slug_unicode_folds_accents() {
    assert_eq!(slug_unicode("Caf\u{e9} Menu"), "cafe-menu");
}

#[test]
fn slug_unicode_folds_every_mark() {
    assert_eq!(slug_unicode("Cr\u{e8}me Br\u{fb}l\u{e9}e"), "creme-brulee");
}

#[test]
fn slug_unicode_keeps_ascii_rules() {
    assert_eq!(slug_unicode("  Hello, World!  "), "hello-world");
}

#[test]
fn unicode_does_not_change_slug() {
    assert_eq!(slug("Caf\u{e9} Menu"), "caf-menu");
}
