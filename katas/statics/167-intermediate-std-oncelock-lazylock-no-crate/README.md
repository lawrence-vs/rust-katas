# 167 statics (intermediate): OnceLock and LazyLock, no crate

**Concept:** A value the whole process shares is built once, the first time it's needed. std's `LazyLock` does that when building can't fail, and `OnceLock` holds a value set once (by the first set or the first read). Neither needs a `static mut`, a flag, an `unsafe` block, or the `once_cell` crate.
**Source:** std docs for [`OnceLock`](https://doc.rust-lang.org/1.93.0/std/sync/struct.OnceLock.html) (stable 1.70) and [`LazyLock`](https://doc.rust-lang.org/1.93.0/std/sync/struct.LazyLock.html) (stable 1.80); kata 41.
**Fails with:** `the keyword table is built once per process`
**Vault:** your note on lazy statics and one-time init

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Keep the public API (`Keyword`, `ConfigError`, `DEFAULT_WORKERS`, `table_builds`, `keyword`, `set_workers`, `workers`). No new crates, no `unsafe` blocks, no `static mut`, no `thread_local!`, no `Mutex`, and no `unwrap`/`expect`/`panic!` in `src/`.

## Before you fix it
1. Run `cargo test`. Which test fails every time, and what is its `left:` value? Work it out from 8 × 1,000.
2. Run it 20 times. Does the racing-set test always pass? Why can't a passing run prove the race isn't there?
3. Predict which std type replaces each of the two globals.

## Answer in the PR
1. Why rebuilding on every call is a cost and not a bug in the results, and what the counter proves.
2. Name the two steps the flag version can interleave, and show one interleaving where two callers get `Ok`.
3. Which type you used where, and why `LazyLock` fits the table but not the worker count.
4. What happens to a waiting thread while another thread runs the init.

## Think further
1. `OnceLock::get_or_try_init` is still unstable on 1.93 (feature `once_cell_try`, issue #109737). How would you cache a fallible init (a config file that may not parse) without it, and when is the `once_cell` crate (kata 98, #198) still worth the dependency?
2. Put the table in `thread_local!`. What does `table_builds()` show, and why?
3. If `build_table` panicked, what would later lookups do (look up "poisoned" in the `LazyLock` docs)?
4. Add `#![forbid(unsafe_code)]`. Which line of the buggy code would it have stopped?
5. `LazyLock::get` (peeking without forcing) is only stable from 1.94. Why might you want it?

## In the wild
- A compiled pattern or lookup table in a parser, built once.
- A global `tracing` subscriber or metrics registry, set once at startup.
- A worker-pool size read from the environment once.

In a review, check:
- `static mut`, or a `bool` "initialised" flag next to a value
- a table or regex built inside a hot function
- `lazy_static!` or `once_cell` in a crate whose MSRV is 1.80 or later

## Related concepts
### `std::sync::Once`
Runs a closure once and stores nothing. Use it for one-time side effects such as FFI library init.
```rust
static INIT: Once = Once::new();
INIT.call_once(|| init_logging());
```

### `thread_local!`
One value per thread, built on that thread's first access. Use it for per-thread scratch buffers.
```rust
thread_local! { static BUF: RefCell<Vec<u8>> = RefCell::new(Vec::new()); }
```

### `const` vs `static`
A `const` is copied inline at every use, while a `static` has one address for the whole program and must be `Sync`. Use `const` for plain values.
```rust
const MAX: usize = 64;
static NAME: &str = "pool";
```

### `once_cell::sync::Lazy`
The crate type std's `LazyLock` came from; it still offers `OnceCell::get_or_try_init` on stable. This is a snippet only, not a dependency.
```rust
static RE: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| "x".repeat(3));
```
