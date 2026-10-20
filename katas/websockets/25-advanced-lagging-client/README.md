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
