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

## Think further
1. A client pulls its network cable: no Close frame, no FIN. The room is quiet. How long until your `handle` notices, and which half notices first?
2. A client sends a Ping. tungstenite queues the Pong for you, but it only goes out when that connection is next written to or flushed. With the halves in different tasks, when does that happen on a quiet room?
3. The talker sends "hi". Does it get its own message back? Should it, and whose job is it to decide?
4. 10,000 clients: two tasks per client or one `select!` per client. What does each cost, and is `StreamExt::next` safe to drop half-way through a frame?

## In the wild
- [`WebSocketStream`](https://docs.rs/tokio-tungstenite/0.30.0/tokio_tungstenite/struct.WebSocketStream.html): it is both a `Stream` and a `Sink`, which is why it can be split.
- [`StreamExt::split`](https://docs.rs/futures-util/0.3.34/futures_util/stream/trait.StreamExt.html#method.split): the two halves share a lock; `reunite` puts them back.
- [`tokio::select!`, cancellation safety](https://docs.rs/tokio/1.53.2/tokio/macro.select.html#cancellation-safety): what is lost when a branch's future is dropped.
- [RFC 6455 §5.5 control frames](https://datatracker.ietf.org/doc/html/rfc6455#section-5.5) and [§7 closing the connection](https://datatracker.ietf.org/doc/html/rfc6455#section-7): Ping/Pong and the close handshake that both halves must respect.
- Exchange-style: a market-data socket where the client mostly listens. an exchange has the client [ping]( and the server send a [heartbeat]( so both sides can tell a quiet feed from a dead one.

In a review, check:
- A loop that awaits `ws.next()` and only then writes: a listen-only client never gets anything.
- Two spawned halves where only one ends: who aborts the other, and does the `broadcast::Receiver` get dropped?
- `select!` over futures that aren't cancellation-safe, recreated on every loop pass.
- No ping or idle timeout on a long-lived socket.
