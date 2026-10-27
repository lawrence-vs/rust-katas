# 206 intermediate: `default` hides a missing field

**Concept:** `#[serde(default)]` makes an absent key fall back to the type's default. For untrusted input that turns "the upstream forgot the quantity" into a zero-quantity fill, which looks valid and is a silent bug. Use `default` only where absence is a real, documented case.
**Source:** serde field attributes, `default`, https://serde.rs/field-attrs.html#default; API Guidelines C-VALIDATE
**Fails with:** `rejects_missing_qty`
**Vault:** your note on strict parsing of untrusted input

## Task
Make `cargo test` pass without changing the test. Keep `id` and `qty` typed as they are. A fill with no `qty` must be an error.

## Before you fix it
1. Run the test. What value did the parser give `qty` when the key was absent?
2. Which of the two fields is allowed to be absent, and why does the answer differ?

## Answer in the PR
1. What is the difference between a missing field and a field set to zero? Which one can a downstream consumer tell apart?
2. A feed adds an optional `fee` that is often absent. How would you model it so absence is explicit in the type?

## Related concepts
- **`Option<T>` field:** absence is a value the type names. The caller must handle `None`.
- **`deny_unknown_fields`:** rejects keys the struct does not name. It does not stop a missing key; that is a separate rule.

```rust
#[serde(default)]
pub fee: Option<u64>,
```
