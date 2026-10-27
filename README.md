# rust-katas

Small Rust bugs to fix, one concept each, from what I'm reading. Every kata **fails on purpose** until it's fixed: it doesn't compile, or its test fails.

Start with the [learning map](MAP.md): every kata in suggested order, with what each one needs first.

## How this repo is built

The katas are written by AI, the solutions by me. My agent pipeline (Claude: a planning model writes the plan, a coding model builds the kata, a separate model audits it against [REVIEW.md](REVIEW.md), and CI proves the kata fails for the reason it teaches) opens one PR per kata; I review and merge. Each kata cites the book or doc it comes from, so I can check it against the source.

Solving is mine: a `solve/<kata>` branch, no AI autocomplete, and the README's questions answered in my own words in the PR. Those PRs are the record of what I've learned.

## How a kata works

```
katas/<topic>/<NN>-<level>-<slug>/
  README.md      concept, source, the error it fails with, task, questions to answer
  src/lib.rs     the bug, with `/// CLUE:` hints (hover with `K` in Neovim)
  tests/kata.rs  passes only when the bug is fixed
```

1. Read the kata's README and run `cargo watch -q -c -x check -x test` inside it. Read the compiler's error before touching code. Only rust-analyzer, the docs and the clues help; no AI autocomplete.
2. Fix it on a branch, answer the README's questions in the PR description in your own words.
3. Open a PR. CI runs `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` on only the katas the PR changes; Junior then reviews it against [REVIEW.md](REVIEW.md) (idioms, ownership, [API Guidelines](https://rust-lang.github.io/api-guidelines/)).
4. Write what you learned as a permanent note in the vault and link the kata from its `code:` property.

### Start a kata (from nothing to coding)

A kata only exists on `main` once its Junior PR is merged; `git pull` brings it in.

```sh
cd ~/Local/Workspace/Programming/rust-katas
git switch main && git pull --ff-only
git switch -c solve/NN-slug                  # e.g. solve/07-unwrap-in-library
cd katas/<topic>/NN-level-slug               # e.g. katas/errors/07-beginner-unwrap-in-library
cargo watch -q -c -x check -x test           # read README.md and the first error, then code
```

Fix, then check and open the PR:

```sh
cargo fmt && cargo clippy -- -D warnings && cargo test
git add -A . && git commit -m "Solve kata NN: <what you fixed>"
git push -u origin solve/NN-slug
gh pr create --fill                          # README's questions answered in the body
```

Shortcut: `source kata.sh` once (add it to your shell rc), then `kata 07` switches to `solve/07-<slug>` (creating it if needed), `cd`s into the kata and starts `cargo watch`.

Pace (80/20):
- 80% solo: read the README, fix it, green tests, short PR answers. No AI. About 15 minutes each.
- 20% with Claude: only when the tests pass but you can't say why, or two fixes both look right (usually `advanced`). Paste your fix and ask "why not X?"; Claude reviews, doesn't write.
- Skip the vault note unless the kata surprised you.
- Batch the `beginner` and `intermediate` katas; save `advanced` and `expert` for dialog.

| Level | What it tests |
|---|---|
| `beginner` | one rule |
| `intermediate` | two rules interacting |
| `advanced` | design: which fix is right and why |
| `expert` | unsafe, Miri, FFI and async internals: code the compiler accepts but that is still wrong |

A new kata's PR must fail on purpose: CI checks that `cargo test` (or `cargo miri test` for katas with `miri = true` under `[package.metadata.kata]`) fails with the README's **Fails with:** text. CI also runs `cargo audit` on every kata's `Cargo.lock`, and runs gitleaks over the PR's commits: katas may contain bad code but never anything secret-shaped, so a kata that needs a token-like string uses an obviously fake one, and its PR explains the narrow allowlist entry.

Katas are made with the `code-kata` skill from a concept, a Readwise highlight or a Claude session; `katas/_template/` is the shape.

## Tracks

Kata links go live as each kata merges.

| Topic | Kata | Source |
|---|---|---|
| ownership | [01 beginner: mutable variable vs mutable reference](katas/ownership/01-beginner-mutable-variable-vs-reference) | Rust for Rustaceans, ch. 1 |
| ownership | [02 intermediate: moving out of a Vec](katas/ownership/02-intermediate-move-out-of-vec) | Rust for Rustaceans, ch. 1 |
| lifetimes | [03 advanced: a struct outliving its reference](katas/lifetimes/03-advanced-struct-outlives-reference) | Rust for Rustaceans, ch. 1 |
| ownership | [04 intermediate: borrow, don't take](katas/ownership/04-intermediate-borrow-dont-take) | Network Programming With Rust (Chanda), introduction to the borrow checker |
| ownership | [05 intermediate: take, don't clone](katas/ownership/05-intermediate-take-dont-clone) | Network Programming With Rust (Chanda), mutable borrows |
| lifetimes | [06 advanced: one lifetime too many](katas/lifetimes/06-advanced-one-lifetime-too-many) | Network Programming With Rust (Chanda), lifetime parameters on structs |
| errors | [07 beginner: unwrap in a library](katas/errors/07-beginner-unwrap-in-library) | Network Programming With Rust (Chanda), methods that panic: expect() and unwrap() |
| errors | [08 intermediate: `?` needs a From](katas/errors/08-intermediate-question-mark-needs-from) | Network Programming With Rust (Chanda), enums carrying data and error handling |
| errors | [09 advanced: `Option` loses the why](katas/errors/09-advanced-option-loses-why) | Network Programming With Rust (Chanda), implementing the Error trait for an enum |
| api-design | [10 beginner: newtypes, not primitives](katas/api-design/10-beginner-newtypes) | Rust API Guidelines, C-NEWTYPE |
| api-design | [11 intermediate: bool flags to an enum](katas/api-design/11-intermediate-bool-flags-to-enum) | Rust API Guidelines, C-CUSTOM-TYPE (arguments convey meaning through types, not bool) |
| api-design | [12 advanced: typestate: consume self](katas/api-design/12-advanced-typestate) | The Typestate Pattern in Rust (Cliff L. Biffle) |
| traits | [13 beginner: a missing trait bound](katas/traits/13-beginner-missing-trait-bound) | All Rust Features Explained, generics bound by traits |
| traits | [14 intermediate: `impl Trait` vs `dyn Trait`](katas/traits/14-intermediate-impl-trait-vs-dyn) | All Rust Features Explained, polymorphism |
| traits | [15 advanced: inject, don't reach for a global](katas/traits/15-advanced-inject-dont-reach-out) | All Rust Features Explained, the trait system vs classical inheritance |
| async | [16 beginner: a std MutexGuard across `.await`](katas/async/16-beginner-std-guard-across-await) | Tokio tutorial, shared state (holding a MutexGuard across an .await) |
| async | [17 intermediate: spawn, then send results back](katas/async/17-intermediate-spawn-and-send-results-back) | Tokio tutorial, channels |
| async | [18 advanced: a lock held over a slow await](katas/async/18-advanced-lock-over-slow-await) | Tokio tutorial, shared state (tasks, threads and contention) |
| db | [19 intermediate: one pool, made at startup](katas/db/19-intermediate-one-pool-at-startup) | sqlx `SqlitePool` docs |
| db | [20 advanced: a sync driver on the async runtime](katas/db/20-advanced-sync-driver-spawn-blocking) | tokio `spawn_blocking` docs |
| security | [21 intermediate: a bounded queue for untrusted input](katas/security/21-intermediate-bounded-queue) | tokio `mpsc::channel` docs |
| security | [22 intermediate: a timeout on every network await](katas/security/22-intermediate-timeout-every-network-await) | tokio `timeout` docs |
| security | [23 advanced: an untrusted length prefix](katas/security/23-advanced-untrusted-length-prefix) | Network Programming With Rust (Chanda), parsing in networking applications |
| security | [92 beginner: a secret that never reaches Debug](katas/security/92-beginner-secrecy-no-leak) | secrecy docs |
| websockets | [24 advanced: split reads from writes](katas/websockets/24-advanced-split-reads-from-writes) | tokio-tungstenite docs |
| websockets | [25 advanced: a lagging client](katas/websockets/25-advanced-lagging-client) | tokio `broadcast` docs, lagging |
| grpc | [26 advanced: generate, don't hand-write](katas/grpc/26-advanced-generate-dont-hand-write) | tonic routeguide tutorial |
| grpc | [27 advanced: a deadline on a stream](katas/grpc/27-advanced-deadline-on-a-stream) | tonic routeguide tutorial (server streaming) |
| unsafe | [28 expert: unsafe to dodge the borrow checker](katas/unsafe/28-expert-unsafe-to-dodge-the-checker) | Rust for Rustaceans ch. 1, pointers point to places |
| unsafe | [29 expert: a safe API over raw pointers](katas/unsafe/29-expert-safe-api-over-raw-pointers) | The Rustonomicon, meet safe and unsafe |
| unsafe | [30 expert: a dangling CString across FFI](katas/unsafe/30-expert-dangling-cstring) | The Rustonomicon, FFI |
| capstone | [31 expert: capstone: a concurrent cache](katas/capstone/31-expert-concurrent-cache) | rust-workshop-labs (staged labs with observations) |
| strings | [32 beginner: `&str`, not `String`](katas/strings/32-beginner-str-not-string) | Programming Rust (Blandy et al.), ch. 17 Strings and Text |
| enums | [33 beginner: a status string to an enum](katas/enums/33-beginner-status-string-to-enum) | Programming Rust (Blandy et al.), ch. 10 Enums and Patterns |
| collections | [34 beginner: entry, not contains_key](katas/collections/34-beginner-entry-not-contains) | Programming Rust (Blandy et al.), ch. 16 Collections, "Entries" |
| iterators | [35 beginner: iterate, don't index](katas/iterators/35-beginner-iterate-dont-index) | Programming Rust (Blandy et al.), ch. 15 Iterators |
| testing | [36 beginner: test the edges](katas/testing/36-beginner-test-the-edges) | Rust for Rustaceans (Gjengset), ch. 6 Testing |
| closures | [37 intermediate: a closure bound stricter than the body needs](katas/closures/37-intermediate-fnonce-too-strict) | Programming Rust (Blandy et al.), ch. 14 Closures |
| smart-pointers | [38 intermediate: a RefCell borrow held across recursion](katas/smart-pointers/38-intermediate-rc-refcell-borrow-panic) | Programming Rust (Blandy et al.), ch. 4 "Rc and Arc: Shared Ownership" and ch. 9 "Interior Mutability" |
| drop | [39 intermediate: an RAII guard, not a manual release](katas/drop/39-intermediate-raii-guard) | Programming Rust (Blandy et al.), ch. 13 Utility Traits, "Drop" |
| generics | [40 intermediate: an associated type, not a generic parameter](katas/generics/40-intermediate-associated-type) | Rust for Rustaceans (Gjengset), ch. 2 Types, "Generic Traits" |
| threads | [41 intermediate: Rc across threads](katas/threads/41-intermediate-rc-across-threads) | Programming Rust (Blandy et al.), ch. 19 Concurrency; std `thread::scope` docs |
| money | [42 intermediate: no float money](katas/money/42-intermediate-no-float-money) | The Rust Reference, arithmetic overflow; `u64::checked_add` docs |
| money | [46 intermediate: quote with a spread](katas/money/46-intermediate-quote-with-spread) | std `u128::div_ceil` and `TryFrom` docs; own design (exchange quoting) |
| quotes | [47 intermediate: a stale quote accepted](katas/quotes/47-intermediate-stale-quote-accepted) | std `u64::checked_add` and `abs_diff` docs; own design (exchange RFQ quoting) |
| routing | [48 advanced: a multi-hop route](katas/routing/48-advanced-multi-hop-route) | std u128::div_ceil and TryFrom docs; own design (exchange smart order routing) |
| batching | [49 advanced: a batch with a remainder](katas/batching/49-advanced-batch-with-remainder) | std VecDeque and BTreeMap docs; own design (exchange liquidity-provider hedging) |
| state-machines | [50 advanced: an idempotent transfer](katas/state-machines/50-advanced-idempotent-transfer) | IETF Idempotency-Key header draft; std `HashMap::entry` docs; own design (exchange withdrawals) |
| credit | [51 intermediate: a credit line overspent](katas/credit/51-intermediate-credit-line-overspend) | Rust Atomics and Locks (Mara Bos), ch. 2 Compare-and-Exchange; loom docs |
| websockets | [52 advanced: an order book sequence gap](katas/websockets/52-advanced-orderbook-sequence-gap) | std `BTreeMap` docs; own design (exchange market-data feed handler) |
| db | [61 advanced: a transactional outbox](katas/db/61-advanced-transactional-outbox) | sqlx `Transaction` docs; microservices.io transactional outbox and idempotent consumer; own design (exchange order events) |
| testing | [53 intermediate: a balance that never goes negative](katas/testing/53-intermediate-balance-never-negative) | proptest book; std `TryFrom` docs; own design (exchange account balances) |
| async | [68 intermediate: accidentally not Send](katas/async/68-intermediate-accidentally-not-send) | Rust for Rustaceans (Gjengset), ch. 2 Types and ch. 8 Asynchronous Programming |
| api-design | [64 advanced: a sealed trait and a non_exhaustive enum](katas/api-design/64-advanced-sealed-and-non-exhaustive) | Rust for Rustaceans (Gjengset), ch. 3 Designing Interfaces; Rust API Guidelines C-SEALED |
| traits | [63 advanced: a generic method behind `dyn`](katas/traits/63-advanced-object-safety) | Rust for Rustaceans (Gjengset), ch. 3 Designing Interfaces, "Object Safety" |
| latency | [55 advanced: a config snapshot, not a read per quote](katas/latency/55-advanced-config-snapshot) | Rust Atomics and Locks (Mara Bos), ch. 1 Basics of Rust Concurrency; std `Arc` and `RwLock` docs; own design (exchange quoting) |
| capstone | [60 expert: capstone: an async feed with parallel depth](katas/capstone/60-expert-async-feed-parallel-depth) | tokio `spawn_blocking`, mpsc and graceful shutdown docs; serde container attributes; rayon docs; own design (exchange market-data take-home) |
| state-machines | [62 advanced: out-of-order and duplicate events](katas/state-machines/62-advanced-out-of-order-events) | proptest book (prop_shuffle); thiserror docs; own design (exchange order-update consumer) |
| errors | [123 beginner: unwrap_or runs its fallback even when the value is there](katas/errors/123-beginner-unwrap-or-eager-fallback) | std `Option::unwrap_or_else` and `Option::ok_or_else` docs; Mastering Rust, combinators on `Option` and `Result` |
| collections | [74 intermediate: sparse ids, a map not a Vec](katas/collections/74-intermediate-sparse-ids-memory) | std collections docs, "When Should You Use Which Collection?"; std HashMap::with_capacity docs |
| collections | [191 intermediate: Dijkstra needs a min-heap](katas/collections/191-intermediate-dijkstra-max-heap) | std `BinaryHeap` and `Reverse` docs; own design (shortest paths); kata 104 |
| collections | [203 intermediate: a recursive DFS overflows the stack](katas/collections/203-intermediate-recursive-dfs-overflows-the-stack) | std `thread` docs, "Stack size"; kata 35 |
| project-structure | [104 intermediate: overflow checks differ by profile](katas/project-structure/104-intermediate-overflow-checks-differ-by-profile) | Rust for Rustaceans (Gjengset), ch. 1, "Cargo profiles" |
| concurrency | [73 advanced: shard by key, keep per-key order](katas/concurrency/73-advanced-shard-by-key-keep-order) | tokio `mpsc::channel` docs (bounded channels); std `sync_channel` and `thread::scope` docs; Rust Atomics and Locks (Mara Bos), ch. 1 |
| concurrency | [176 intermediate: a hash ring where adding a node moves few keys](katas/concurrency/176-intermediate-consistent-hash-ring-moves-few-keys) | Karger et al., "Consistent Hashing and Random Trees" (STOC 1997); DeCandia et al., "Dynamo" (SOSP 2007), section 4.2; kata 73 |
| events | [96 advanced: a poison message, dead-lettered](katas/events/96-advanced-poison-message-dead-letter) | Apache Kafka consumer docs (offsets, error handling); rdkafka docs |
| events | [170 advanced: a rebalance that applies a payment twice](katas/events/170-advanced-kafka-rebalance-double-processing) | Apache Kafka docs (delivery semantics, consumer groups); rdkafka docs; kata 96 |
| events | [193 advanced: a k-way merge with a watermark](katas/events/193-advanced-k-way-merge-with-watermark) | "The Dataflow Model" (VLDB 2015); Apache Flink watermark docs; std BinaryHeap docs; katas 52, 62, 73 |
| money | [70 intermediate: parse a decimal string](katas/money/70-intermediate-parse-decimal-string) | The Rust Book ch. 9; std str::parse / FromStr docs; own design (exchange price feeds) |
| money | [87 intermediate: rust_decimal rounding](katas/money/87-intermediate-rust-decimal-rounding) | rust_decimal docs (RoundingStrategy, checked_mul, FromStr); own design (fee and settlement rounding) |
| io | [71 intermediate: Read and Write, not a path](katas/io/71-intermediate-generic-read-write-for-tests) | Rust API Guidelines, C-RW-VALUE; std::io docs |
| errors | [72 intermediate: a bad-row policy](katas/errors/72-intermediate-bad-row-policy) | The Rust Book ch. 9; csv `Error` docs; own design (trade file import) |
| errors | [88 intermediate: thiserror in the library, anyhow in the binary](katas/errors/88-intermediate-thiserror-lib-anyhow-bin) | thiserror and anyhow docs |
| errors | [105 intermediate: an error source chain](katas/errors/105-intermediate-error-source-chain) | Rust for Rustaceans (Gjengset), ch. 4 Error Handling; std Error::source docs |
| errors | [124 beginner: `let _ =` swallows the error](katas/errors/124-beginner-let-underscore-swallows-err) | The Rust Book ch. 9; std `Read::read_to_string` docs |
| async | [58 advanced: graceful shutdown](katas/async/58-advanced-graceful-shutdown) | Tokio topics, graceful shutdown; tokio `JoinSet` and `watch` docs |
| por | [54 advanced: a Merkle sum tree](katas/por/54-advanced-merkle-sum-tree) | RustCrypto sha2 docs; std u64::checked_add docs; own design (exchange proof of liabilities) |
| rayon | [56 intermediate: rayon inside async](katas/rayon/56-intermediate-rayon-inside-async) | tokio docs, CPU-bound tasks; rayon `spawn` docs; Alice Ryhl, "Async: What is blocking?" |
| rayon | [59 intermediate: a shared Mutex inside par_iter](katas/rayon/59-intermediate-shared-mutex-in-par-iter) | rayon docs (ParallelIterator, collect, reduce); own design (exchange mark-to-market) |
| serde | [57 intermediate: untrusted JSON](katas/serde/57-intermediate-untrusted-json) | serde container attributes (deny_unknown_fields, try_from); API Guidelines C-VALIDATE; own design (exchange order requests) |
| atomics | [43 advanced: Relaxed is not enough](katas/atomics/43-advanced-relaxed-is-not-enough) | Rust Atomics and Locks (Mara Bos), ch. 3 Memory Ordering |
| macros | [44 advanced: `macro_rules!`, not copy-paste](katas/macros/44-advanced-macro-rules-not-copy-paste) | Rust for Rustaceans (Gjengset), ch. 7 Macros, "Declarative Macros" |
| capstone | [45 expert: capstone: a mini order book](katas/capstone/45-expert-mini-order-book) | own design (exchange-style); std `BTreeMap` docs |
| threads | [67 advanced: two locks taken in opposite orders](katas/threads/67-advanced-lock-ordering-deadlock) | Rust for Rustaceans (Gjengset), ch. 10 Concurrency (and Parallelism); std `Mutex` docs |
| resilience | [90 advanced: a timeout and a bounded retry, not a retry-forever loop](katas/resilience/90-advanced-tower-timeout-retry) | tower docs (ServiceBuilder, timeout, retry::Policy) |
| resilience | [97 advanced: a circuit breaker, not a retry storm](katas/resilience/97-advanced-circuit-breaker) | Release It! (Nygard), ch. 5 Stability Patterns, "Circuit Breaker"; kata 15 |
| resilience | [177 advanced: a load balancer that ejects, probes and recovers](katas/resilience/177-advanced-lb-health-check-ejection-and-recovery) | Envoy docs (outlier detection, panic threshold); Release It! (Nygard), ch. 5; katas 90, 97 |
| cli | [91 beginner: clap derive, not hand-parsed args](katas/cli/91-beginner-clap-derive) | clap derive tutorial; own design (report CLI) |
| db | [95 advanced: optimistic lock, no lost update](katas/db/95-advanced-optimistic-lock-lost-update) | sqlx docs; Designing Data-Intensive Applications ch. 7 |
| testing | [161 intermediate: a wallet invariant that table tests miss](katas/testing/161-intermediate-proptest-wallet-invariant) | proptest book (strategies, shrinking, failure persistence); docs.rs proptest `ProptestConfig` and `RngSeed` |
| testing | [162 advanced: a mutant your tests let live](katas/testing/162-advanced-cargo-mutants-surviving-mutant) | mutants.rs (cargo-mutants); own design (a transfer fee threshold) |
| testing | [163 advanced: a parser that never panics on garbage](katas/testing/163-advanced-fuzz-parser-never-panics) | rust-fuzz book (cargo-fuzz); own design (a record parser) |
| testing | [205 intermediate: knapsack loop direction](katas/testing/205-intermediate-knapsack-loop-direction) | CLRS ch. 15 (dynamic programming); Kleinberg and Tardos, *Algorithm Design*, ch. 6; own design (a cargo-loading problem); kata 36 |
| rayon | [164 intermediate: a parallel reduce that disagrees with the loop](katas/rayon/164-intermediate-parallel-reduce-order-differs-from-sequential) | rayon docs (ParallelIterator fold, reduce, try_reduce; ThreadPoolBuilder::num_threads); katas 42, 59 |
| money | [168 intermediate: rust_decimal scale and checked ops](katas/money/168-intermediate-rust-decimal-scale-and-checked-ops) | rust_decimal docs (scale, normalize, rescale, Display precision, checked_add, checked_mul, serde::str); katas 70, 87 |
| cache | [171 intermediate: a lock released by its owner only](katas/cache/171-intermediate-redis-lock-release-wrong-owner) | Redis docs, "Distributed Locks with Redis" (SET NX PX, DELEX IFEQ, release script); Kleppmann, "How to do distributed locking" (2016); antirez's reply (2016); katas 39, 51 |
| cache | [172 intermediate: a rate limiter whose counter loses its TTL](katas/cache/172-intermediate-redis-rate-limit-incr-expire-race) | Redis docs (INCR rate-limiter pattern, PEXPIRE); Cloudflare sliding-window counter post (2017); kata 171 |
| cache | [173 advanced: a slow reader writes a stale row back after the delete](katas/cache/173-advanced-cache-aside-stale-write-after-delete) | Facebook, "Scaling Memcache at Facebook" (NSDI 2013, leases); Redis docs (SET PX, EVAL); katas 95, 172 |
| project-structure | [180 beginner: logic in a lib, a thin main](katas/project-structure/180-beginner-lib-plus-thin-bin-logic-in-main) | The Rust Book ch. 7 and ch. 11.3; Cargo Book, Cargo Targets |
| async | [178 intermediate: readiness flips before the SIGTERM drain](katas/async/178-intermediate-readiness-flips-before-sigterm-drain) | Kubernetes docs (Pod termination; liveness, readiness and startup probes; container lifecycle hooks); tokio `watch` and `time` docs; kata 58 |
| db | [174 advanced: an idempotency key written apart from the payment it guards](katas/db/174-advanced-atomic-idempotency-key-with-business-write) | sqlx Transaction docs; SQLite PRIMARY KEY and RETURNING docs; Stripe idempotent requests; Brandur, idempotency keys in Postgres; katas 61, 95 |
| state-machines | [175 advanced: replaying the log gives a different state](katas/state-machines/175-advanced-event-replay-is-deterministic) | Fowler, "Event Sourcing" (2005); std RandomState and SystemTime docs; katas 50, 62 |
| project-structure | [181 intermediate: a mod tree where everything is pub](katas/project-structure/181-intermediate-mod-tree-and-pub-crate-privacy) | The Rust Book ch. 7 "Packages, Crates, and Modules"; Rust Reference "Visibility and privacy"; rustc `unreachable_pub` lint |
| project-structure | [182 intermediate: a pub use facade hides the module layout](katas/project-structure/182-intermediate-pub-use-facade-hides-module-layout) | The Rust Book ch. 7 (`use`, `pub use`) and ch. 14 "Exporting a Convenient Public API"; rustdoc book, "Re-exports"; kata 64 |
| project-structure | [183 intermediate: Cargo features are additive](katas/project-structure/183-intermediate-cargo-features-are-additive) | The Cargo Book, "Features" (feature unification, optional dependencies, dep:); kata 104 |
| project-structure | [184 advanced: another crate's type in your public API](katas/project-structure/184-advanced-external-crate-type-leaks-into-public-api) | Rust API Guidelines C-STABLE; dtolnay semver-trick; kata 87 |
| lifetimes | [186 intermediate: two references in, one out](katas/lifetimes/186-intermediate-two-refs-one-out) | The Rust Reference, lifetime elision; kata 6 |
| csv | [187 intermediate: a comma inside quotes](katas/csv/187-intermediate-quoted-comma) | RFC 4180 (CSV quoting); kata 72 |
| matching | [188 intermediate: a buy takes the cheapest ask first](katas/matching/188-intermediate-buy-takes-cheapest-ask) | std `BTreeMap` docs; own design (exchange price-time priority); kata 45 |
| async | [189 intermediate: two awaits run one after the other](katas/async/189-intermediate-sequential-awaits-not-parallel) | tokio `join!` docs; Tokio tutorial, spawning; kata 17 |
| serde | [196 advanced: an idempotency fingerprint over canonical JSON](katas/serde/196-advanced-canonical-json-fingerprint) | RFC 8259 §4 and §6; RFC 8785 (JCS); serde_json feature docs (arbitrary_precision); IETF Idempotency-Key header draft; kata 50 |
| state-machines | [194 intermediate: a hold not released on cancel](katas/state-machines/194-intermediate-order-hold-not-released-on-cancel) | own design (exchange reserve-then-settle); std u64 checked_* docs; katas 42, 51, 53 |
| state-machines | [201 advanced: an amend keeps its place in the queue](katas/state-machines/201-advanced-amend-keeps-queue-priority) | std `BTreeMap` and `VecDeque` docs; own design (exchange price-time priority); katas 45, 12 |
| websockets | [202 advanced: conflation drops trades](katas/websockets/202-advanced-conflation-drops-trades) | own design (exchange market-data fan-out); std `VecDeque` docs; katas 25, 52 |
