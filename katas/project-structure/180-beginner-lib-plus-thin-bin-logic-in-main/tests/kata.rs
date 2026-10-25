// These tests call the library directly; they need a lib target to import.
use kata_project_structure_180::{Mode, UsageError, count, parse_mode};

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn each_mode_parses() {
    assert_eq!(parse_mode(&args(&["lines"])), Ok(Mode::Lines));
    assert_eq!(parse_mode(&args(&["words"])), Ok(Mode::Words));
    assert_eq!(parse_mode(&args(&["bytes"])), Ok(Mode::Bytes));
}

#[test]
fn no_mode_is_a_usage_error() {
    assert_eq!(parse_mode(&args(&[])), Err(UsageError::MissingMode));
}

#[test]
fn an_unknown_mode_is_named_in_the_error() {
    assert_eq!(
        parse_mode(&args(&["chars"])),
        Err(UsageError::UnknownMode("chars".to_string()))
    );
    assert_eq!(
        UsageError::UnknownMode("chars".to_string()).to_string(),
        "unknown mode 'chars': expected lines, words or bytes"
    );
}

#[test]
fn extra_arguments_are_a_usage_error() {
    assert_eq!(
        parse_mode(&args(&["lines", "words"])),
        Err(UsageError::TooManyArgs)
    );
}

#[test]
fn the_usage_message_names_every_mode() {
    assert_eq!(
        UsageError::MissingMode.to_string(),
        "usage: tally <lines|words|bytes>"
    );
}

#[test]
fn counts_lines_words_and_bytes() {
    let text = "one two\nthree\n";
    assert_eq!(count(Mode::Lines, text), 2);
    assert_eq!(count(Mode::Words, text), 3);
    assert_eq!(count(Mode::Bytes, text), 14);
}

#[test]
fn empty_text_counts_zero() {
    assert_eq!(count(Mode::Lines, ""), 0);
    assert_eq!(count(Mode::Words, ""), 0);
    assert_eq!(count(Mode::Bytes, ""), 0);
}

#[test]
fn bytes_are_not_chars() {
    assert_eq!(count(Mode::Bytes, "héllo"), 6);
    assert_eq!(count(Mode::Words, "héllo"), 1);
}

#[test]
fn a_last_line_without_a_newline_still_counts() {
    assert_eq!(count(Mode::Lines, "a\nb"), 2);
}
