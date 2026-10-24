# 93 intermediate: one task per connection

**Concept:** An accept loop that awaits each handler serves one client at a time, so one slow client stalls everyone. Spawn a task per connection that owns its socket, and share state through an `Arc`.
**Source:** the tokio tutorial, "Spawning", https://tokio.rs/tokio/tutorial/spawning; *Network Programming with Rust* (Chanda) ch. 3, the TCP echo server (no Readwise highlight yet, so the chapter is the citation)
**Fails with:** `a stalled client blocked the next one`
**Vault:** your note on one task per connection

## Protocol
A client sends one line. The server replies `#<n> <line trimmed>\n` and closes. `n` counts the lines this server has answered, across all connections, starting at 1. A client that hangs up before sending a newline gets no reply and is not counted.

## Task
Make `cargo test` pass without changing the tests. Keep the `serve` signature. No `unwrap`/`expect`/`panic!` in `src/`. tokio only.

## Before you fix it
1. Walk the starting code through the stalled-client test: where is `serve` suspended?
2. Add `tokio::spawn` and read the compiler error. What does it say about `served`?

## Answer in the PR
1. Why did `&served` stop compiling once you spawned, and what does `Arc` change?
2. In the starting code, one client's IO error ended `serve`. Where does it go now, and is that right?
3. What does a task cost compared with an OS thread per connection?

## Think further
1. Nothing limits the number of tasks. What does a flood of connections cost, and how would a `Semaphore` bound it? (Kata 21.)
2. A client that connects and never sends still holds a task and a socket. What would you add? (Kata 22.)
3. `accept` can fail with "too many open files" (EMFILE). With `?`, what happens to the server?
4. Is `Ordering::Relaxed` enough for this counter? What would you need before it guarded other data? (Kata 43.)
5. `serve` stops, but spawned tasks keep running. How would you wait for them, or tell them to stop?

## In the wild
- [tokio tutorial: Spawning](https://tokio.rs/tokio/tutorial/spawning): the `'static` bound and why a spawned task owns its data.
- [mini-redis `server.rs`](https://github.com/tokio-rs/mini-redis/blob/master/src/server.rs): a task per connection, plus a `Semaphore` that caps connections.
- [`axum::serve`](https://docs.rs/axum/latest/axum/fn.serve.html) and hyper-util: one task per accepted connection.
- *Network Programming with Rust* ch. 3: the same server with `thread::spawn` per connection.

In a review, check:
- `.await` on a connection handler inside an accept loop.
- `?` on a per-client result inside the accept loop: one bad client ends the server.
- Spawned tasks with no limit.
