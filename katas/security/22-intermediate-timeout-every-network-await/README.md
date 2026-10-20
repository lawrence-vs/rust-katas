# 22 intermediate: a timeout on every network await

**Concept:** A peer that connects and says nothing holds a task, a socket and a buffer forever. Every await on the network needs a deadline; `tokio::time::timeout` puts one on any future.
**Source:** tokio `timeout` docs, https://docs.rs/tokio/1.53.2/tokio/time/fn.timeout.html; the Rust security gist in your Readwise, https://gist.github.com/auser/c3161f55a8393faa8af5ddda68c6befa
**Fails with:** `read_greeting hung`
**Vault:** your note on deadlines

## Task
Make `cargo test` pass without changing the test. Return an `io::Error` of kind `TimedOut` when the peer is silent.

## Before you fix it
1. The peer never writes. When does `read_line` return?
2. What type does `timeout(..).await` return, and how do you turn its error into `io::ErrorKind::TimedOut`?

## Answer in the PR
1. What does a silent peer cost the server per connection, and how many can an attacker open?
2. Why does the test pause time, and what would a real-time test cost?
3. Where else does a network service need a deadline? (name three awaits)
