# kraken-cli: technical review prep

Source: `/Users/lawrence/Local/Workspace/Programming/kraken-cli` (shallow clone, read-only study). Refs are `path:line` in that clone. Items marked INFERRED are not stated in the repo. Unverified = read from code, not compiled or run.

## Part A: questions on kraken-ws (connection actor)

| # | Question | Answer | Ref |
|---|---|---|---|
| 1 | Why `biased;` first and shutdown first in `run_event_loop`? | Default `select!` picks a random start. `biased` polls in order, so shutdown wins even when the reader is always ready. | `connection.rs:526-533` |
| 2 | Why is the idle timer created before the loop, polled as `&mut idle`, not `timeout(.., reader.recv())`? | A timer per iteration restarts whenever another branch (ping) fires, so it never trips. `&mut` keeps one timer. `tokio::pin!` is needed because `Sleep` is `!Unpin`. | `connection.rs:516-519`; `transport.rs:4-6` |
| 3 | What resets the idle deadline, and why not an unparseable frame? | Only a parsed frame. Junk is not liveness, so an all-junk feed still reconnects. | `connection.rs:538-544` |
| 4 | What does `run_until_cancelled` return? | `Option<T>`: `Some(output)` if the future finished first, `None` if cancelled. `None` means stop. | `connection.rs:139-144, 361-366` |
| 5 | Backoff formula, and when does it reset? | `delay = min(base,max) + rand[0,jitter_max]`, `base <- min(2·base,max)`. Reset to `initial` after a session >= `stable_after`, not after a setup failure. | `backoff.rs:91-104, 82-86`; `connection.rs:385-392` |
| 6 | Why zero jitter and a 5-attempt cap for one-shot dial, but streams retry forever? | One process, one host: no thundering herd. The cap surfaces the last concrete error. Streams retry until the operator interrupts. | `connection.rs:789-806`; `backoff.rs:3-4` |
| 7 | Why must `Transport` be `Send + Sync + 'static`? | The `Connection` holding it moves into `tokio::spawn` (needs `Send + 'static`). `&self` methods awaited in that future need `T: Sync`. | `transport.rs:69, 76-77`; `stream_client.rs:136` |
| 8 | Why does `TokenSource::fetch` return `BoxFuture` with `+ Send`, not `async fn`? | Object safety for `Arc<dyn TokenSource>`. `dyn` hides auto traits, so `Send` must be explicit. | `token.rs:27-29, 65-74` |
| 9 | What does `pub(super)` on `Connection`'s fields allow here? | Parent is the crate root, so any kraken-ws module can read the fields and build the struct literal. Nothing outside the crate can. | `connection.rs:73-88`; `connection_tests.rs:108-119` |
| 10 | Why can `one_shot_tests` call `connection.dial()` but `connection_tests.rs` could not? | `one_shot_tests` is a child module (sees private items). `connection_tests` is a sibling (sees only `pub(super)`/`pub(crate)`). `dial` is private. | `connection.rs:870, 951-1007`; `lib.rs:17-19` |
| 11 | Actor stop if the caller drops `EventStream` but keeps `StreamClient`? | Next `events_tx.send` fails. `emit` sees shutdown not yet cancelled and cancels it itself. | `connection.rs:100-117`; test `connection_tests.rs:887-903` |
| 12 | Why is the actor-to-client channel unbounded, and what bounds memory downstream? | A burst never back-pressures the socket read loop. Downstream `kraken-app` uses a 1024-slot broadcast; slow consumers get `Lagged(n)`. | `stream_client.rs:119-121`; `kraken-app/src/stream.rs:22, 382`; `monitor.rs:79-86` |
| 13 | Why does one-shot `run` close the socket only after `pump` ends? | A Close before the reply lets the server drop the reply (RFC 6455: no data after Close echo). | `connection.rs:857-862`; test `connection.rs:1057-1086` |
| 14 | A socket error after the request was sent: what error, and why? | `Error::ReplyLost(Box<cause>)`. Verify-first: the order may have reached the exchange, so no blind retry. | `connection.rs:822-828, 933-939`; `kraken-core/src/error.rs:127-132` |
| 15 | Why is `Attempt` an enum, not two bools? | Only 3 of 4 combinations are legal: a replay always re-mints. | `connection.rs:291-317` |
| 16 | How does a test make a 90 s idle timeout pass instantly? | `#[tokio::test(start_paused = true)]` + `tokio::time::advance(READ_IDLE_TIMEOUT + 1s)`. Works because prod code uses `tokio::time` timers. | `connection_tests.rs:954-966` |
| 17 | How does `TokenCache` guarantee one fetch for 8 concurrent cold `get()`s? | The `tokio::sync::Mutex` guard is held across the fetch `.await`; waiters find the cache warm. Tested on a 2-worker multi-thread runtime. | `token.rs:108-116, 214-228` |

## Part B: questions on standards, lints, crates, tests (new)

