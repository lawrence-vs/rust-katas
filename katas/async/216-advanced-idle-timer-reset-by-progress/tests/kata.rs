use std::time::Duration;

use kata_async_216::{End, run};
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};

const HEARTBEAT: Duration = Duration::from_millis(100);
const IDLE: Duration = Duration::from_secs(1);

#[tokio::test(start_paused = true)]
async fn silent_peer_goes_idle_despite_heartbeats() {
    let (_tx, rx) = mpsc::channel(8);
    let end = timeout(Duration::from_secs(10), run(rx, IDLE, HEARTBEAT))
        .await
        .expect("idle never fired");
    assert_eq!(end, End::Idle);
}

#[tokio::test(start_paused = true)]
async fn frames_keep_it_alive_until_closed() {
    let (tx, rx) = mpsc::channel(8);
    let task = tokio::spawn(run(rx, IDLE, HEARTBEAT));
    for i in 0..6 {
        sleep(Duration::from_millis(500)).await;
        tx.send(format!("frame {i}")).await.unwrap();
    }
    drop(tx);
    let end = timeout(Duration::from_secs(10), task)
        .await
        .expect("closed never seen")
        .unwrap();
    assert_eq!(end, End::Closed);
}
