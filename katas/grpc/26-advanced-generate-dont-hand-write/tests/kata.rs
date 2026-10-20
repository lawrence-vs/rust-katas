use kata_grpc_26::pb::greeter_client::GreeterClient;
use kata_grpc_26::pb::greeter_server::{Greeter, GreeterServer};
use kata_grpc_26::pb::{HelloReply, HelloRequest};
use prost::Message;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Request, Response, Status};

#[test]
fn decodes_what_the_proto_says() {
    // Field 1, wire type 2 (length-delimited), 3 bytes: "ada".
    let wire = [0x0a, 3, b'a', b'd', b'a'];
    assert_eq!(HelloRequest::decode(&wire[..]).unwrap().name, "ada");
}

struct Svc;

#[tonic::async_trait]
impl Greeter for Svc {
    async fn say_hello(&self, req: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("hello {}", req.into_inner().name),
        }))
    }
}

#[tokio::test]
async fn round_trip_over_grpc() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(GreeterServer::new(Svc))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    let mut client = GreeterClient::connect(format!("http://{addr}"))
        .await
        .unwrap();
    let reply = client
        .say_hello(HelloRequest { name: "ada".into() })
        .await
        .unwrap();
    assert_eq!(reply.into_inner().message, "hello ada");
}