| # | Question | Answer | Ref |
|---|---|---|---|
| 18 | Why deny `unwrap`/`expect`/`panic` in prod but allow them in tests? | `clippy.toml` sets `allow-*-in-tests`. A prod `expect` needs a local `#[expect]` whose reason states the invariant. | root `Cargo.toml` `[workspace.lints.clippy]`; `clippy.toml:1-3`; `ws/lib.rs:13` |
| 19 | What does `unreachable_pub = "warn"` force, and how does it relate to Q9? | Items not meant as API must be `pub(crate)` or `pub(super)`. Keeps the public surface small, so privacy is enforced by the compiler, not convention. | root `Cargo.toml` `[workspace.lints.rust]` |
| 20 | Why hand-written `Debug` on token and request types, and why not `missing_debug_implementations` workspace-wide? | Redacts the token so it can't reach logs via `{:?}`. The root Cargo.toml comment explains why the lint is not enabled workspace-wide. | `ws/token.rs:39-48`; `ws/request_client.rs:34-43`; `ws/lib.rs:6` |
| 21 | Why does `deny.toml` ban `native-tls` and `openssl-sys`? | One TLS stack (rustls only). Fewer system deps and a single audited crypto path. | `deny.toml` |
| 22 | Why does `wait_until` panic after 500 tries instead of returning? | An exhausted wait fails loud at the right place, not as a confusing downstream assert. | `mock_transport.rs:16-26` |
| 23 | Why does `advance_until` yield 20 times, then advance 100 ms, up to 200 rounds? | Under paused time the actor task must run between clock steps; yielding lets it reach its next `.await`. | `connection_tests.rs:144-158` |
| 24 | Why does `MockTransport` derive `Clone` with `Arc` fields? | The test keeps one clone to inspect sent frames while the actor owns the other. | `mock_transport.rs:30-35` |
| 25 | Why expose `UnboundedReceiverStream` as `EventStream`, not the channel? | Callers get a `Stream`; the channel stays an implementation detail. | `stream_client.rs:70` |
| 26 | Why `tokio-util` `CancellationToken` and `DropGuard`? | INFERRED: dropping the stream handle cancels the actor automatically, with no explicit stop call. | `connection.rs:100-117` |
| 27 | Why an MSRV gate, and why `#[allow(deprecated)]` in the mock? | CI checks the declared `rust-version` (`just msrv`). `try_update` needs 1.99+, so the mock allows the deprecation. | root `Cargo.toml`; `justfile`; `ws/mock_transport.rs:83-84` |
| 28 | Why a doctest tripwire script? | nextest skips doctests, so a stray doctest would silently never run. The script fails the build if one appears. | `justfile:128-167` |
| 29 | Why `preserve_order` and `raw_value` on serde workspace-wide? | Read the comment in root `Cargo.toml` before answering. Not verified in this review. | root `Cargo.toml` |
| 30 | Why `getrandom` for PKCE and OAuth state, not the `rand` crate? | Already in the lockfile via `uuid` and `tempfile`. Reuse over a new dependency. | root `Cargo.toml` comment |
| 31 | Why `schemars` on command types? | A flag rename cannot silently reshape the MCP tool surface; the schema changes and a snapshot catches it. | root `Cargo.toml` comment |
| 32 | Why `Box<Error>` inside `ReplyLost`? | A recursive enum needs indirection to have finite size. INFERRED as the reason; standard Rust rule. | `error.rs:46` |
| 33 | Why `rmcp` elicitation? | Lets the dangerous-command gate reach the human operator for confirmation. | root `Cargo.toml` comment |
| 34 | Why is `anyhow` used only at the binary edge? | Libraries return typed errors so callers can match and decide on retry. Only `kraken-app` needs a catch-all. | `kraken-app/src/errors.rs:239`; `kraken-app/Cargo.toml:24` |

## Part C: techniques not yet in rust-katas (learn list)

Checked by grep of `katas/` and `REVIEW.md`. "None" = no kata or rule mentions it. "Partial" = mentioned, not as the kraken pattern.

| Technique | kraken ref | rust-katas status | Suggested kata |
|---|---|---|---|
| `CancellationToken` + `DropGuard` | `connection.rs:100-117` | partial (4 files mention token) | shutdown by drop, not by call |
| `unreachable_pub` + `[workspace.lints]` | root `Cargo.toml` | partial (3 files) | lint config as code review item |
| `expect` with `#[expect(reason)]` | root `Cargo.toml` | partial (1 file) | "a documented expect, not an unwrap" |
| Redacted `Debug` for secrets | `ws/token.rs:39-48` | covered by kata 92 | none |
| `advance_until` under paused time | `connection_tests.rs:144-158` | partial (`start_paused` in 2 files) | yield-then-advance test helper |
| `wait_until` with a loud panic | `mock_transport.rs:16-26` | none | a poll helper that fails with the last state |
| Test double with `Arc` fields + `Clone` | `mock_transport.rs:30-35` | none | fake that the test keeps a handle to |
| `UnboundedReceiverStream` hides the channel | `stream_client.rs:70` | none | return `impl Stream`, not the receiver |
| `cargo deny` ban list | `deny.toml` | none | ban a crate, prove the build fails |
| MSRV + `rust-version` | root `Cargo.toml`; `justfile` | partial (1 file) | "your code needs a newer std than declared" |
| proptest for property tests | `backoff.rs:212`; many crates | covered by kata 214 | more: retain-is-narrowing, round-trip |
| `insta` snapshot tests | `kraken-workspace/tests/snapshots.rs` | partial (snapshot mentioned in 12 files) | review snapshot diffs |
| `schemars` schema as contract | root `Cargo.toml` | none | schema snapshot catches a renamed field |
| serde `preserve_order` / `raw_value` | root `Cargo.toml` | none | key order matters on the wire |

## Part D: open items

- Q3 and Q10 answers read from code only. Confirm by running the tests or reading the file.
- Q29 answer not verified: read the root `Cargo.toml` comment.
- No `watch` channel exists in the workspace; `JoinSet` is used only in `kraken-auth`. Do not claim otherwise in the review.
- Kata candidates in Part C are proposals. None are built.
