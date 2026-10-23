use kata_errors_72::{LoadError, Trade, load_trades};
use std::io::{self, Read};

/// Hands out `data`, then fails once, then reports end of file.
struct FailsOnce {
    data: &'static [u8],
    failed: bool,
}

impl Read for FailsOnce {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.data.is_empty() {
            let n = self.data.len().min(buf.len());
            buf[..n].copy_from_slice(&self.data[..n]);
            self.data = &self.data[n..];
            return Ok(n);
        }
        if !self.failed {
            self.failed = true;
            return Err(io::Error::other("disk went away"));
        }
        Ok(0)
    }
}

#[test]
fn one_bad_row_in_five_keeps_the_other_four() {
    let csv = "id,account,amount_cents\n1,acc-1,100\n2,acc-2,200\n3,acc-3,12x\n4,acc-4,400\n5,acc-5,500\n";
    let report = load_trades(csv.as_bytes()).expect("a bad row aborted the batch");
    let ids: Vec<u64> = report.trades.iter().map(|t| t.id).collect();
    assert_eq!(ids, [1, 2, 4, 5]);
    assert_eq!(
        report.skipped, 1,
        "a bad row was dropped without being counted"
    );
}

#[test]
fn every_kind_of_bad_row_is_counted() {
    let csv = "id,account,amount_cents\n1,acc-1,100\n2,acc-2,200\n4,acc-4\n-5,acc-5,500\n6,acc-6,abc\n7,acc-7,700\n";
    let report = load_trades(csv.as_bytes()).expect("a bad row aborted the batch");
    assert_eq!(report.trades.len(), 3);
    assert_eq!(
        report.skipped, 3,
        "a bad row was dropped without being counted"
    );
}

#[test]
fn a_clean_file_skips_nothing() {
    let csv = "id,account,amount_cents\n1,acc-1,100\n2,acc-2,-200\n3,acc-3,300\n";
    let report = load_trades(csv.as_bytes()).unwrap();
    let t = |id, account: &str, amount_cents| Trade {
        id,
        account: account.to_string(),
        amount_cents,
    };
    assert_eq!(
        report.trades,
        [t(1, "acc-1", 100), t(2, "acc-2", -200), t(3, "acc-3", 300)]
    );
    assert_eq!(report.skipped, 0);
}

#[test]
fn an_io_error_stops_the_load() {
    let reader = FailsOnce {
        data: b"id,account,amount_cents\n1,acc-1,100\n2,acc-2,200\n",
        failed: false,
    };
    assert!(
        matches!(load_trades(reader), Err(LoadError::Io(_))),
        "an I/O error was swallowed instead of stopping the load"
    );
}
