# 25 advanced: a lagging client

**Concept:** A `broadcast` channel drops the oldest messages for a receiver that falls behind and reports `Lagged(n)`. That is a policy choice: skip ahead and count, or disconnect. Treating it as `Closed` silently drops the client.
**Source:** tokio `broadcast` docs, lagging, https://docs.rs/tokio/1.53.2/tokio/sync/broadcast/index.html
**Fails with:** `lagging_client_keeps_receiving`
**Vault:** your note on slow consumers

## Task
Make `cargo test` pass without changing the test. Skip ahead and count what was missed.

## Before you fix it
1. What does `recv` return to a receiver that fell 3 messages behind a channel of capacity 2?
2. Which `RecvError` variant really means stop?

## Answer in the PR
1. Why did the old loop disconnect a slow client without telling anyone?
2. Skip ahead or disconnect: which policy fits a chat room, and which a trade feed?
3. Why is this the per-client write task from kata 24, and what does it protect the other clients from?

## Think further
1. The feed is order-book deltas, not chat. The client skipped 3 deltas. Is its book still right, and what would the client or server have to do next?
2. Raise the capacity from 2 to 10,000 and `Lagged` mostly goes away. What does that cost, and who pays for it: the slow client or every client?
3. `outbox` is bounded at 16 and the client stops reading. Where does `fan_out` wait, and what happens on the `broadcast` side while it waits? You now have two buffers. Which policy does each one have?
4. Should the client be told it missed `n` messages? What would that look like on the wire?

## In the wild
- [tokio `broadcast`, lagging](https://docs.rs/tokio/1.53.2/tokio/sync/broadcast/index.html#lagging) and [`RecvError`](https://docs.rs/tokio/1.53.2/tokio/sync/broadcast/error/enum.RecvError.html): `Lagged` doesn't close the receiver.
- [axum's chat example](https://github.com/tokio-rs/axum/blob/main/examples/chat/src/main.rs): a widely copied `while let Ok(msg) = rx.recv().await` loop. Look at what it does with a lagging client.
- [`BroadcastStream`](https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.BroadcastStream.html): the same choice once a receiver is wrapped as a `Stream`; lag comes through as an error item.
- [tokio `watch`](https://docs.rs/tokio/1.53.2/tokio/sync/watch/index.html): for when only the latest value matters (a ticker), so there is no backlog to fall behind on.
- [tungstenite `WebSocketConfig::max_write_buffer_size`](https://docs.rs/tungstenite/0.30.0/tungstenite/protocol/struct.WebSocketConfig.html): the default is unbounded. A slow socket is a third buffer.
- Exchange-style: [Kraken's v2 book channel](https://docs.kraken.com/exchange/api-reference/spot-websocket-v2/book) sends a snapshot, then updates with a checksum, so a client that missed updates can tell its book is wrong.

In a review, check:
- `while let Ok(..) = rx.recv().await` on a `broadcast::Receiver`: `Lagged` silently ends the loop.
- No metric or log for how many messages a client missed.
- A skip-ahead policy on a feed where every message matters (deltas, fills, balances).
- Channel capacities and socket write buffers with no stated bound or reason.
