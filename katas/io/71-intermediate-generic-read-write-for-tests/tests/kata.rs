use std::io::{self, Write};

use kata_io_71::{Error, run};

const TRADES: &str = "symbol,side,qty\nBTC,buy,3\nETH,sell,2\nBTC,sell,1\n";

#[test]
fn nets_each_symbol_from_a_byte_slice() {
    let mut out = Vec::new();
    run(TRADES.as_bytes(), &mut out).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "BTC 2\nETH -2\n");
}

#[test]
fn a_header_only_input_writes_nothing() {
    let mut out = Vec::new();
    let result = run("symbol,side,qty\n".as_bytes(), &mut out);
    assert!(result.is_ok(), "expected Ok, got {result:?}");
    assert!(out.is_empty(), "expected no output, got {out:?}");
}

#[test]
fn a_bad_row_is_a_csv_error_not_a_panic() {
    let mut out = Vec::new();
    let result = run("symbol,side,qty\nBTC,buy,x\n".as_bytes(), &mut out);
    assert!(
        matches!(result, Err(Error::Csv(_))),
        "expected Err(Error::Csv(_)), got {result:?}"
    );
}

struct Full;

impl Write for Full {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("disk full"))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn a_failing_writer_is_an_io_error_not_a_panic() {
    let result = run(TRADES.as_bytes(), Full);
    assert!(
        matches!(result, Err(Error::Io(_))),
        "expected Err(Error::Io(_)), got {result:?}"
    );
}
