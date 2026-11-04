use kata_project_structure_222::word_count;

#[test]
fn counts_words_split_on_any_whitespace() {
    assert_eq!(word_count("a b  c"), 3);
}

#[test]
fn empty_input_has_no_words() {
    assert_eq!(word_count(""), 0);
}
