# 169 grpc (intermediate): an old field number reused by a new field

**Concept:** On the wire a protobuf field is its number, not its name. A deleted field's number and name are `reserved` so no one can reuse them, and a new field gets a new number. Readers skip numbers they don't know. In proto3 a plain scalar can't tell 0 from "not sent"; `optional` can.
**Source:** [proto3 language guide](https://protobuf.dev/programming-guides/proto3/) ("Updating A Message Type", "Reserved Fields", "Field Presence"); [prost](https://docs.rs/prost/0.14.4/prost/) for how `proto3 optional` becomes an `Option`.
**Fails with:** `old sender's legacy_email decoded as phone`
**Vault:** your note on schema evolution and wire compatibility

## Task
Make `cargo test` pass by changing only `proto/v2/account.proto` and `src/lib.rs`. Do not touch `proto/v1/` (it is what old clients run), `tests/`, `build.rs` or `Cargo.toml`. Keep `Account`, `encode_v2`, `decode_v2` and `pb`. No `unwrap`, `expect` or `panic!` in `src/`. No new crates. Don't renumber a field v2 already has.

## Before you fix it
1. Run `cargo test`. Which three tests pass, and why does `old_sender_leaves_new_optional_field_absent` pass while the code is still wrong?
2. Write out the bytes `v1::Account { legacy_email: "a".into(), ..Default::default() }` encodes to (tag byte, length, data). What does a v2 reader see?
3. Predict what `decode_v2` returns for a v2 sender that set an overdraft limit of 0.

## Answer in the PR
1. Why did reusing 3 give no error on either side?
2. What does `reserved 3; reserved "legacy_email";` protect against that simply not using 3 doesn't?
3. Why was going from a plain `uint64` to `optional uint64` at number 5 safe on the wire, but moving it to a new number would not be?
4. When is "0 means unset" acceptable, and when is it a bug?
5. Why must an old reader skip unknown fields instead of rejecting them? What breaks during a rolling deploy if it rejects them?

## Think further
1. Changing a field's type (e.g. `int32` → `int64`, `string` → `bytes`, `uint64` → `sint64`): which changes are wire-compatible? Check the "Updating A Message Type" table.
2. `buf breaking` in CI: which rule would have caught this PR? Check the rule name on [buf.build/docs/breaking/rules](https://buf.build/docs/breaking/rules) before writing it down. Then show the command `buf breaking --against '.git#branch=main'` and say what it compares.
3. An enum gains a value. What does an old reader get in proto3 (open enums), and what does prost give you?
4. Avro and JSON Schema do evolution differently (Avro resolves by field name plus a reader/writer schema; JSON by name). What is each one's equivalent of `reserved`?

## In the wild
- [proto3 language guide, "Reserved Fields"](https://protobuf.dev/programming-guides/proto3/#reserved): what to reserve and why.
- [Protobuf dos and don'ts](https://protobuf.dev/best-practices/dos-donts/): "Don't re-use a tag number".
- [Buf breaking-change detection](https://buf.build/docs/breaking/): CI checks that compare a schema against the last release.
- [Field presence](https://protobuf.dev/programming-guides/field_presence/): implicit versus explicit presence, and why `optional` changes the generated code.
- `google.protobuf.wrappers` (`UInt64Value`): the pre-`optional` way to get presence.

In a review, check:
- a deleted field with no `reserved`
- a field number reused or changed
- a plain scalar where 0 is a valid value
- decode code of the form `if x == 0 { None }`

## Related concepts
### `oneof`
At most one of several fields is set, and the reader can see which one. Use it for a tagged union on the wire.
```proto
oneof contact {
  string email = 7;
  string sms = 8;
}
```

### Wrapper types (`google.protobuf.UInt64Value`)
A message around a scalar, so the field can be absent. Use it in APIs and old code written before proto3 `optional`.
```proto
google.protobuf.UInt64Value limit = 9;
```

### `#[serde(default)]` and `skip_serializing_if`
The JSON-side tools for missing fields. Use them when the same domain type also crosses JSON.
```rust
#[serde(default)]
pub note: Option<String>,
```

### Buf `breaking`
A CI check that compares a `.proto` against the last release. Use it on every PR that touches a schema.
```sh
buf breaking --against '.git#branch=main'
```
