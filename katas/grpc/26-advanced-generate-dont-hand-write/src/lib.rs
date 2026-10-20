//! Messages for the `hello.Greeter` service in `proto/hello.proto`, written by hand.

/// CLUE: Compare every field's tag with `proto/hello.proto`. Who keeps them in sync when the .proto changes?
/// CLUE: `tonic-prost-build` and `protox` are already build-dependencies. What reads them, and what does `tonic::include_proto!` expect?
pub mod pb {
    #[derive(Clone, PartialEq, prost::Message)]
    pub struct HelloRequest {
        #[prost(string, tag = "2")]
        pub name: String,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    pub struct HelloReply {
        #[prost(string, tag = "1")]
        pub message: String,
    }
}
