use clap::{Parser, error::ErrorKind};
use kata_cli_91::{Args, Format};

fn kind(args: &[&str]) -> ErrorKind {
    match Args::try_parse_from(args) {
        Ok(parsed) => panic!("expected an error, got {parsed:?}"),
        Err(e) => e.kind(),
    }
}

#[test]
fn all_flags_are_parsed_into_typed_fields() {
    let args = Args::try_parse_from([
        "report",
        "trades.csv",
        "--limit",
        "5",
        "--format",
        "json",
        "-v",
    ])
    .unwrap();
    assert_eq!(args.input, std::path::PathBuf::from("trades.csv"));
    assert_eq!(args.limit, 5);
    assert_eq!(args.format, Format::Json);
    assert!(args.verbose);
}

#[test]
fn defaults_apply_when_flags_are_absent() {
    let args = Args::try_parse_from(["report", "trades.csv"]).unwrap();
    assert_eq!(args.limit, 10);
    assert_eq!(args.format, Format::Text);
    assert!(!args.verbose);
}

#[test]
fn a_missing_input_is_an_error() {
    assert_eq!(kind(&["report"]), ErrorKind::MissingRequiredArgument);
}

#[test]
fn a_non_number_limit_is_rejected() {
    assert_eq!(
        kind(&["report", "t.csv", "--limit", "abc"]),
        ErrorKind::ValueValidation
    );
}

#[test]
fn a_limit_outside_the_range_is_rejected() {
    assert_eq!(
        kind(&["report", "t.csv", "--limit", "0"]),
        ErrorKind::ValueValidation
    );
    assert_eq!(
        kind(&["report", "t.csv", "--limit", "1001"]),
        ErrorKind::ValueValidation
    );
}

#[test]
fn a_format_outside_the_allowed_list_is_rejected() {
    assert_eq!(
        kind(&["report", "t.csv", "--format", "xml"]),
        ErrorKind::InvalidValue
    );
}

#[test]
fn an_unknown_flag_is_rejected() {
    assert_eq!(
        kind(&["report", "t.csv", "--limt", "5"]),
        ErrorKind::UnknownArgument
    );
}

#[test]
fn help_is_generated_from_the_struct() {
    let e = Args::try_parse_from(["report", "--help"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::DisplayHelp);
    let text = e.to_string();
    assert!(text.contains("--limit"), "{text}");
    assert!(text.contains("[default: 10]"), "{text}");
    assert!(text.contains("[possible values: text, json]"), "{text}");
}
