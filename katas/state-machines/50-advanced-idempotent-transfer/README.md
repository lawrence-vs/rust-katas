# 50 advanced: an idempotent transfer

**Concept:** A transfer is a state machine per idempotency key. A timeout is `Unknown`, not failure. A retry or a replayed event returns the current state and never sends twice. Outcomes can't overwrite a final state.
**Source:** std [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry); IETF draft "The Idempotency-Key HTTP Header Field" (draft-ietf-httpapi-idempotency-key-header); own design (exchange withdrawals); kata 33 (enums), kata 12 (why not typestate here).
**Fails with:** `a retry after a timeout does not send twice`
**Vault:** your note on idempotency keys

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect` or `panic!` in `src/`. `Rail::send` is called at most once per key. std only.

## Before you fix it
1. Which tests already pass, and why?
2. After a timeout, did the fake rail move the money?

## Answer in the PR
1. In the starting code, walk through a timeout and then a retry. How many times did money move, and who would notice?
2. Write your transition table. Which states are final, and which events are no-ops in them?
3. Why is a retry in `Unknown` not resent? What would have to be true of the rail for resending to be safe?
4. Why does the same key with a different amount return an error instead of sending, or returning the old state?
5. Why one enum per key instead of a set per state or bool flags? What does `match` do when someone adds a `Reversed` state? Why not typestate (kata 12)?

## Think further
1. Rebuilding `Transfers` from a stored log after a restart: replay must not call `send`. How do you split deciding (the transition) from doing (the side effect)?
2. An `Unknown` that never resolves: reconciling by asking the rail about the key, or a `NeedsReview` state.
3. The key map grows forever: key retention (TTL), and what a retry after eviction does.
4. Two threads applying the same `Requested` at once (kata 51).
5. Single-use quote IDs from kata 47 as the same pattern.

## In the wild
- Payment APIs that take an `Idempotency-Key` header and return the stored response on a repeat.
- Exchange withdrawals (requested → broadcast → confirmed), where a node RPC timeout doesn't mean the transaction wasn't broadcast.
- FIX `ClOrdID` uniqueness and the `PossResend` flag.
- At-least-once message delivery, which needs idempotent consumers.

In a review, check:
- A retry loop around a non-idempotent call.
- A timeout mapped to "failed".
- Deduplication only on success.
- State stored as several bools.
- A final state overwritten by a late event.
