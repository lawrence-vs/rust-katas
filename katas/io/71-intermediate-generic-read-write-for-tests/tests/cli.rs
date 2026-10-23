use std::fs;
use std::path::Path;
use std::process::Command;

const TRADES: &str = "symbol,side,qty\nBTC,buy,3\nETH,sell,2\nBTC,sell,1\n";

#[test]
fn the_binary_reads_a_file_and_prints_to_stdout() {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("trades.csv");
    fs::write(&path, TRADES).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kata-io-71"))
        .arg(&path)
        .output()
        .unwrap();
    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "BTC 2\nETH -2\n");
}

#[test]
fn a_missing_file_exits_non_zero_with_an_error() {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("does-not-exist.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_kata-io-71"))
        .arg(&path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.starts_with("error: "), "stderr: {stderr}");
}
