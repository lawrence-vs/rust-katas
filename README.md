# rust-katas

Small Rust bugs to fix, one concept each, from what I'm reading. Every kata **fails on purpose** until it's fixed: it doesn't compile, or its test fails.

Start with the [learning map](MAP.md): every kata in suggested order, with what each one needs first.

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

| Level | What it tests |
|---|---|
| `beginner` | one rule |
| `intermediate` | two rules interacting |
| `advanced` | design: which fix is right and why |
| `expert` | unsafe, Miri, FFI and async internals: code the compiler accepts but that is still wrong |

A new kata's PR must fail on purpose: CI checks that `cargo test` (or `cargo miri test` for katas with `miri = true` under `[package.metadata.kata]`) fails with the README's **Fails with:** text. CI also runs `cargo audit` on every kata's `Cargo.lock`.

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
| websockets | [24 advanced: split reads from writes](katas/websockets/24-advanced-split-reads-from-writes) | tokio-tungstenite docs |
| websockets | [25 advanced: a lagging client](katas/websockets/25-advanced-lagging-client) | tokio `broadcast` docs, lagging |
| grpc | [26 advanced: generate, don't hand-write](katas/grpc/26-advanced-generate-dont-hand-write) | tonic routeguide tutorial |
| grpc | [27 advanced: a deadline on a stream](katas/grpc/27-advanced-deadline-on-a-stream) | tonic routeguide tutorial (server streaming) |
| unsafe | [28 expert: unsafe to dodge the borrow checker](katas/unsafe/28-expert-unsafe-to-dodge-the-checker) | Rust for Rustaceans ch. 1, pointers point to places |
| unsafe | [29 expert: a safe API over raw pointers](katas/unsafe/29-expert-safe-api-over-raw-pointers) | The Rustonomicon, meet safe and unsafe |
| unsafe | [30 expert: a dangling CString across FFI](katas/unsafe/30-expert-dangling-cstring) | The Rustonomicon, FFI |
| capstone | [31 expert: capstone: a concurrent cache](katas/capstone/31-expert-concurrent-cache) | rust-workshop-labs (staged labs with observations) |
| enums | [33 beginner: a status string to an enum](katas/enums/33-beginner-status-string-to-enum) | Programming Rust (Blandy et al.), ch. 10 Enums and Patterns |
| collections | [34 beginner: entry, not contains_key](katas/collections/34-beginner-entry-not-contains) | Programming Rust (Blandy et al.), ch. 16 Collections, "Entries" |
| iterators | [35 beginner: iterate, don't index](katas/iterators/35-beginner-iterate-dont-index) | Programming Rust (Blandy et al.), ch. 15 Iterators |
| money | [42 intermediate: no float money](katas/money/42-intermediate-no-float-money) | The Rust Reference, arithmetic overflow; `u64::checked_add` docs |
