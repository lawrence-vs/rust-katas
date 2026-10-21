# 07 beginner: unwrap in a library

**Concept:** A library doesn't know what its caller wants to do with bad input, so it returns the error instead of panicking. `unwrap` belongs in tests and in `main`, not in library code.
**Source:** Network Programming With Rust (Chanda), methods that panic: expect() and unwrap(), https://readwise.io/open/954432478
**Fails with:** `E0308`
**Vault:** your note on panics vs errors

## Task
Make `cargo test` pass without changing the test. No `unwrap`, `expect` or `panic!` in `src/`.

## Before you fix it
1. What type does `str::parse::<u16>` return before the `unwrap`?
2. The test compares with `Ok(8080)`. What return type does that imply?

## Answer in the PR
1. Who should decide what happens on bad input, and why not the library?
2. Why is `"70000"` an error here? Which type enforced that?
3. Where in a program is `unwrap` acceptable?

## Think further
1. After it passes, call it with `"+80"`, `"0"`, `""` and `"8080\n"`. Which come back as a port? Port 0 is a valid `u16` but tells the OS "pick any port": should this function reject it, and how would a caller learn why?
2. Print the error for `"http"` and for `"70000"`, once with `{}` and once with `{:?}`. Is either good enough to show the person who typed the flag?
3. A binary calls `parse_port` on a command-line flag. Where does the error turn into a message and an exit code now? What does `fn main() -> Result<(), E>` print when it returns `Err`, and what must `E` implement?

## In the wild
- [The Book, ch. 9.3, To `panic!` or Not to `panic!`](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html): when a panic is the right call (examples, tests, a broken invariant) and when the caller should decide.
- [`tokio::net::TcpListener::bind`](https://docs.rs/tokio/1.54.0/tokio/net/struct.TcpListener.html#method.bind): a port already in use is a normal outcome for a server, so `bind` hands it back to the caller. Read its signature.
- [`std::net::AddrParseError`](https://doc.rust-lang.org/std/net/struct.AddrParseError.html): what std gives back when `"127.0.0.1:80x"` won't parse as an address. std's own text parsing doesn't panic on bad input.
- [API Guidelines, C-FAILURE](https://rust-lang.github.io/api-guidelines/documentation.html#c-failure): public functions document their errors and their panics. A library function that can panic on input has to say so.
- [Clippy lint list](https://clippy.rust-lang.org/master/index.html): search for `unwrap_used` and `expect_used`. Which group are they in, and what does a crate have to do to turn them on?
- Exchange-style: the order gateway parses a quantity from every inbound message. A panic in that parser kills the connection task (or the whole process with `panic = "abort"`), so one bad message from one client becomes an outage for everyone. An error becomes a reject sent back to that one client.

In a review, check:
- `unwrap` or `expect` under `src/` on anything that came from outside the program: input, files, the network, config.
- `expect("...")` messages that claim an invariant. Is it really one, and who guarantees it?
- Public functions that can still panic: is there a `# Panics` section in the docs?
