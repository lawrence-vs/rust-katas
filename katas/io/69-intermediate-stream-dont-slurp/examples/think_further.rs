//! 69 intermediate: stream, don't slurp
//!
//! Think further, from the kata README:
//! 1. Is wrapping the input in `BufReader` useful here?
//! 2. One 2 GB field: is memory still flat?
//! 3. `read_byte_record` with a reused `ByteRecord` and borrowed `&str` fields, versus one `String` per row.
//! 4. Gzipped input via any `Read` adapter.
//! 5. Bad-row policy, which kata 72 picks up.

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

fn main() {
    // A stream of rows read one at a time, never the whole file in memory.
    let data = "ada,36\ngrace,41\nlinus,28\n";

    // Think further 3: one reused ByteRecord, so each row doesn't allocate a new String.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let mut record = csv::ByteRecord::new();
    let mut total_age = 0u64;
    let mut rows = 0;
    while reader.read_byte_record(&mut record).unwrap_or(false) {
        rows += 1;
        if let Some(age) = record
            .get(1)
            .and_then(|f| std::str::from_utf8(f).ok())
            .and_then(|s| s.parse::<u64>().ok())
        {
            total_age += age;
        }
    }
    println!("{rows} rows, total age {total_age}");

    // Think further 1: BufReader is not needed here, csv already buffers its reads.
    // In the wild: serde_json can read a stream of values the same way, with Deserializer::from_reader.
}
