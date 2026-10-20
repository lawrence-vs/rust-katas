# 26 advanced: generate, don't hand-write

**Concept:** Protobuf messages and gRPC stubs are generated from the `.proto` at build time (`build.rs` + `tonic::include_proto!`), so they can't drift from it. Hand-written or checked-in generated code drifts silently.
**Source:** tonic routeguide tutorial, https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md; protox docs, https://docs.rs/protox/0.9.1/protox/
**Fails with:** `E0432`
**Vault:** your note on code generation from schemas

## Task
Make `cargo test` pass without changing the test, `proto/` or `Cargo.toml`. Add a `build.rs` (the build-dependencies are already there; `protox` compiles the `.proto` in pure Rust, no `protoc` needed). Don't fix the tag by hand.

## Before you fix it
1. Compare the `name` tag in `src/lib.rs` with `proto/hello.proto`.
2. The test imports `pb::greeter_client` and `pb::greeter_server`. Who writes those?

## Answer in the PR
1. What went wrong on the wire with tag 2, and why did nothing fail at compile time?
2. Why generate in `build.rs` instead of committing the generated file?
3. Why `protox` instead of `protoc` here, and what does it cost?
