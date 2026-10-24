use std::time::Duration;

use kata_async_65::{Session, run};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

fn frame(body: &[u8]) -> Vec<u8> {
    let mut out = (body.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(body);
    out
}

#[tokio::test(start_paused = true)]
async fn frames_without_heartbeats() {
    let (mut tx, rx) = tokio::io::duplex(64);
    let h = tokio::spawn(run(rx, Duration::from_secs(1)));
    tx.write_all(&frame(b"hello")).await.unwrap();
    tx.write_all(&frame(b"world")).await.unwrap();
    drop(tx);
    let session = h.await.unwrap().unwrap();
    assert_eq!(
        session,
        Session {
            frames: vec![b"hello".to_vec(), b"world".to_vec()],
            heartbeats: 0
        }
    );
}

#[tokio::test(start_paused = true)]
async fn heartbeats_fire_while_the_line_is_idle() {
    let (tx, rx) = tokio::io::duplex(64);
    let h = tokio::spawn(run(rx, Duration::from_secs(1)));
    sleep(Duration::from_millis(3500)).await;
    drop(tx);
    let session = h.await.unwrap().unwrap();
    assert_eq!(
        session,
        Session {
            frames: vec![],
            heartbeats: 3
        }
    );
}

#[tokio::test(start_paused = true)]
async fn a_heartbeat_mid_frame_loses_nothing() {
    let (mut tx, rx) = tokio::io::duplex(64);
    let h = tokio::spawn(run(rx, Duration::from_secs(1)));
    tx.write_all(&[0, 0, 0, 5]).await.unwrap();
    tx.write_all(b"he").await.unwrap();
    sleep(Duration::from_millis(1500)).await;
    tx.write_all(b"llo").await.unwrap();
    tx.write_all(&frame(b"world")).await.unwrap();
    drop(tx);
    let session = h
        .await
        .unwrap()
        .unwrap_or_else(|e| panic!("a heartbeat mid-frame lost bytes: {e}"));
    assert_eq!(
        session,
        Session {
            frames: vec![b"hello".to_vec(), b"world".to_vec()],
            heartbeats: 1
        }
    );
}
