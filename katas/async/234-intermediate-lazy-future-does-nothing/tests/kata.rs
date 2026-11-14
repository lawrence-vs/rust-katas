use kata_async_234::Session;

#[tokio::test]
async fn started_flag_set_after_run() {
    let session = Session::new();
    session.run().await;
    assert!(
        session.is_started(),
        "run() returned but the session never started"
    );
}
