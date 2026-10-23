use kata_db_61::{
    CommandId, CrashPoint, Fill, Handler, OrderFilled, OrderId, OutboxError, Outcome, Sink,
    SinkDown, setup,
};

const O1: OrderId = OrderId(1);
const O2: OrderId = OrderId(2);
const C1: CommandId = CommandId(10);
const C2: CommandId = CommandId(11);
const QTY: i64 = 5;

struct FakeSink {
    events: Vec<OrderFilled>,
    down: usize,
}

impl Sink for FakeSink {
    fn publish(&mut self, event: &OrderFilled) -> Result<(), SinkDown> {
        if self.down > 0 {
            self.down -= 1;
            return Err(SinkDown);
        }
        self.events.push(*event);
        Ok(())
    }
}

async fn handler(down: usize) -> Handler<FakeSink> {
    let pool = setup().await.expect("setup");
    let h = Handler::new(
        pool,
        FakeSink {
            events: Vec::new(),
            down,
        },
    );
    h.open_order(O1).await.expect("open O1");
    h.open_order(O2).await.expect("open O2");
    h
}

fn fill(command: CommandId, order: OrderId) -> Fill {
    Fill {
        command,
        order,
        qty: QTY,
    }
}

fn event(command: CommandId, order: OrderId) -> OrderFilled {
    OrderFilled {
        command,
        order,
        qty: QTY,
    }
}

#[tokio::test]
async fn a_fill_is_applied_and_published() {
    let mut h = handler(0).await;
    let r = h.handle(fill(C1, O1)).await;
    assert!(
        matches!(r, Ok(Outcome::Applied)),
        "a fill is applied and published: {r:?}"
    );
    h.publish_pending().await.expect("drain");
    assert_eq!(
        h.filled(O1).await.expect("filled"),
        Some(QTY),
        "a fill is applied and published"
    );
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1)],
        "a fill is applied and published"
    );
}

#[tokio::test]
async fn a_redelivered_command_is_applied_once() {
    let mut h = handler(0).await;
    h.handle(fill(C1, O1)).await.expect("first");
    let r = h.handle(fill(C1, O1)).await;
    assert!(
        matches!(r, Ok(Outcome::Duplicate)),
        "a redelivered command is applied once: {r:?}"
    );
    h.publish_pending().await.expect("drain");
    assert_eq!(
        h.filled(O1).await.expect("filled"),
        Some(QTY),
        "a redelivered command is applied once"
    );
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1)],
        "a redelivered command is applied once"
    );
}

#[tokio::test]
async fn a_crash_after_the_state_change_writes_nothing() {
    let mut h = handler(0).await;
    h.crash_once(CrashPoint::AfterStateChange);
    let r = h.handle(fill(C1, O1)).await;
    assert!(
        matches!(r, Err(OutboxError::Crashed(CrashPoint::AfterStateChange))),
        "a crash after the state change writes nothing: {r:?}"
    );
    assert_eq!(
        h.filled(O1).await.expect("filled"),
        Some(0),
        "a crash after the state change writes nothing"
    );
    let r = h.publish_pending().await;
    assert!(
        r.is_ok(),
        "a crash after the state change writes nothing: {r:?}"
    );
    assert!(
        h.sink().events.is_empty(),
        "a crash after the state change writes nothing"
    );
}

#[tokio::test]
async fn a_crash_then_a_redelivery_applies_the_fill_once() {
    let mut h = handler(0).await;
    h.crash_once(CrashPoint::AfterStateChange);
    let _ = h.handle(fill(C1, O1)).await;
    let r = h.handle(fill(C1, O1)).await;
    assert!(
        matches!(r, Ok(Outcome::Applied)),
        "a crash then a redelivery applies the fill once: {r:?}"
    );
    assert_eq!(
        h.filled(O1).await.expect("filled"),
        Some(QTY),
        "a crash then a redelivery applies the fill once"
    );
    h.publish_pending().await.expect("drain");
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1)],
        "a crash then a redelivery applies the fill once"
    );
}

#[tokio::test]
async fn a_sink_outage_does_not_lose_the_event() {
    let mut h = handler(1).await;
    let r = h.handle(fill(C1, O1)).await;
    assert!(
        matches!(r, Ok(Outcome::Applied)),
        "a sink outage does not lose the event: {r:?}"
    );
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Err(OutboxError::Sink(_))),
        "a sink outage does not lose the event: {r:?}"
    );
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Ok(1)),
        "a sink outage does not lose the event: {r:?}"
    );
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1)],
        "a sink outage does not lose the event"
    );
}

#[tokio::test]
async fn the_publisher_drains_in_order_and_only_once() {
    let mut h = handler(0).await;
    h.handle(fill(C1, O1)).await.expect("first");
    h.handle(fill(C2, O2)).await.expect("second");
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Ok(2)),
        "the publisher drains in order and only once: {r:?}"
    );
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Ok(0)),
        "the publisher drains in order and only once: {r:?}"
    );
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1), event(C2, O2)],
        "the publisher drains in order and only once"
    );
}

#[tokio::test]
async fn a_crash_after_publishing_republishes_the_same_event() {
    let mut h = handler(0).await;
    h.crash_once(CrashPoint::AfterPublish);
    h.handle(fill(C1, O1)).await.expect("handle");
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Err(OutboxError::Crashed(CrashPoint::AfterPublish))),
        "a crash after publishing republishes the same event: {r:?}"
    );
    let r = h.publish_pending().await;
    assert!(
        matches!(r, Ok(1)),
        "a crash after publishing republishes the same event: {r:?}"
    );
    assert_eq!(
        h.sink().events,
        vec![event(C1, O1), event(C1, O1)],
        "a crash after publishing republishes the same event"
    );
}

#[tokio::test]
async fn an_unknown_order_writes_nothing() {
    let mut h = handler(0).await;
    let r = h.handle(fill(C1, OrderId(9))).await;
    assert!(
        matches!(r, Err(OutboxError::UnknownOrder(OrderId(9)))),
        "an unknown order writes nothing: {r:?}"
    );
    h.open_order(OrderId(9)).await.expect("open 9");
    let r = h.handle(fill(C1, OrderId(9))).await;
    assert!(
        matches!(r, Ok(Outcome::Applied)),
        "an unknown order writes nothing: {r:?}"
    );
    h.publish_pending().await.expect("drain");
    assert_eq!(
        h.sink().events,
        vec![event(C1, OrderId(9))],
        "an unknown order writes nothing"
    );
}
