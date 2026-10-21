# 30 expert: a dangling CString across FFI

**Concept:** `CString::new(s)?.as_ptr()` points into a temporary that is dropped at the end of the statement. C then reads freed memory. Bind the `CString` to a variable that outlives the FFI call.
**Source:** The Rustonomicon, FFI, https://doc.rust-lang.org/nomicon/ffi.html; `CString::as_ptr` docs, https://doc.rust-lang.org/std/ffi/struct.CString.html
**Fails with:** `Undefined Behavior`
**Vault:** your note on ownership across FFI

## Task
Make `cargo +nightly miri test` pass without changing the test or the `extern` block, with no compiler warnings.

This is an `expert` kata: the bug compiles and may even pass `cargo test`. Run `cargo +nightly miri test`; CI does too.

## Before you fix it
1. Read the compiler warning. Which value does it say is dropped, and when?
2. Run `cargo +nightly miri test`. Which line reads freed memory?

## Answer in the PR
1. Why does C need the `CString` alive, and who owns it during the call?
2. Why did plain `cargo test` likely pass?
3. What does `CString::new` reject, and why does C care?

## Think further
1. Call `c_len("héllo")`. Does `strlen` count characters or bytes? Which does a C caller usually want?
2. The C function keeps the pointer after it returns (it registers a name or a callback). Is keeping the `CString` alive "across the call" still enough? Who must own it, and until when?
3. The other direction: C returns a `*const c_char` that it owns. What must you know before you turn it into a `&str`, and how many ways can that fail?
4. The string comes from a client (an order's client ID) and contains a NUL. Without the check in `CString::new`, what would C have seen?

## In the wild
- rustc's [`dangling_pointers_from_temporaries`](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#dangling-pointers-from-temporaries) lint, the warning you read: it detects getting a pointer to data of a temporary that will immediately get dropped.
- [`CString::into_raw`](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_raw) is for when C really must own the string. The pointer must be returned to Rust and reconstituted with `CString::from_raw`, or it leaks.
- [`CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html) and `c"..."` literals: a borrowed C string, for constants and for strings C hands back.
- rusqlite's [`SmallCString`](https://github.com/rusqlite/rusqlite/blob/master/src/util/small_cstr.rs): every SQL string and name passed to SQLite goes through a Rust-owned, NUL-checked buffer.
- Exchange-style: FIX engines, market-data feed handlers and kernel-bypass network libraries are often C or C++ called over FFI on the hot path, where one dangling string pointer is a crash or a corrupted message.

In a review, check:
- `.as_ptr()` (or `.as_mut_ptr()`) called on an expression that isn't bound to a local.
- `#[allow(dangling_pointers_from_temporaries)]`, or any allowed warning near FFI.
- `CString::new(user_input).unwrap()`: interior NUL from a client becomes a panic.
- `into_raw` without a matching `from_raw` on every path, or `CStr::from_ptr` on a pointer whose lifetime nobody wrote down.
