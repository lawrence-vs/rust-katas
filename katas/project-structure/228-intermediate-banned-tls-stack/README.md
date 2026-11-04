# 228 intermediate: a banned TLS stack in the tree

**Concept:** A workspace should link one TLS stack. Kraken's `deny.toml` bans `native-tls` and `openssl-sys` so every crate uses rustls. In CI, `cargo deny check bans` enforces that rule. This kata has no `cargo deny`; a plain test reads `Cargo.toml` as text and stands in for it, so it runs under `cargo test` with no extra tool.

**Source:**
- kraken-cli `deny.toml` (MIT licensed; the rule is reimplemented here, no text copied)
- https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html

**Fails with:** `native-tls is banned, use rustls`

**Vault:** Your note on supply-chain rules: one TLS stack, banned crates, and enforcing policy in CI instead of review.

## Task
Make the manifest use rustls, not native-tls. Do not edit tests. Rustls needs no system library: use the `ring` crypto provider with default features off.

## Related concepts

### `openssl-sys`
Raw bindings to the system OpenSSL, pulled in by many crates. It needs a C library at build time.
```toml
[dependencies]
openssl = "0.10"
```

### `[bans]` in `deny.toml`
A cargo-deny section that lists crates the dependency graph must not contain.
```toml
[bans]
deny = [{ crate = "some-crate" }]
```

### Cargo features
A dependency's optional parts, switched on from the manifest. Often a crate picks its TLS backend this way.
```toml
reqwest = { version = "0.12", default-features = false }
```

## Before you fix it
1. Run `cargo test`. Which test fails, and what does it read?
2. Run `cargo tree -i native-tls`. Which crates does it bring along?

## Answer in the PR
1. Why does a workspace want exactly one TLS stack?
2. What does a test on a text file prove, and what does it not prove?
3. What would `cargo deny` check that this test cannot?

## Think further
1. A transitive dependency pulls `native-tls` in. Where would you see it, and how would you stop it?
2. Which cargo-deny `[bans]` options let one crate be allowed only through a named wrapper?

## In the wild
- cargo-deny bans check: https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html
- rustls docs: https://docs.rs/rustls
