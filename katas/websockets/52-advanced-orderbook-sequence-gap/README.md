# 52 advanced: an order book sequence gap

**Concept:** A local book is a snapshot plus every delta after it, in order. A gap means the book is wrong, so drop it and resync from a new snapshot. A delta at or before the book's seq is stale, and a snapshot behind the stream can't be used.
**Source:** std [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html); own design (an exchange market-data feed handler); kata 25 Think further 1; kata 50 (replay is a no-op).
**Fails with:** `a gap triggers a resync`
**Vault:** your note on order-book sync

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect` or `panic!` in `src/`. std only. Don't change `SnapshotSource`, and fetch a snapshot only when the book can't go on.

## Before you fix it
1. Which tests already pass, and why?
2. In `a_gap_triggers_a_resync`, what did delta 12 do, and what does your book show at price 100?

## Answer in the PR
1. Walk through the starting code on the gap test. Which level is a lie, and who would trade on it?
2. Why can't you repair a gap by applying the next delta, or by merging the new snapshot into the old book?
3. Replaying the same delta is harmless here. Why is applying an *older* one not?
4. Why does a snapshot behind the stream leave the book empty instead of serving the snapshot? What does the caller see, and what happens on the next delta?
5. Why `Option<u64>` and not a `synced: bool` beside the seq (kata 11)? What does your next-seq check do at `u64::MAX`?

## Think further
1. Feeds that send `first..=last` update ids per message: what must the first delta after a snapshot straddle?
2. The snapshot is a slow REST call while deltas keep arriving. Buffer them during the fetch: which do you replay, and what bounds the buffer (katas 21, 25)?
3. A gap you can't see, such as a bug in `apply`: periodic checksums of the top N levels.
4. A crossed book (best bid ≥ best ask) as another sign of lost state.
5. A snapshot call that fails or is rate-limited: a fallible source, backoff, and what the book serves meanwhile.
6. One connection, many symbols: per-symbol seq or per-connection seq?

## In the wild
- Exchange WebSocket depth streams whose docs say: fetch a REST snapshot, buffer the stream, drop events at or before the snapshot's update id, and resync if the ids don't chain.
- Feeds that send a checksum with each book update.
- FIX `MsgSeqNum` gaps answered with `ResendRequest` / `SequenceReset`.
- UDP multicast market data: sequence-numbered packets, a retransmit service for small gaps and a snapshot channel for large ones.
- TCP ordering doesn't save you: reconnects and server-side drops (kata 25's `Lagged`) still lose messages.

In a review, check:
- Deltas applied with no seq check.
- A reconnect that keeps the old book.
- A resync that merges instead of replacing.
- A stale book served while resyncing.
- `seq + 1` that can overflow.
