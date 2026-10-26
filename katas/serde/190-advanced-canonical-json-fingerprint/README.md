# 190 serde (advanced): an idempotency fingerprint over canonical JSON

**Concept:** An idempotency fingerprint must hash what the request means, not its bytes: one canonical form (keys sorted by UTF-8 bytes, one escape form, no whitespace, numbers exactly as written), duplicate keys and deep nesting refused while parsing, and method, path and body framed so no two requests share bytes.
**Source:** RFC 8259 §4 (names "SHOULD be unique") and §6 (binary64 is exact only up to 2^53); RFC 8785 (JCS), which sorts by UTF-16 code units, unlike this kata; serde_json feature docs (`arbitrary_precision`); IETF Idempotency-Key header draft (draft-ietf-httpapi-idempotency-key-header); kata 50
**Fails with:** `a 19-digit integer id was not preserved exactly`
**Vault:** your note on canonical forms

A retry with the same key must produce the same fingerprint, and a different request must produce a different one. A fingerprint that changes with key order rejects honest retries. One that loses digits, keeps the last of two duplicate keys, or runs fields together matches a different request and replays the old result. Complexity: O(n log n) time (the key sort), O(depth) stack, depth ≤ 32.

## Task
Make `cargo test` pass by changing only `src/lib.rs`.

- Keep `MAX_DEPTH`, `FingerprintError` with its three variants and their `Display`, `canonical_json` and `fingerprint`.
- Don't change `tests/` or `Cargo.toml`, so serde_json keeps its default features. No new crates.
- No `unwrap`, `expect`, `panic!` or `#[allow(...)]` in `src/`.

The rules the tests check:

- Canonical form: no whitespace outside strings. An object's members are sorted by the UTF-8 bytes of the decoded key, written as `"key":value` and joined by `,` inside `{}`. An array keeps its input order. A string is decoded, then written the way `serde_json::to_string` writes it. A number is written as its input text, if that text matches `-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?`. `true`, `false` and `null` are written as they are.
- Errors: a body that is not UTF-8, has bad grammar, trailing data, a bad escape, a lone surrogate or a raw control character in a string is `Invalid`. A second identical decoded key in one object is `DuplicateKey(key)`. A container opened at depth 33 is `TooDeep`.
- Framing: `SHA-256 over (u64 big-endian byte length, bytes) of the method, the path and the canonical body, in that order.`

## Before you fix it
1. Run `cargo test`. Which six fail, and which six already pass?
2. Print `canonical_json(br#"{"id":-9999999999999999999}"#)`. Where did the digits go?
3. What is the largest integer an `f64` holds exactly, and why that one?

## Answer in the PR
1. What did the starting code do to `-9999999999999999999`, and why did `9999999999999999999` survive?
2. Why is a duplicate key an error and not "last wins"? Describe two parsers that read `{"amount":1,"amount":1000}` differently, and what an idempotency check then lets through.
3. Why must the depth check run while parsing, not on the parsed value? What does the 100_000 `[` test prove?
4. Why UTF-8 byte order and not UTF-16 code units? Name two keys whose order flips.
5. Why does the fingerprint refuse NFC folding and number re-encoding? What could a client do with one idempotency key if it didn't?
6. Why does a length prefix separate fields where a separator character alone doesn't?

## Think further
1. RFC 8785 (JCS) re-serialises numbers as ECMAScript doubles and sorts by UTF-16 units. Which tests here would a JCS library fail?
2. serde_json's `arbitrary_precision` keeps the digits. What does a custom `Visitor` see for a big number, and what does the parser write for `1E2`? Check in the serde_json source.
3. A JavaScript client reads `9007199254740993`. What does it get? Should the API send ids as strings?
4. The depth limit bounds the stack, not memory. Where does a body size limit go (katas 21, 23)?
5. Should the fingerprint also cover the query string, the content type or the tenant (kata 179)?

## In the wild
- The IETF Idempotency-Key draft: the server may fingerprint the request, and a reused key with a different payload gets an error, not the stored response.
- Payment APIs that compare a retry's parameters with the original and reject a mismatch.
- Matrix canonical JSON for signed events: no whitespace, keys in code-point order, integers only.

In a review, check:
- a hash over raw request bytes
- `serde_json::Value` used to canonicalise numbers
- no duplicate-key policy
- method and path concatenated without framing

## Related concepts

### `arbitrary_precision`
A serde_json feature that stores each number as its digit string. Use it when numbers must round-trip exactly. This kata's Task rules it out.
```toml
serde_json = { version = "1", features = ["arbitrary_precision"] }
```

### `serde_json::value::RawValue` (feature `raw_value`)
Borrows a value's exact text without parsing it. Use it to pass a sub-document through untouched.
```rust
let raw: Box<serde_json::value::RawValue> = serde_json::from_str(r#"{"b":1, "a":2}"#)?;
assert_eq!(raw.get(), r#"{"b":1, "a":2}"#);
```

### `serde::de::DeserializeSeed`
A deserialize that carries state, such as a depth counter. Use it to pass context down while a type is being deserialized.
```rust
struct Depth(usize);
impl<'de> serde::de::DeserializeSeed<'de> for Depth {
    type Value = ();
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<(), D::Error> { serde::de::IgnoredAny::deserialize(d).map(|_| ()) }
}
```

### Unicode NFC (`unicode-normalization` crate)
Folds composed and decomposed forms into one. Use it for search and display, not for identity hashes.
```rust
use unicode_normalization::UnicodeNormalization;
let s: String = "e\u{301}".nfc().collect();
assert_eq!(s, "\u{e9}");
```
