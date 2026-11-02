# 232 intermediate: JSON that comes back in a different order

**Concept:** `serde_json::Value::Object` wraps a `Map`. By default that map is a `BTreeMap`, so keys come out sorted, not in the order they arrived. The `preserve_order` feature swaps it for an `IndexMap`, which keeps insertion order. An API that echoes a client's JSON back must keep the order, or a signature over the bytes, or a textual diff, breaks.
**Source:** https://docs.rs/serde_json/latest/serde_json/#features ; https://docs.rs/serde_json/latest/serde_json/map/index.html ; Kraken's CLI sets `preserve_order` workspace-wide (kraken-cli, MIT; rule reimplemented here, not copied)
**Fails with:** `key order changed`
**Vault:** your note on JSON as an ordered format (the spec calls objects unordered, the bytes you sign are not)

## Task
Make `echo` return the keys in the order they arrived. The fix is in `Cargo.toml` (enable the right `serde_json` feature). Do not change the tests or the function body.

## Related concepts

### `serde_json::value::RawValue`
Holds a JSON fragment as unparsed text, so it round-trips byte for byte. Use it to pass a payload through untouched.
```rust
#[derive(serde::Deserialize, serde::Serialize)]
struct Env { body: Box<serde_json::value::RawValue> }
```

### `BTreeMap<String, V>`
A sorted map. Iteration order is key order. Use it when you want deterministic output regardless of input.
```rust
let mut m = std::collections::BTreeMap::new();
m.insert("b", 1); m.insert("a", 2);
assert_eq!(m.keys().next(), Some(&"a"));
```

### `Vec<(String, V)>`
Pairs in a vector keep arrival order and allow duplicate keys. Use it when order matters and lookups are rare.
```rust
let pairs: Vec<(String, i32)> = vec![("b".into(), 1), ("a".into(), 2)];
```

## Before you fix it
1. Run the test. In what order did the keys come back, and what rule produces that order?
2. Which crate feature, not code, decides the container behind `Value::Object`?

## Answer in the PR
1. What is a `BTreeMap`'s iteration order?
2. What does `preserve_order` change in the type behind `Map`?
3. When is `RawValue` better than a `Value`?

## Think further
1. Feature flags are additive across a workspace: what happens if another crate in the dependency graph enables `preserve_order`, and your crate relied on sorted output?
2. Is preserving order enough to make a signature stable, or do you need a canonical form? Compare kata 196.

## In the wild
- https://docs.rs/serde_json/latest/serde_json/#features
- https://docs.rs/serde_json/latest/serde_json/value/struct.RawValue.html
