use std::io::{self, Read};

use kata_errors_124::{DuplicateId, Table, read_all};

struct Broken;

impl Read for Broken {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "disk gone"))
    }
}

fn garbage() -> impl Read {
    [0xff_u8].as_slice()
}

#[test]
fn a_failed_read_is_an_error_not_an_empty_string() {
    let err = read_all(Broken).expect_err("a failing source must be an Err");
    assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
}

#[test]
fn invalid_utf8_is_an_error() {
    let err = read_all(garbage()).expect_err("bytes that are not UTF-8 must be an Err");
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn a_good_source_is_read_in_full() {
    assert_eq!(
        read_all("id,amount\n1,5\n".as_bytes()).unwrap(),
        "id,amount\n1,5\n"
    );
}

#[test]
fn an_empty_source_is_an_empty_string() {
    assert_eq!(read_all(io::empty()).unwrap(), "");
}

#[test]
fn inserting_the_same_id_twice_is_an_error_on_the_second() {
    let mut table = Table::new();
    assert_eq!(table.insert(7), Ok(()));
    assert_eq!(table.insert(7), Err(DuplicateId(7)));
    assert_eq!(table.len(), 1);
}

#[test]
fn batch_with_a_duplicate_id_is_an_error() {
    let mut table = Table::new();
    assert_eq!(table.insert_all(&[1, 2, 2, 3]), Err(DuplicateId(2)));
}

#[test]
fn batch_stops_at_the_first_failure() {
    let mut table = Table::new();
    let _ = table.insert_all(&[1, 2, 2, 3]);
    assert_eq!(table.len(), 2);
}

#[test]
fn a_clean_batch_counts_every_row() {
    let mut table = Table::new();
    assert_eq!(table.insert_all(&[1, 2, 3]), Ok(3));
    assert!(!table.is_empty());
}
