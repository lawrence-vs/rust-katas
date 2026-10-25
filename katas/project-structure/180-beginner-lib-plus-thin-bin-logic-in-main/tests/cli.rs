// Runs the real `tally` binary. Passes before and after the fix: behaviour must not change.
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tally"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // Dropping stdin after the write closes it, so the child sees EOF.
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn lines_prints_the_count() {
    let out = run(&["lines"], b"one two\nthree\n");
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "2\n");
    assert_eq!(String::from_utf8(out.stderr).unwrap(), "");
}

#[test]
fn words_prints_the_count() {
    let out = run(&["words"], b"one two\nthree\n");
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "3\n");
}

#[test]
fn bytes_prints_the_count() {
    let out = run(&["bytes"], b"one two\nthree\n");
    assert!(out.status.success());
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "14\n");
}

#[test]
fn no_mode_exits_2_with_usage_on_stderr() {
    let out = run(&[], b"");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "");
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "error: usage: tally <lines|words|bytes>\n"
    );
}

#[test]
fn an_unknown_mode_exits_2() {
    let out = run(&["chars"], b"");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "error: unknown mode 'chars': expected lines, words or bytes\n"
    );
}

#[test]
fn too_many_arguments_exits_2() {
    let out = run(&["lines", "words"], b"");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(
        String::from_utf8(out.stderr).unwrap(),
        "error: expected one argument: lines, words or bytes\n"
    );
}

#[test]
fn invalid_utf8_on_stdin_exits_1() {
    let out = run(&["bytes"], &[0xff, 0xfe]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(String::from_utf8(out.stdout).unwrap(), "");
    assert!(
        String::from_utf8(out.stderr)
            .unwrap()
            .starts_with("error: reading stdin:")
    );
}
