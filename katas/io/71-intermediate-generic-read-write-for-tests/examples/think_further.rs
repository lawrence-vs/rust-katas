//! 71 intermediate: a `run` that tests can call without files
//!
//! Think further, from the kata README:
//! 1. Should the `File` go in a `BufReader`? (csv already buffers.)
//! 2. Does `stdout().lock()` need a `BufWriter`? Why does dropping a `BufWriter` swallow flush errors?
//! 3. Should `cargo run | head` report a broken pipe or exit 0?
//! 4. When could the `i64` sum overflow here?
//! 5. `impl Read` in argument position versus `<R: Read>`.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::io::{self, BufWriter, Write};

// Think further 5: impl Write in argument position, so a test passes a Vec<u8> and main passes stdout.
fn write_report(out: &mut impl Write, rows: &[(&str, i64)]) -> io::Result<()> {
    for (name, amount) in rows {
        writeln!(out, "{name},{amount}")?;
    }
    Ok(())
}

// Think further 4: checked_add, so a sum that overflows is an error, not a wrong total.
fn total(rows: &[(&str, i64)]) -> Option<i64> {
    rows.iter().try_fold(0i64, |acc, (_, amount)| acc.checked_add(*amount))
}

fn main() -> io::Result<()> {
    let rows = [("ada", 500), ("grace", -120)];

    // A test writes into a Vec<u8> and checks the bytes.
    let mut buf: Vec<u8> = Vec::new();
    write_report(&mut buf, &rows)?;
    println!("in memory: {:?}", String::from_utf8_lossy(&buf));

    // Think further 2: a BufWriter over stdout, and flush before dropping so errors show up.
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    write_report(&mut out, &rows)?;
    out.flush()?;

    println!("total: {:?}", total(&rows));
    println!("overflow: {:?}", total(&[("big", i64::MAX), ("one", 1)]));
    Ok(())
}
