use std::io::ErrorKind;
use std::time::Duration;

use kata_security_22::read_greeting;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;

#[tokio::test(start_paused = true)]
async fn reads_a_greeting() {
    let (client, mut server) = tokio::io::duplex(64);
    server.write_all(b"HELLO v1\n").await.unwrap();
    assert_eq!(read_greeting(client).await.unwrap(), "HELLO v1");
}

#[tokio::test(start_paused = true)]
async fn silent_peer_times_out() {
    let (client, _server) = tokio::io::duplex(64);
    let result = timeout(Duration::from_secs(60), read_greeting(client))
        .await
        .expect("read_greeting hung for 60s");
    assert_eq!(result.unwrap_err().kind(), ErrorKind::TimedOut);
}
