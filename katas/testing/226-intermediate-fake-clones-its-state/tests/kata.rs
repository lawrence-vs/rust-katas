use kata_testing_226::FakeTransport;

#[test]
fn test_side_sees_frames_sent_by_actor_side() {
    let test_side = FakeTransport::default();
    let mut actor_side = test_side.clone();
    actor_side.send("subscribe");
    assert_eq!(
        test_side.sent(),
        vec!["subscribe".to_string()],
        "sent frames not visible"
    );
}

#[test]
fn single_handle_records_and_reads_back() {
    let mut transport = FakeTransport::default();
    transport.send("ping");
    transport.send("pong");
    assert_eq!(
        transport.sent(),
        vec!["ping".to_string(), "pong".to_string()]
    );
}
