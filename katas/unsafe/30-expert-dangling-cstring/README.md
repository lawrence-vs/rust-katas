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
