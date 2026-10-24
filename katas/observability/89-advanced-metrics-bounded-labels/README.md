# 89 advanced: bounded metric labels, durations in seconds

**Concept:** A label is a dimension. Every distinct value is another time series to store, so labels come from small enums (outcome, kind), never from ids, and durations are recorded in seconds.
**Source:** metrics and metrics-util docs on docs.rs (`with_local_recorder`, `DebuggingRecorder`); Prometheus, "Metric and label naming" (https://prometheus.io/docs/practices/naming/)
**Fails with:** `one series per account`
**Vault:** your note on metric cardinality

**Why this crate:** `metrics` is a facade like `log`: the library records and the binary picks the exporter. The `prometheus` crate ties library code to one registry and one wire format.

## Task
Make `cargo test` pass without changing `tests/`, the public signatures or `Cargo.toml`. No `unwrap`, `expect` or `panic!` in `src/`, and no new dependencies.

## Before you fix it
1. Run `cargo test`. Which of the five tests fail, and what does each message tell you?
2. Read the `/// CLUE:` style comments in `src/lib.rs`. Predict the fix in one sentence.

## Answer in the PR
1. How many series does the starting code make for a million accounts? Who pays for them?
2. Why `&'static str` from an enum and not `String`?
3. What does `as_millis() as f64` lose for a 300 µs request?
4. Why do the tests use a local recorder?

## Think further
1. Where does per-account detail go instead: logs, trace spans, exemplars?
2. Histogram buckets vs summaries: what can you still aggregate across instances?
3. An error-kind label built from an error enum (kata 09).
4. A cardinality budget in review: who owns the number?

## In the wild
- Prometheus conventions: counters end in `_total`, base units in the name (`_seconds`, `_bytes`).
- OpenTelemetry's `http.server.request.duration` is in seconds.
- Time-series databases have fallen over from a user-id or URL-path label.

In a review, check:
- `format!` or `to_string()` in a label value.
- `as_millis` going into a histogram.
- A metric name with no unit suffix.
