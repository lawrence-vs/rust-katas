# 12 advanced: typestate: consume self

**Concept:** If a method takes `self` by value and returns a different type, the old state can't be used again. Encode a connection's state in its type and calling `query` on a closed connection stops compiling.
**Source:** The Typestate Pattern in Rust (Cliff L. Biffle), https://cliffle.com/blog/rust-typestate/; API Guidelines checklist https://rust-lang.github.io/api-guidelines/checklist.html
**Fails with:** `E0432`
**Vault:** your note on typestate

## Task
Make `cargo test` pass without changing the test or the `compile_fail` doc test on `Connection` (it is part of the contract: `cargo test` runs it).

## Before you fix it
1. Why does the `compile_fail` doc test compile today?
2. If `connect` takes `self` and returns `Connection<Open>`, what happens to the old binding?

## Answer in the PR
1. Where did the runtime check go, and what does the compiler now prove?
2. Why `PhantomData<State>` and zero-sized state types, and what do they cost at runtime?
3. When is typestate the wrong tool? (think: state that comes from the network)
