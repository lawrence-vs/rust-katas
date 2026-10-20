# 24 advanced: split reads from writes

**Concept:** A WebSocket is two independent streams. One loop that reads and then writes can only deliver when the client speaks. `StreamExt::split` gives a sink and a stream that can run in separate tasks.
**Source:** tokio-tungstenite docs, https://docs.rs/tokio-tungstenite/0.30.0/tokio_tungstenite/; Tokio tutorial, channels, https://tokio.rs/tokio/tutorial/channels
**Fails with:** `silent_client_still_receives`
**Vault:** your note on full-duplex connections

## Task
Make `cargo test` pass without changing the test or `serve`'s signature.

## Before you fix it
1. The listening client never sends. Where is its `handle` stuck?
2. What does `ws.split()` return, and which half needs `SinkExt`?

## Answer in the PR
1. Why did one loop starve the silent client?
2. How does your `handle` end when either side closes, and what stops the other task leaking?
3. `tokio::select!` in one task or two spawned tasks: what did you pick and why?
