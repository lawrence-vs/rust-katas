use kata_websockets_25::fan_out;
use tokio::sync::{broadcast, mpsc};

#[tokio::test]
async fn lagging_client_keeps_receiving() {
    let (room, rx) = broadcast::channel(2);
    for m in ["a", "b", "c", "d", "e"] {
        room.send(m.to_string()).unwrap();
    }
    drop(room);
    let (outbox, mut client) = mpsc::channel(16);
    let missed = fan_out(rx, outbox).await;
    let mut got = Vec::new();
    while let Some(m) = client.recv().await {
        got.push(m);
    }
    assert_eq!(got, ["d", "e"]);
    assert_eq!(missed, 3);
}
