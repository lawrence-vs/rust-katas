# 231 intermediate: a serde rename changes the wire contract

**Concept:** `#[serde(rename = "...")]` changes the JSON key, whatever the Rust field is called. Deriving `JsonSchema` exports the same key into the schema, so the schema is a reviewable contract: a rename shows up as a diff instead of a silent client break.
**Source:** serde field attributes, `rename`, https://serde.rs/field-attrs.html#rename; schemars, https://docs.rs/schemars/latest/schemars/
**Fails with:** `wire key changed`
**Vault:** your note on wire formats as contracts (the serialized shape is the API, not the struct)

## Task
Make `cargo test` pass without editing the tests. Restore the `volume` wire key and keep `pub volume` in the struct's public signature.

## Related concepts
### `#[serde(alias = "...")]`
Accepts extra names when deserializing only. Use it to read an old key during a migration.
```rust
#[serde(alias = "quantity")]
pub volume: u32,
```

### `#[serde(rename_all = "...")]`
Renames every field by a case rule. Use it to match a camelCase API.
```rust
#[serde(rename_all = "camelCase")]
pub struct Quote { pub best_bid: u32 }
```

### `#[serde(skip_serializing)]`
Drops a field from the output. Use it for fields that must not leave the process.
```rust
#[serde(skip_serializing)]
pub token: String,
```

## Before you fix it
1. Run the test. Which key does the JSON hold, and which attribute put it there?
2. Does the struct's field name or the attribute win when the two disagree?

## Answer in the PR
1. What does the rename change for a client that sends or reads `volume`?
2. Why does a Rust field name not decide the wire?
3. What would a schema diff show in review for this change?

## Think further
1. Clients still send `qty` in stored messages. How would you accept both keys while emitting one?
2. How would you fail CI when the generated schema drifts from a committed snapshot?

## In the wild
- https://docs.rs/schemars/latest/schemars/
- https://serde.rs/field-attrs.html#rename
