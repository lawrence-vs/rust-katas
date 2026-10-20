use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use kata_websockets_24::serve;
use tokio::net::TcpListener;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn silent_client_still_receives() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    tokio::spawn(serve(listener));

    let (mut listener_client, _) = connect_async(&url).await.unwrap();
    let (mut talker, _) = connect_async(&url).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    talker.send(Message::text("hi")).await.unwrap();

    let got = timeout(Duration::from_secs(2), listener_client.next())
        .await
        .expect("silent client got nothing in 2s")
        .unwrap()
        .unwrap();
    assert_eq!(got, Message::text("hi"));
}
