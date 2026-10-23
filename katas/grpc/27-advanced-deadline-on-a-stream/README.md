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

## Think further
1. The stream is slow, not stalled: one tick every 900 ms, and you want 5. What does your version return? Is that what a trading screen wants?
2. The deadline fires after 3 of 5 ticks. You return `DeadlineExceeded` and throw away 3 good prices. When would returning the partial result be better, and how would the signature have to change to say so?
3. The client gives up and drops the stream. What happens to the server's stream: does it keep producing, and how would it find out?
4. Service A calls B with 2 s left, and B calls C. How long should B give C?

## In the wild
- [gRPC deadlines guide](https://grpc.io/docs/guides/deadlines/): deadline propagation, and servers checking for cancellation.
- [gRPC status codes](https://grpc.io/docs/guides/status-codes/): `DEADLINE_EXCEEDED` vs `UNAVAILABLE`, and which ones are safe to retry.
- [gRPC retry guide](https://grpc.io/docs/guides/retry/): retry policy and budgets.
- [tonic `Endpoint::timeout`](https://docs.rs/tonic/0.14.6/tonic/transport/struct.Endpoint.html#method.timeout): its docs say it doesn't tell the server.
- [gRPC keepalive](https://grpc.io/docs/guides/keepalive/): detects a dead connection, but not a live server that has gone quiet.
- Exchange-style: market-data feeds send heartbeats so a client can tell a quiet market from a dead feed, e.g. a WebSocket heartbeat message.

In a review, check:
- `while let Some(..) = stream.next().await` on a long-lived stream with no deadline and no heartbeat.
- A timeout reported as `internal`/`unknown`, or turned into a panic, instead of a deadline status.
- Retrying `DeadlineExceeded` with no backoff or retry budget, or retrying a call that isn't idempotent (placing an order).
- A timeout set on the `Endpoint`, or as `grpc-timeout` metadata, and assumed to cover the whole stream.
