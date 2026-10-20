use kata_security_21::{Rejected, ingest};

#[tokio::test]
async fn floods_are_rejected() {
    let (front, mut rx) = ingest(8);
    let results: Vec<_> = (0..100u8).map(|i| front.accept(vec![i; 1024])).collect();
    assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 8);
    assert_eq!(results[99], Err(Rejected));
    let mut queued = 0;
    while rx.try_recv().is_ok() {
        queued += 1;
    }
    assert_eq!(queued, 8);
}

#[tokio::test]
async fn accepts_again_after_draining() {
    let (front, mut rx) = ingest(1);
    front.accept(b"a".to_vec()).unwrap();
    assert_eq!(front.accept(b"b".to_vec()), Err(Rejected));
    assert_eq!(rx.recv().await.unwrap(), b"a");
    front.accept(b"c".to_vec()).unwrap();
}
