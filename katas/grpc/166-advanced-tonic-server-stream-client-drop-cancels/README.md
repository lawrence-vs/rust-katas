# 166 grpc (advanced): a server stream that outlives its client

**Concept:** A server stream is a producer task plus a channel. An unbounded channel never pushes back, so a slow client makes it grow. Nothing tells the task the client left, so it runs forever. A bounded channel gives back-pressure, and a closed receiver is the cancel signal.
**Source:** tonic's server-streaming example in the [routeguide tutorial](https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md); [`tokio_stream::wrappers::ReceiverStream`](https://docs.rs/tokio-stream/0.1.19/tokio_stream/wrappers/struct.ReceiverStream.html); [tokio `mpsc`](https://docs.rs/tokio/1.53.2/tokio/sync/mpsc/index.html).
**Fails with:** `producer still running after client drop`
**Vault:** your note on back-pressure and cancellation

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`Stats`, `StatementService`, `STREAM_BUFFER`, `pb`) and keep the `WatchStream` type. In `src/`, no `unwrap`, `expect`, `panic!` or `unsafe`. No new crates. The buffer size is `STREAM_BUFFER`. Kata 27 covers the deadline on a stream; this one is client disconnect and back-pressure.

## Before you fix it
1. Run `cargo test`. Which tests pass, and why do they pass?
2. Count `stats.produced()` for a client that read once and stopped. Where do those entries live?
3. What does the producer do after the client leaves?

## Answer in the PR
1. What did the channel's capacity change for a slow client, and what does the producer do when the channel is full?
2. How does the producer learn the client has gone? Which value was dropped, and what does the failing call return?
3. Why does the producer need to stop by itself and not wait to be told?
4. Why `STREAM_BUFFER = 4` and not 4,000 or 0? What does each choice cost (latency, memory, a stalled producer)?
5. A stream per client, 10,000 clients: what is the worst-case memory before and after your fix?

## Think further
1. The producer is blocked in `send` on a full channel and the client vanishes without a clean close, for example a dead network. What wakes the task? (Hint: gRPC keepalive; `Endpoint::http2_keep_alive_interval`.)
2. When is dropping the oldest entry for a slow client the right policy instead of blocking? What would the channel type be?
3. One producer feeds many clients. Why does one bounded channel per client plus a blocking producer let the slowest client stall the others?
4. How do you test "stops eventually" without `sleep` in the assertion?

## In the wild
- [gRPC cancellation guide](https://grpc.io/docs/guides/cancellation/): the server learns of a cancel, and should stop work.
- [gRPC flow control](https://grpc.io/docs/guides/flow-control/): HTTP/2 windows and back-pressure.
- [tokio `mpsc` docs](https://docs.rs/tokio/1.53.2/tokio/sync/mpsc/index.html): bounded vs unbounded, and the memory note.
- [Tonic `Streaming` docs](https://docs.rs/tonic/0.14.6/tonic/struct.Streaming.html).
- Exchange-style: a market-data gateway fans a feed out per client through a bounded queue and disconnects the slow consumer.

In a review, check:
- `unbounded_channel` in a handler
- `let _ = tx.send(..)` ignoring the error
- `tokio::spawn` with no stop condition
- a spawned task holding a sender, with no counter or metric

## Related concepts
### `try_send`
Fails at once when the channel is full or closed. Use it to drop or shed load instead of waiting.
```rust
if tx.try_send(item).is_err() {
    dropped += 1;
}
```

### `Sender::closed()`
A future that completes when the receiver is dropped. Use it in a `select!` to notice a gone consumer while the producer is idle.
```rust
tokio::select! {
    _ = tx.closed() => return,
    _ = tick.tick() => {}
}
```

### `CancellationToken`
From `tokio-util` (snippet only, not a dependency). A shared flag for cancelling several tasks at once.
```rust
let t = CancellationToken::new();
let c = t.clone();
t.cancel();
c.cancelled().await;
```

### `broadcast` channel
One sender and many receivers. A receiver that falls behind gets `Lagged` instead of blocking the sender.
```rust
let (tx, mut rx) = broadcast::channel::<u64>(16);
tx.send(1)?;
rx.recv().await?;
```
