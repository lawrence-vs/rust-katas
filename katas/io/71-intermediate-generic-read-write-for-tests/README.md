# 71 intermediate: a `run` that tests can call without files

**Concept:** Take `R: Read` and `W: Write` by value and return write errors, so tests pass `&[u8]` and `Vec<u8>` while `main` passes a `File` and locked stdout.
**Source:** Rust API Guidelines, [C-RW-VALUE](https://rust-lang.github.io/api-guidelines/interoperability.html#c-rw-value); the std::io docs for [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html), [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) and [`Stdout::lock`](https://doc.rust-lang.org/std/io/struct.Stdout.html#method.lock)
**Fails with:** `E0061`
**Vault:** your note on injecting IO instead of opening it

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Update `src/main.rs` to match. None of the following in `src/lib.rs`: `unwrap`, `expect`, `panic!`, `println!`, `#[allow(...)]`.

## Before you fix it
1. Read the E0061 error: what does the test pass that `run` can't accept?
2. What do `&[u8]` and `File` have in common?

## Answer in the PR
1. Why can't the old `run` be unit-tested, and what does a test have to set up instead?
2. Why take `R` and `W` by value, and how does a caller keep using their writer afterwards?
3. What did `println!` do with a write error, and what does your `run` do?
4. Where does a read error show up (`Io` or `Csv`), and why?

## Think further
1. Should the `File` go in a `BufReader`? (csv already buffers.)
2. Does `stdout().lock()` need a `BufWriter`? Why does dropping a `BufWriter` swallow flush errors?
3. Should `cargo run | head` report a broken pipe or exit 0?
4. When could the `i64` sum overflow here?
5. `impl Read` in argument position versus `<R: Read>`.

## In the wild
- `csv::Reader::from_reader`
- `serde_json::to_writer`
- `flate2::read::GzDecoder<R>`
- A ledger export tested against a `Vec<u8>`.

In a review, check:
- A `&str` or `&Path` parameter in a library API where `R: Read` would fit.
- `println!` in library code.
- `&mut dyn Write` where a generic would do.
