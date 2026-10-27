# 193 events (advanced): a k-way merge with a watermark

**Concept:** Merge streams that are each in order by holding events in a min-heap and emitting only up to the watermark: the slowest stream's latest timestamp minus the allowed lateness. Ties break by id, a quiet stream sends idle markers, and anything behind the emitted position goes to a counted late output.
**Source:** Akidau et al., "The Dataflow Model" (VLDB 2015); Apache Flink docs, "Timely Stream Processing" and "Generating Watermarks" (WatermarkStrategy::withIdleness); std BinaryHeap and std::cmp::Reverse docs; katas 52, 62, 73. No Readwise highlight was available to the builder.
**Fails with:** `merged feed is out of (timestamp, id) order`
**Vault:** your note on watermarks and event-time ordering

`Merger` takes `Input`s from N streams, and each stream is in `(ts, id)` order. The tests are plain `#[test]` functions, with no threads and no clock.

## Why
A dashboard or ledger that reads the merged feed must see one order every run. Wrong cross-stream order and arrival-order ties produce different pages on replay. A silent stream with no idle handling stalls output forever.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`. No new crates. Each emitted event costs O(log k), not a scan or sort of the whole buffer.

## Before you fix it
1. Run `cargo test`. Which tests fail, and which three behaviours do they point at?
2. In `streams_merge_into_one_timestamp_ordered_feed`, write the watermark after each push by hand.

## Answer in the PR
1. Why can't the head of one stream be emitted before every stream has reported something?
2. Why does a heap give O(log k) per event, and what would a `Vec` re-sorted per push cost?
3. What does an idle marker promise, and what breaks if a stream sends one and then an event below it?
4. Why is a late event sent aside and counted, not inserted into the feed? Who should alert on the count?
5. Sizing: for 10,000 events/s, 250 ms lateness and 64-byte events, how big is the buffer? What does raising lateness buy, and what does it cost?

## Think further
1. Flink `WatermarkStrategy::forBoundedOutOfOrderness(...).withIdleness(...)`: who sends the idle signal there, the source or a timeout?
2. Allowed lateness vs a side output for late data in Flink/Beam: re-fire a window, or drop and count?
3. Per-partition watermarks in Kafka consumers: a partition with no traffic is a silent stream.
4. Memory bound: what should happen when `buffered()` exceeds the `buffer_bound` events? Backpressure, spill, or advance the watermark?
5. Duplicates: an equal `(ts, id)` is not late here. Where would you dedupe?

## In the wild
- Akidau et al., "The Dataflow Model" (VLDB 2015)
- Apache Flink, "Timely Stream Processing" and "Generating Watermarks"
- Apache Beam programming guide, "Watermarks and late data"
- Kafka Streams `grace` period

In a review, check:
- an emit-on-arrival merge
- ties broken by arrival
- a min over streams with no idle handling
- late data reordered into an already-published feed
- an unbounded buffer

## Related concepts

### BTreeSet as an ordered buffer
An ordered set that iterates from the smallest element. Use it for an ordered buffer that also needs range queries.
```rust
let mut s = std::collections::BTreeSet::new();
s.insert((10, 1));
let first = s.pop_first();
```

### itertools::kmerge
Merges finite, already-sorted iterators lazily. Use it for merging sorted files or batches, not live streams.
```rust
use itertools::Itertools;
let all: Vec<i32> = vec![vec![1, 4], vec![2, 3]].into_iter().kmerge().collect();
```

### Sorting a closed batch
A stable sort of a finished batch by key. Use it when all input is known up front.
```rust
let mut batch = vec![(10, 7), (10, 3)];
batch.sort_by_key(|&(ts, id)| (ts, id));
```

### VecDeque per source
A FIFO queue per source. Use it to hold each source's backlog in arrival order.
```rust
let mut q = std::collections::VecDeque::new();
q.push_back(5);
let head = q.front();
```
