use std::sync::Arc;
use std::time::Duration;

use kata_grpc_166::pb::statement_client::StatementClient;
use kata_grpc_166::pb::statement_server::{Statement, StatementServer};
use kata_grpc_166::pb::{Entry, WatchRequest};
use kata_grpc_166::{StatementService, Stats};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Status};

fn req() -> Request<WatchRequest> {
    Request::new(WatchRequest {
        account_id: "acc-1".into(),
    })
}

/// Reads the next entry's `seq`. Fails the test if the stream hangs, ends, or errors.
async fn next_seq<S>(s: &mut S) -> u64
where
    S: Stream<Item = Result<Entry, Status>> + Unpin,
{
    tokio::time::timeout(Duration::from_secs(2), s.next())
        .await
        .expect("stream hung for 2s")
        .expect("stream ended")
        .expect("stream returned an error")
        .seq
}

/// Polls until no producer is alive, or gives up after `secs`.
async fn wait_idle(stats: &Stats, secs: u64) -> bool {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(secs);
    while tokio::time::Instant::now() < deadline {
        if stats.running() == 0 {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    stats.running() == 0
}

#[tokio::test]
async fn entries_arrive_in_order() {
    let stats = Arc::new(Stats::default());
    let svc = StatementService::new(stats.clone());
    let mut stream = svc.watch(req()).await.unwrap().into_inner();
    let mut got = Vec::new();
    for _ in 0..3 {
        got.push(next_seq(&mut stream).await);
    }
    assert_eq!(got, [1, 2, 3]);
}

#[tokio::test]
async fn slow_client_gets_back_pressure_not_growth() {
    let stats = Arc::new(Stats::default());
    let svc = StatementService::new(stats.clone());
    let mut stream = svc.watch(req()).await.unwrap().into_inner();
    assert_eq!(next_seq(&mut stream).await, 1);
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(
        stats.produced() <= 6,
        "producer ran ahead of a slow client: {} entries made",
        stats.produced()
    );
    let mut rest = Vec::new();
    for _ in 0..3 {
        rest.push(next_seq(&mut stream).await);
    }
    assert_eq!(rest, [2, 3, 4]);
}

#[tokio::test]
async fn producer_stops_when_the_client_drops_the_stream() {
    let stats = Arc::new(Stats::default());
    let svc = StatementService::new(stats.clone());
    let mut stream = svc.watch(req()).await.unwrap().into_inner();
    assert_eq!(next_seq(&mut stream).await, 1);
    assert_eq!(stats.running(), 1);
    drop(stream);
    assert!(
        wait_idle(&stats, 2).await,
        "producer still running after client drop"
    );
}

#[tokio::test]
async fn producer_stops_when_dropped_before_the_first_read() {
    let stats = Arc::new(Stats::default());
    let svc = StatementService::new(stats.clone());
    let stream = svc.watch(req()).await.unwrap().into_inner();
    assert_eq!(stats.running(), 1);
    drop(stream);
    assert!(
        wait_idle(&stats, 2).await,
        "producer still running after client drop"
    );
}

#[tokio::test]
async fn each_stream_gets_its_own_producer() {
    let stats = Arc::new(Stats::default());
    let svc = StatementService::new(stats.clone());
    let a = svc.watch(req()).await.unwrap().into_inner();
    let b = svc.watch(req()).await.unwrap().into_inner();
    let c = svc.watch(req()).await.unwrap().into_inner();
    assert_eq!(stats.running(), 3);
    drop(a);
    drop(b);
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(stats.running(), 1);
    drop(c);
    assert!(
        wait_idle(&stats, 2).await,
        "producer still running after client drop"
    );
}

#[tokio::test]
async fn wire_disconnect_stops_the_producer() {
    let stats = Arc::new(Stats::default());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(StatementServer::new(StatementService::new(stats.clone())))
            .serve_with_incoming(TcpListenerStream::new(listener)),
    );
    // Keep the client alive, so only the stream is cancelled, not the connection.
    let mut client = StatementClient::connect(format!("http://{addr}"))
        .await
        .unwrap();
    let mut stream = client.watch(req()).await.unwrap().into_inner();
    assert_eq!(next_seq(&mut stream).await, 1);
    assert_eq!(next_seq(&mut stream).await, 2);
    assert_eq!(stats.running(), 1);
    drop(stream);
    assert!(
        wait_idle(&stats, 5).await,
        "producer still running after client drop"
    );
    drop(client);
}
