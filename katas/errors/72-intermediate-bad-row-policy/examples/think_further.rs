//! 72 intermediate: a bad-row policy
//!
//! Think further, from the kata README:
//! 1. A threshold policy: fail if more than N% of rows are bad.
//! 2. Return the bad rows with their line numbers (a dead-letter file) instead of a count.
//! 3. How does `ReaderBuilder::flexible(true)` change what counts as bad?
//! 4. Is a non-UTF-8 row a bad row or an I/O error?
//! 5. Kata 71's `W: Write` for the log.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// Think further 1 and 2: keep the bad rows with their line numbers, and apply a threshold.
fn main() {
    let data = "ada,36\nbob,x\ncarol,41\n,22\ndan,30\n";
    let mut good = 0;
    let mut bad: Vec<(usize, String)> = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    for (i, record) in reader.records().enumerate() {
        let line = i + 1;
        match record {
            Ok(row) if row.len() == 2 && !row[0].is_empty() && row[1].parse::<u32>().is_ok() => {
                good += 1;
            }
            Ok(row) => bad.push((line, format!("{row:?}"))),
            Err(e) => bad.push((line, e.to_string())),
        }
    }

    println!("good rows: {good}");
    println!("bad rows (dead-letter): {bad:?}");

    let total = good + bad.len();
    let pct = bad.len() * 100 / total.max(1);
    if pct > 20 {
        println!("refusing the file: {pct}% bad");
    } else {
        println!("accepting the file: {pct}% bad");
    }
}
