# 212 intermediate: a cxx bridge whose C++ source is never built

**Concept:** `cxx::bridge` only declares the C++ function and generates glue. The C++ file must be compiled into the crate by `build.rs`, or the linker has no symbol to resolve.
**Source:** cxx, "Building the bridge" and `cxx_build::Build::file`, https://cxx.rs/build/cargo.html; `cc` crate docs, https://docs.rs/cc
**Fails with:** `add_i64`. The linker names it as an undefined symbol (`Undefined symbols` on macOS, `undefined reference to` on Linux).
**Vault:** none yet; write one on how a C++ symbol gets from `.cc` into the final binary.

## Task
Make `cargo test` pass by fixing `build.rs`. Do not change `src/lib.rs`, `src/add.h`, `src/add.cc` or the tests. No `#[link]` attribute, no `extern "C"` block.

## Before you fix it
1. Read the linker error. Which symbol is missing, and which file defines it?
2. Open `build.rs`. Which file names does it hand to the build?

## Answer in the PR
1. Why does the bridge compile without `add.cc` but the test link fails?
2. What does `cxx_build::bridge` generate, and what is still your job?
3. Why `include("src")`? Which file does that path find?

## Related concepts
- `cc::Build` alone: compiles C/C++ files into a static library, but knows nothing about the Rust bridge. None: no close siblings for this exact wiring.
- `#[link(name = "...")]`: links an already-built library by name. Wrong here, because nothing is built yet. None: no close siblings.

## In the wild
- Every `cxx` project has a `build.rs` with a list of `.file(...)` calls; a forgotten one is the most common setup bug.
- Exchange and market-data stacks wrap C++ libraries this way, so the `.cc` lists grow with the library.

In a review, check:
- Every `.cc` in `src/` is listed in `build.rs` or intentionally excluded.
- `rerun-if-changed` covers the header and the source, or edits will not rebuild.
