use kata_strings_32::{initial, truncate};

#[test]
fn initial_of_an_ascii_name() {
    assert_eq!(initial("ada"), Some('a'));
}

#[test]
fn initial_is_a_whole_char_not_a_byte() {
    assert_eq!(initial("Émile"), Some('É'));
    assert_eq!(initial("€uro"), Some('€'));
}

#[test]
fn empty_name_has_no_initial() {
    assert_eq!(initial(""), None);
}

#[test]
fn truncate_counts_chars_not_bytes() {
    assert_eq!(truncate("Zürich", 2), "Zü");
    assert_eq!(truncate("Zürich", 6), "Zürich");
}

#[test]
fn truncate_short_or_zero() {
    assert_eq!(truncate("Zürich", 10), "Zürich");
    assert_eq!(truncate("ada", 0), "");
    assert_eq!(truncate("", 3), "");
}

#[test]
fn borrows_the_callers_string() {
    let owned = String::from("Émile");
    let short: &str = truncate(&owned, 1);
    assert_eq!(short, "É");
    assert_eq!(initial(&owned), Some('É'));
    assert_eq!(owned, "Émile");
}
