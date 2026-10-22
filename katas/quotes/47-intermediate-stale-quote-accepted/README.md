# 47 intermediate: a stale quote accepted

**Concept:** A quote is only firm until its deadline and only while the market stays near the mid it was priced off. It carries both, and `accept` rejects it when it has expired (checked first) or when the market has drifted past the tolerance. Deadline and drift arithmetic can't overflow.
**Source:** std [`u64::checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add) and [`u64::abs_diff`](https://doc.rust-lang.org/std/primitive.u64.html#method.abs_diff); own design (exchange RFQ quoting); kata 15 for the injected clock.
**Fails with:** `a quote accepted after its deadline is rejected`
**Vault:** your note on quotes as a firm price with a deadline

## Task
Make `cargo test` pass without changing the tests. No `unwrap`, `expect`, `panic!`, `as`, `saturating_*` or `wrapping_*`. No `SystemTime` or `Instant` in `src/`: the time comes in as `now`.

## Before you fix it
1. Run the tests. Which ones already pass, and why?
2. How many cents is 10 bps of $65,000.00?

## Answer in the PR
1. In the starting code, what did a client gain by waiting a minute and then accepting, and who paid for it?
2. Why is a quote at exactly its deadline rejected? Did you store the deadline or the issue time, and why?
3. Why check expiry before drift?
4. How did you keep the deadline and the drift product from overflowing? What would `wrapping_add` have done to the deadline?
5. Why can't a caller build a `Quote` with a later deadline?

## Think further
1. Asymmetric last look: reject only moves against the house. Is that fair?
2. A quote accepted twice: single-use quote IDs (see kata 50).
3. `now` earlier than the issue time, because a clock stepped backwards or a client sent the timestamp.
4. A tolerance in ticks instead of bps.

## In the wild
- std [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) (monotonic) vs [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) (wall clock, can step back).
- RFQ desks hand out a quote ID with a TTL of seconds.
- The FX "last look" window.
- Good-till-date orders on exchanges.

In a review, check:
- An accept path that never reads the time.
- A deadline built with `+`.
- A client-supplied timestamp trusted for expiry.
