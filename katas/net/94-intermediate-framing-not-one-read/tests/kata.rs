use std::time::Duration;

use kata_net_94::{FrameError, MAX_LINE, read_messages};
use tokio::io::{AsyncWriteExt, duplex};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

/// Writes each chunk, pausing between chunks so the reader sees them as separate reads.
async fn feed(chunks: Vec<Vec<u8>>) -> Result<Vec<String>, FrameError> {
    let (mut tx, rx) = duplex(64);
    tokio::spawn(async move {
        for (i, chunk) in chunks.iter().enumerate() {
            if i > 0 {
                sleep(Duration::from_millis(10)).await;
            }
            if tx.write_all(chunk).await.is_err() {
                return;
            }
        }
    });
    read_messages(rx).await
}

#[tokio::test(start_paused = true)]
async fn one_message_per_read() {
    let got = feed(vec![b"PING\n".to_vec()]).await.unwrap();
    assert_eq!(got, vec!["PING"]);
}

#[tokio::test(start_paused = true)]
async fn one_message_split_across_two_reads() {
    let got = feed(vec![b"HEL".to_vec(), b"LO\nBYE\n".to_vec()])
        .await
        .unwrap();
    assert_eq!(got, vec!["HELLO", "BYE"]);
}

#[tokio::test(start_paused = true)]
async fn two_messages_in_one_read() {
    let (mut tx, rx) = duplex(64);
    tx.write_all(b"PING\nPONG\n").await.unwrap();
    drop(tx);
    let got = read_messages(rx).await.unwrap();
    assert_eq!(got, vec!["PING", "PONG"]);
}

#[tokio::test(start_paused = true)]
async fn a_line_of_exactly_max_is_accepted() {
    let mut line = vec![b'a'; MAX_LINE];
    line.push(b'\n');
    let got = feed(vec![line]).await.unwrap();
    assert_eq!(got, vec!["a".repeat(MAX_LINE)]);
}

#[tokio::test(start_paused = true)]
async fn a_line_longer_than_max_is_an_error() {
    let mut line = vec![b'a'; MAX_LINE + 1];
    line.push(b'\n');
    let got = feed(vec![line]).await;
    assert!(matches!(got, Err(FrameError::TooLong)), "got {got:?}");
}

#[tokio::test]
async fn real_tcp_socket_split_write() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let Ok(mut client) = TcpStream::connect(addr).await else {
            return;
        };
        let _ = client.set_nodelay(true);
        if client.write_all(b"HEL").await.is_err() {
            return;
        }
        sleep(Duration::from_millis(50)).await;
        let _ = client.write_all(b"LO\nBYE\n").await;
    });
    let (server, _) = listener.accept().await.unwrap();
    let got = tokio::time::timeout(Duration::from_secs(10), read_messages(server))
        .await
        .expect("timed out")
        .unwrap();
    assert_eq!(got, vec!["HELLO", "BYE"]);
}
