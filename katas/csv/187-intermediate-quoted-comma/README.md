# 187 intermediate: a comma inside quotes

**Concept:** A CSV field is delimited by commas only outside quotes. Splitting on every comma is not a CSV parser: a quoted field can hold the delimiter, and the quotes must come off only at the field's edges.
**Source:** RFC 4180, Common Format and MIME Type for CSV Files, sections 2.5 and 2.6, https://www.rfc-editor.org/rfc/rfc4180
**Fails with:** `keeps_quoted_comma`
**Vault:** your note on CSV fields and delimiters

## Task
Make `cargo test` pass without changing the test. Use only the standard library, no `csv` crate.

## Before you fix it
1. Run the test. Which field did the function return in place of `Smith, J`, and where did the cut happen?
2. Why does `trim_matches('"')` look like it handles quotes but still fail?

## Answer in the PR
1. What state does your parser need to know: inside quotes or outside? Where does that state change?
2. What would the `csv` crate cost you here, and when would you pick it over a hand-written splitter?

## Related concepts
- **`str::split`:** cuts at every match, with no notion of quoting. Right for `key=value` lines, wrong for CSV.
- **`csv` crate:** a full RFC-style reader with headers, escaped quotes and `serde`. Used in kata 72.

```rust
let fields: Vec<&str> = "a,b".split(',').collect();
```
