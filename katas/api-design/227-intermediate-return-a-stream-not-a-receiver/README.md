# 227 intermediate: a public stream, not a public receiver

**Concept:** `UnboundedReceiver` has an inherent `recv()` but is not a `futures::Stream`, so `StreamExt` does not apply to it. A public API should promise `impl Stream<Item = T>`, not the channel type, so the channel stays private and can change.
**Source:** [`UnboundedReceiverStream`](https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.UnboundedReceiverStream.html) and [`impl Trait` in return position](https://doc.rust-lang.org/std/keyword.impl.html)
**Fails with:** `but its trait bounds were not satisfied`
**Vault:** your note on what a public signature promises

## Task
Make `cargo test` pass without changing the test. Return an `impl Stream<Item = u32>` built from the channel, keep the three values and their order, and keep the channel out of the public signature.

## Related concepts
### `ReceiverStream`
The same adapter for the bounded `mpsc::channel`. Use it when senders need backpressure.
```rust
let (tx, rx) = tokio::sync::mpsc::channel::<u8>(8);
let s = tokio_stream::wrappers::ReceiverStream::new(rx);
```

### `tokio_stream::iter`
Turns any `IntoIterator` into a stream. Use it in tests or when the items already exist in memory.
```rust
let s = tokio_stream::iter(vec![1, 2, 3]);
```

### `Box<dyn Stream>`
A boxed, type-erased stream. Use it when branches return different stream types.
```rust
use std::pin::Pin;
type Boxed = Pin<Box<dyn tokio_stream::Stream<Item = u8> + Send>>;
```

## Before you fix it
1. Read the error. Which trait is `collect` looking for, and what does `UnboundedReceiver` offer instead?
2. Which tokio-stream type wraps a receiver, and what does it implement?

## Answer in the PR
1. Why does `StreamExt` not apply to the receiver?
2. What does `impl Stream` hide that the receiver exposes?
3. What would changing the channel type cost a caller of the receiver?

## Think further
1. The sender is dropped before return. What would the stream do if it were kept alive and never sent again?
2. Should the item type be `u32` or a `Result`? What would a caller learn from the end of the stream?

## In the wild
- [`UnboundedReceiverStream`](https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.UnboundedReceiverStream.html): read what it forwards. Is `size_hint` kept?
- Kraken's Rust client (MIT) exposes an `UnboundedReceiverStream` as its public `EventStream` (`stream_client.rs:70`). What does a caller lose by seeing that concrete type instead of `impl Stream`?
