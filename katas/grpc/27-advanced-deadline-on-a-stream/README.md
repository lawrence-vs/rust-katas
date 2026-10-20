# 27 advanced: a deadline on a stream

**Concept:** A server stream can stall while its connection stays open; `next().await` then waits forever. A client deadline turns a hang into `DeadlineExceeded`, which callers can retry or report.
**Source:** tonic routeguide tutorial (server streaming), https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md; tokio `timeout` docs, https://docs.rs/tokio/1.53.2/tokio/time/fn.timeout.html
**Fails with:** `first_n hung`
**Vault:** your note on deadlines in RPC

## Task
Make `cargo test` pass without changing the test, `proto/`, `build.rs` or `Cargo.toml`. Return `Status::deadline_exceeded` when the deadline fires.

## Before you fix it
1. The server sent two ticks and went quiet. What is `stream.next()` waiting for?
2. Should the deadline cover each message, or the whole call?

## Answer in the PR
1. Whole-call deadline or per-message timeout: which did you pick, and when is the other right?
2. Why does `Request::set_timeout` alone not cover a stream that stalls after its headers?
3. What should a caller do with `DeadlineExceeded`?
