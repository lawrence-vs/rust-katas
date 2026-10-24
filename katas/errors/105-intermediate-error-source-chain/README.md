# 105 intermediate: an error source chain

**Concept:** An error that wraps another error should hand it back through `Error::source`, so a caller holding only a `&dyn Error` can walk the chain and `downcast_ref` to the cause; a wrapper variant must also name the stream that actually failed.
**Source:** Rust for Rustaceans (Gjengset), ch. 4 Error Handling, "Enumeration" and "Opaque Errors"; std `Error::source` docs
**Fails with:** `source_reaches_the_io_error`
**Vault:** your note on error source chains

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. In `src/`: no `unwrap`, `expect`, `panic!` or `#[allow]`. Std only; do not add `thiserror`.

## Before you fix it
1. Run `cargo test`. Which tests fail, and which of them fail for a different reason than the first?
2. Read the default body of `Error::source` in the std docs. What does it return?
3. Three tests pass on the starting code. Why do they, and what do they not look at?

## Answer in the PR
1. What does `source()` return for `CopyError` today, and why does `Display` not need to change?
2. Why should `Display` not repeat the text of the cause when `source()` returns it?
3. Which variant is a read failure, and which is a write failure? Where does the doc comment on the variant say so?
4. What does `#[derive(Error)]` with `#[source]` write for you, and which part of it did you write by hand?
5. The `source()` return type is `Option<&(dyn Error + 'static)>`. Why `'static`, and what does it let `downcast_ref` do?

## Think further
1. What does a `Box<dyn Error>` field do to `downcast_ref` on the wrapper, and what does it do to `source()`?
2. Why is a `From<io::Error>` impl a poor fit for an enum with two variants that both wrap an `io::Error`?
3. Who should print the whole chain: the library, or the binary? What does an `anyhow`-style `{:#}` do with `source()`?
4. If `CopyError` returned its cause from both `source()` and `Display`, what would a log line show?
5. Run `cargo run --example think_further`. How many errors are in the chain, and which one does `downcast_ref::<io::Error>` find first?

## In the wild
- [`Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source): the default returns `None`, so a wrapper that forgets it silently cuts the chain.
- Error reporters walk `source()` to print "caused by" lines; a missing link means the root cause never reaches the log.
- A retry layer that checks for `ErrorKind::WouldBlock` or `TimedOut` deep in a chain needs the chain to be intact.

In a review, check:
- An `impl Error for X {}` where `X` wraps another error.
- A variant named for one side (`In`, `Read`) that is built from the other side's failure.
- A `Display` that formats the inner error and a `source()` that also returns it.
