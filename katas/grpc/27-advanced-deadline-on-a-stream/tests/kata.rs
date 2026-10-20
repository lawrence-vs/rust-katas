use std::pin::Pin;
use std::time::Duration;

use kata_grpc_27::first_n;
use kata_grpc_27::pb::ticker_client::TickerClient;
use kata_grpc_27::pb::ticker_server::{Ticker, TickerServer};
use kata_grpc_27::pb::{Tick, WatchRequest};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Code, Request, Response, Status};

/// Sends two ticks, then keeps the stream open and silent forever.
struct Stalls;

#[tonic::async_trait]
impl Ticker for Stalls {
    type WatchStream = Pin<Box<dyn Stream<Item = Result<Tick, Status>> + Send>>;

    async fn watch(&self, _: Request<WatchRequest>) -> Result<Response<Self::WatchStream>, Status> {
        let ticks = ["1.00", "1.01"].map(|p| Ok(Tick { price: p.into() }));
        let stream = tokio_stream::iter(ticks).chain(tokio_stream::pending());
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::test]
async fn stalled_stream_hits_a_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(TickerServer::new(Stalls))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    let mut client = TickerClient::connect(format!("http://{addr}"))
        .await
        .unwrap();
    let result = tokio::time::timeout(Duration::from_secs(10), first_n(&mut client, "BTC", 5))
        .await
        .expect("first_n hung for 10s");
    assert_eq!(result.unwrap_err().code(), Code::DeadlineExceeded);
}
