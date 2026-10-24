use kata_errors_88::{Settings, SettingsError, load_settings, parse_settings};
use std::io::{self, Read};
use std::num::IntErrorKind;
use std::path::PathBuf;
use std::process::Command;

struct Broken;

impl Read for Broken {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("disk went away"))
    }
}

#[test]
fn a_good_file_parses() {
    let got = parse_settings("# demo\nport = 8080\n\nworkers=4\n");
    assert!(matches!(got, Ok(Settings { port: 8080, workers: 4 })));
}

#[test]
fn a_missing_key_is_named() {
    let err = parse_settings("port=8080\n").err();
    assert!(matches!(&err, Some(SettingsError::Missing { key }) if key == "workers"));
    assert_eq!(
        err.map(|e| e.to_string()).as_deref(),
        Some("missing required key `workers`")
    );
}

#[test]
fn a_bad_value_keeps_its_cause() {
    let Err(err) = parse_settings("port=99999\nworkers=4\n") else {
        panic!("expected an error");
    };
    match &err {
        SettingsError::BadValue { key, source } => {
            assert_eq!(key, "port");
            assert_eq!(source.kind(), &IntErrorKind::PosOverflow);
        }
        other => panic!("wrong variant: {other:?}"),
    }
    assert!(std::error::Error::source(&err).is_some());
    assert_eq!(err.to_string(), "invalid value for `port`");
    assert!(!err.to_string().contains("too large"));
}

#[test]
fn a_line_without_equals_names_its_line() {
    let got = parse_settings("port=1\nworkers 4\n");
    assert!(matches!(got, Err(SettingsError::Malformed { line: 2 })));
}

#[test]
fn an_io_error_is_its_own_variant() {
    match load_settings(Broken) {
        Err(SettingsError::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::Other),
        other => panic!("wrong result: {other:?}"),
    }
}

#[test]
fn the_library_does_not_use_anyhow() {
    assert!(!include_str!("../src/lib.rs").contains("anyhow"));
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("{name}-{}.conf", std::process::id()))
}

fn run(path: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kata-errors-88"))
        .arg(path)
        .output()
        .expect("the binary should start")
}

#[test]
fn the_binary_names_the_file_it_could_not_read() {
    let path = temp_path("kata-88-missing");
    let out = run(&path);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(stderr.contains(path.to_string_lossy().as_ref()), "{stderr}");
    assert!(stderr.contains("Caused by:"), "{stderr}");
}

#[test]
fn the_binary_shows_the_whole_chain_for_a_bad_value() {
    let path = temp_path("kata-88-bad");
    std::fs::write(&path, "port=99999\nworkers=4\n").expect("write temp file");
    let out = run(&path);
    let _ = std::fs::remove_file(&path);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success());
    assert!(stderr.contains(path.to_string_lossy().as_ref()), "{stderr}");
    assert!(stderr.contains("invalid value for `port`"), "{stderr}");
    assert!(
        stderr.contains("number too large to fit in target type"),
        "{stderr}"
    );
}

#[test]
fn the_binary_runs_a_good_file() {
    let path = temp_path("kata-88-good");
    std::fs::write(&path, "port=8080\nworkers=4\n").expect("write temp file");
    let out = run(&path);
    let _ = std::fs::remove_file(&path);
    assert!(out.status.success());
}
