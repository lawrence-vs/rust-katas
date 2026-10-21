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

## Think further
1. Next month someone deletes `name` from `hello.proto` and adds `string nickname = 1;`. Old clients still send tag 1. What does the server read, and what line in the `.proto` would have stopped the reuse?
2. The client is on an older `.proto` and the server on a newer one with an extra field. What does prost do with a field number it doesn't know when decoding? Does anyone get an error?
3. Once it passes, edit `proto/hello.proto` and run `cargo test` again. Does cargo rerun your build script? What tells it to?
4. Two services in two repos share `hello.proto`. Where should the one copy live, and how does each build get it?

## In the wild
- [Protobuf language guide (proto3)](https://protobuf.dev/programming-guides/proto3/): see "Updating A Message Type" and "Reserved Fields".
- [Protobuf dos and don'ts](https://protobuf.dev/best-practices/dos-donts/): includes the rule about re-using a tag number.
- [`tonic-prost-build` docs](https://docs.rs/tonic-prost-build/0.14.6/tonic_prost_build/): the client and server generation options.
- [tonic's own examples build script](https://github.com/grpc/grpc-rust/blob/master/examples/build.rs): every example service is generated in one place.
- [opentelemetry-proto's `grpc_build.rs` test](https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-proto/tests/grpc_build.rs): a counterexample that commits generated code and guards it with a test that regenerates and diffs it.
- [Buf breaking-change detection](https://buf.build/docs/breaking/): the same check as a CI step.
- Exchange-style: order-entry and market-data schemas are shared by gateways, the matching engine and client SDKs. A renumbered field on an order message decodes as the default (empty or zero) instead of failing.

In a review, check:
- Hand-written `#[prost(..., tag = "...")]` structs, or a committed generated `.rs` with no regenerate-and-diff check.
- A removed field with no `reserved` for its number and name.
- A field number or type changed in a PR with no breaking-change check in CI.
- Whether editing the `.proto` actually triggers a rebuild.
