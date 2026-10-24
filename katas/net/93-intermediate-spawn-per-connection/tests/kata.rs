use std::net::SocketAddr;
use std::time::Duration;

use kata_net_93::serve;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, timeout};

const BOUND: Duration = Duration::from_secs(2);

async fn start() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(serve(listener));
    addr
}

async fn connect(addr: SocketAddr) -> TcpStream {
    timeout(BOUND, TcpStream::connect(addr))
        .await
        .expect("connect timed out")
        .expect("connect")
}

/// Sends `line\n` and reads one reply line.
async fn ask(stream: &mut TcpStream, line: &str) -> String {
    stream
        .write_all(format!("{line}\n").as_bytes())
        .await
        .expect("write");
    let mut reply = String::new();
    BufReader::new(stream)
        .read_line(&mut reply)
        .await
        .expect("read");
    reply
}

#[tokio::test]
async fn answers_one_client() {
    let addr = start().await;
    let mut c = connect(addr).await;
    let reply = timeout(BOUND, ask(&mut c, "hello"))
        .await
        .expect("no reply");
    assert_eq!(reply, "#1 hello\n");
}

#[tokio::test]
async fn stalled_client_does_not_block_the_next() {
    let addr = start().await;
    let mut c1 = connect(addr).await;
    // Let the server accept the stalled client first.
    sleep(Duration::from_millis(100)).await;
    let mut c2 = connect(addr).await;

    let reply = timeout(BOUND, ask(&mut c2, "second"))
        .await
        .expect("a stalled client blocked the next one");
    assert_eq!(reply, "#1 second\n");

    let reply = timeout(BOUND, ask(&mut c1, "first"))
        .await
        .expect("the stalled client was never served");
    assert_eq!(reply, "#2 first\n");
}

#[tokio::test]
async fn concurrent_clients_share_one_count() {
    let addr = start().await;
    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(tokio::spawn(async move {
            let mut c = connect(addr).await;
            timeout(BOUND, ask(&mut c, &format!("client {i}")))
                .await
                .expect("no reply")
        }));
    }
    let mut counts = Vec::new();
    for h in handles {
        let reply = h.await.expect("client task");
        let n: u64 = reply
            .strip_prefix('#')
            .and_then(|r| r.split(' ').next())
            .and_then(|n| n.parse().ok())
            .expect("reply starts with #<n>");
        counts.push(n);
    }
    counts.sort_unstable();
    assert_eq!(counts, (1..=10).collect::<Vec<u64>>());
}

#[tokio::test]
async fn hang_up_is_not_counted() {
    let addr = start().await;
    drop(connect(addr).await);
    sleep(Duration::from_millis(100)).await;

    let mut c = connect(addr).await;
    let reply = timeout(BOUND, ask(&mut c, "next")).await.expect("no reply");
    assert_eq!(reply, "#1 next\n");
}
