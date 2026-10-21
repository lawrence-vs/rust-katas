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

## Think further
1. The peer sends one byte every 4 seconds and never a newline. Does a deadline on the whole greeting behave like a deadline on each read? Which one stops this peer?
2. The peer sends 1 GiB with no newline, fast. How big does `line` get before any deadline fires? (Kata 23 meets the same problem.)
3. The deadline fires halfway through a line. What happens to the bytes already in the `BufReader`? Could the caller try again on the same connection?
4. Who should choose the timeout: a constant in this function, config, or the caller? What breaks when a slow mobile client and a co-located bot share one value?

## In the wild
- [hyper http1 `Builder::header_read_timeout`](https://docs.rs/hyper/latest/hyper/server/conn/http1/struct.Builder.html#method.header_read_timeout): hyper's defence against a client that is slow to send headers. Read its default.
- [tower-http `TimeoutLayer`](https://docs.rs/tower-http/latest/tower_http/timeout/struct.TimeoutLayer.html): a whole-request timeout for axum. Ask what it covers and what it doesn't (the time before the request reaches the handler, and streaming bodies).
- [Slowloris](https://en.wikipedia.org/wiki/Slowloris_(cyber_attack)): the classic attack built on a server waiting forever for a slow client.
- [reqwest `ClientBuilder`](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html): `timeout`, `connect_timeout` and `read_timeout`. The outbound side needs deadlines too. Read the defaults.
- [sqlx `PoolOptions::acquire_timeout`](https://docs.rs/sqlx-core/latest/sqlx_core/pool/struct.PoolOptions.html#method.acquire_timeout): waiting for a database connection is a network await as well.

In a review, check:
- `.read*().await`, `.next().await` or `.accept().await` on a socket with no timeout around it and no layer above it.
- A timeout per read inside a loop with no overall deadline: a slow drip resets it forever.
- A timeout error turned into `unwrap`, a panic, or a generic error that hides `TimedOut`.
- HTTP and database clients built with default settings and no deadlines.
