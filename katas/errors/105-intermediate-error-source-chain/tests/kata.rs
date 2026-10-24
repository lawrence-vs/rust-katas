use std::error::Error;
use std::io::{self, ErrorKind, Read, Write};

use kata_errors_105::{CopyError, copy, io_kind};

struct BrokenReader(ErrorKind);

impl Read for BrokenReader {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from(self.0))
    }
}

struct BrokenWriter(ErrorKind);

impl Write for BrokenWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(self.0))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn copies_every_byte() {
    let mut out = Vec::new();
    let n = copy(&mut &b"a ledger line, longer than one buffer"[..], &mut out).unwrap();
    assert_eq!(n, 37);
    assert_eq!(out, b"a ledger line, longer than one buffer");
}

#[test]
fn empty_input_copies_nothing() {
    let mut out = Vec::new();
    assert_eq!(copy(&mut io::empty(), &mut out).unwrap(), 0);
    assert!(out.is_empty());
}

#[test]
fn display_does_not_repeat_the_cause() {
    let err = copy(&mut BrokenReader(ErrorKind::WouldBlock), &mut Vec::new()).unwrap_err();
    assert!(!err.to_string().contains("would block"));
}

#[test]
fn source_reaches_the_io_error() {
    let err = copy(&mut BrokenReader(ErrorKind::WouldBlock), &mut Vec::new()).unwrap_err();
    assert!(
        err.source().is_some(),
        "source_reaches_the_io_error: source() is None"
    );
}

#[test]
fn walking_the_chain_finds_would_block() {
    let err = copy(&mut BrokenReader(ErrorKind::WouldBlock), &mut Vec::new()).unwrap_err();
    assert_eq!(io_kind(&err), Some(ErrorKind::WouldBlock));
}

#[test]
fn walking_the_chain_finds_the_write_side_kind_too() {
    let err = copy(&mut &b"x"[..], &mut BrokenWriter(ErrorKind::BrokenPipe)).unwrap_err();
    assert_eq!(io_kind(&err), Some(ErrorKind::BrokenPipe));
}

#[test]
fn a_reader_that_always_errors_yields_in() {
    let err = copy(&mut BrokenReader(ErrorKind::TimedOut), &mut Vec::new()).unwrap_err();
    assert!(matches!(err, CopyError::In(_)), "got {err:?}");
}

#[test]
fn a_writer_that_always_errors_yields_out() {
    let err = copy(&mut &b"x"[..], &mut BrokenWriter(ErrorKind::TimedOut)).unwrap_err();
    assert!(matches!(err, CopyError::Out(_)), "got {err:?}");
}
