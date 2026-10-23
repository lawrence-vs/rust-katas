# 72 intermediate: a bad-row policy

**Concept:** Reading a batch needs an explicit policy for each record: a malformed row is skipped, counted and logged; an I/O error stops the load. `?` loses the good rows, and `.ok()` can't tell a typo from a dead disk.
**Source:** The Rust Book ch. 9 "Recoverable Errors with `Result`"; csv docs for `csv::Error::is_io_error`, `csv::ErrorKind` and `Position::line` (https://docs.rs/csv/1.4.0/csv/); the csv tutorial's error-handling section
**Fails with:** `a bad row aborted the batch`
**Vault:** your note on recoverable vs unrecoverable errors

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. Log each skipped row to stderr with its line number. None of `unwrap`, `expect`, `panic!` or `#[allow(...)]` in `src/`.

## Before you fix it
1. Run `cargo test`. Which variant does the bad row come back as?
2. Which test passes on the starting code, and what wrong fix is it there to catch?

## Answer in the PR
1. Why is `?` wrong here, and why is `.ok()` wrong too? Name the test each one fails.
2. How does your code tell a bad row from an I/O error? Why does one stop the load and the other not?
3. Did you keep `From<csv::Error> for LoadError`? What would it let a future `?` do?
4. A library printing with `eprintln!`: what does the caller lose, and what could it return or take instead?

## Think further
1. A threshold policy: fail if more than N% of rows are bad.
2. Return the bad rows with their line numbers (a dead-letter file) instead of a count.
3. How does `ReaderBuilder::flexible(true)` change what counts as bad?
4. Is a non-UTF-8 row a bad row or an I/O error?
5. Kata 71's `W: Write` for the log.

## In the wild
- An end-of-day trade file reconciled against a ledger: one corrupt line must not hide the other 49,999, and the skipped count must show up in the reconciliation.
- Bank statement imports.
- Log pipelines that drop unparsable lines and count them as a metric.
- PostgreSQL 17 `COPY ... (ON_ERROR ignore)` skips rows that fail to convert and reports how many.

In a review, check:
- `?` inside a per-record loop.
- `.ok()` / `filter_map(Result::ok)` over I/O results.
- A `From` impl that merges unrelated error kinds into one variant.
- A skip with no count.
