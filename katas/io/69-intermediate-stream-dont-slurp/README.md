# 69 intermediate: stream, don't slurp

**Concept:** Hand the reader to the CSV parser and fold one record at a time: memory stays flat whatever the file size.
**Source:** the csv docs for [`Reader`](https://docs.rs/csv/latest/csv/struct.Reader.html), [`Reader::deserialize`](https://docs.rs/csv/latest/csv/struct.Reader.html#method.deserialize) and [`ReaderBuilder::trim`](https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html#method.trim), plus the csv tutorial's performance section; katas 08 and 35
**Fails with:** `peak allocation grew with the file`
**Vault:** your note on streaming input instead of loading it

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. None of the following in `src/`: `unwrap`, `expect`, `panic!`, `#[allow(...)]`.

## Before you fix it
1. What peak did the test print, and roughly how many bytes per row is that?
2. Which two lines hold the whole file?

## Answer in the PR
1. What held memory at the peak in the old code, and how did it scale with N rows?
2. What does `csv::Reader` keep between two `next()` calls, and why is that enough?
3. Why take `R: Read` rather than `&Path` or `&str`? Name two callers that gain.
4. A bad row on line 900,000: what has the streaming version already done by then, and what does the caller get back?

## Think further
1. Is wrapping the input in `BufReader` useful here?
2. One 2 GB field: is memory still flat?
3. `read_byte_record` with a reused `ByteRecord` and borrowed `&str` fields, versus one `String` per row.
4. Gzipped input via any `Read` adapter.
5. Bad-row policy, which kata 72 picks up.

## In the wild
- `serde_json::Deserializer::from_reader(..).into_iter()`
- `BufRead::lines`
- `tokio::io::AsyncBufReadExt::lines`
- End-of-day trade and ledger reconciliation files.

In a review, check:
- `fs::read_to_string`, `read_to_end` or `collect::<Vec<_>>()` on input whose size the caller doesn't control.
- A `&str` or `&Path` parameter where `R: Read` would fit.
