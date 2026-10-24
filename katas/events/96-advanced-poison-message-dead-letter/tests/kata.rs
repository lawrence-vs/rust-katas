use std::collections::HashMap;
use std::num::NonZeroU32;
use std::time::Duration;

use kata_events_96::{
    Broker, BrokerError, Consumer, ConsumerError, DeadLetter, HandleError, Handler, Message,
    Offset, RetryPolicy, Step,
};
use tokio::time::Instant;

struct FakeBroker {
    log: Vec<Message>,
    next: usize,
    commits: Vec<Offset>,
    published: Vec<(String, DeadLetter)>,
    fail_publishes: u32,
}

impl FakeBroker {
    fn new(log: Vec<Message>) -> Self {
        FakeBroker {
            log,
            next: 0,
            commits: Vec::new(),
            published: Vec::new(),
            fail_publishes: 0,
        }
    }
}

impl Broker for FakeBroker {
    async fn poll(&mut self) -> Result<Option<Message>, BrokerError> {
        Ok(self.log.get(self.next).cloned())
    }

    async fn commit(&mut self, offset: Offset) -> Result<(), BrokerError> {
        self.commits.push(offset);
        self.next = offset.0 as usize + 1;
        Ok(())
    }

    async fn publish(&mut self, topic: &str, letter: &DeadLetter) -> Result<(), BrokerError> {
        if self.fail_publishes > 0 {
            self.fail_publishes -= 1;
            return Err(BrokerError("publish refused".into()));
        }
        self.published.push((topic.to_string(), letter.clone()));
        Ok(())
    }
}

struct ScriptedHandler {
    /// Offset `k` fails `fail[k]` times; `u32::MAX` means always.
    fail: HashMap<Offset, u32>,
    calls: Vec<Offset>,
}

impl Handler for ScriptedHandler {
    async fn handle(&mut self, msg: &Message) -> Result<(), HandleError> {
        self.calls.push(msg.offset);
        match self.fail.get_mut(&msg.offset) {
            Some(left) if *left > 0 => {
                *left -= 1;
                Err(HandleError("boom".into()))
            }
            _ => Ok(()),
        }
    }
}

fn msg(offset: u64, payload: &str) -> Message {
    Message {
        offset: Offset(offset),
        key: format!("k{offset}"),
        payload: payload.into(),
    }
}

fn policy(max_attempts: u32) -> RetryPolicy {
    RetryPolicy {
        max_attempts: NonZeroU32::new(max_attempts).unwrap(),
        backoff: Duration::from_millis(100),
        dead_letter_topic: "events.dlq".into(),
    }
}

fn consumer(
    broker: FakeBroker,
    fail: &[(u64, u32)],
    max_attempts: u32,
) -> Consumer<FakeBroker, ScriptedHandler> {
    let fail = fail.iter().map(|&(k, n)| (Offset(k), n)).collect();
    let handler = ScriptedHandler {
        fail,
        calls: Vec::new(),
    };
    Consumer::new(broker, handler, policy(max_attempts))
}

#[tokio::test(start_paused = true)]
async fn a_good_message_is_processed_and_committed() {
    let mut c = consumer(FakeBroker::new(vec![msg(0, "good")]), &[], 3);
    assert_eq!(
        c.step().await.unwrap(),
        Step::Processed { offset: Offset(0) }
    );
    assert_eq!(c.step().await.unwrap(), Step::Idle);
    assert_eq!(c.broker().commits, vec![Offset(0)]);
}

#[tokio::test(start_paused = true)]
async fn a_transient_failure_succeeds_within_its_attempts() {
    let mut c = consumer(FakeBroker::new(vec![msg(0, "flaky")]), &[(0, 2)], 3);
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(0),
            attempt: 1
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(0),
            attempt: 2
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::Processed { offset: Offset(0) }
    );
    assert!(c.broker().published.is_empty());
}

#[tokio::test(start_paused = true)]
async fn a_poison_message_is_dead_lettered_after_max_attempts() {
    let poison = msg(0, "poison");
    let mut c = consumer(FakeBroker::new(vec![poison.clone()]), &[(0, u32::MAX)], 3);
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(0),
            attempt: 1
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(0),
            attempt: 2
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::DeadLettered {
            offset: Offset(0),
            attempts: 3
        }
    );
    assert_eq!(c.handler().calls, vec![Offset(0), Offset(0), Offset(0)]);
    assert_eq!(
        c.broker().published,
        vec![(
            "events.dlq".to_string(),
            DeadLetter {
                original: poison,
                attempts: 3,
                error: "boom".into(),
            }
        )]
    );
    assert_eq!(c.broker().commits, vec![Offset(0)]);
}

#[tokio::test(start_paused = true)]
async fn the_next_message_is_processed_after_a_dead_letter() {
    let log = vec![msg(0, "poison"), msg(1, "good")];
    let mut c = consumer(FakeBroker::new(log), &[(0, u32::MAX)], 3);
    for _ in 0..3 {
        c.step().await.unwrap();
    }
    assert_eq!(
        c.step().await.unwrap(),
        Step::Processed { offset: Offset(1) }
    );
    assert_eq!(c.step().await.unwrap(), Step::Idle);
}

#[tokio::test(start_paused = true)]
async fn attempts_are_counted_per_message() {
    let log = vec![msg(0, "poison"), msg(1, "fails-twice")];
    let mut c = consumer(FakeBroker::new(log), &[(0, u32::MAX), (1, 2)], 3);
    for _ in 0..3 {
        c.step().await.unwrap();
    }
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(1),
            attempt: 1
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::Retrying {
            offset: Offset(1),
            attempt: 2
        }
    );
    assert_eq!(
        c.step().await.unwrap(),
        Step::Processed { offset: Offset(1) }
    );
}

#[tokio::test(start_paused = true)]
async fn a_failed_dead_letter_publish_does_not_commit() {
    let mut broker = FakeBroker::new(vec![msg(0, "poison")]);
    broker.fail_publishes = 1;
    let mut c = consumer(broker, &[(0, u32::MAX)], 3);
    for _ in 0..2 {
        c.step().await.unwrap();
    }
    let err = c.step().await.unwrap_err();
    assert!(matches!(err, ConsumerError::Broker(_)), "got {err:?}");
    assert!(c.broker().commits.is_empty());

    assert_eq!(
        c.step().await.unwrap(),
        Step::DeadLettered {
            offset: Offset(0),
            attempts: 3
        }
    );
    assert_eq!(c.handler().calls.len(), 3);
    assert_eq!(c.broker().commits, vec![Offset(0)]);
}

#[tokio::test(start_paused = true)]
async fn a_poison_message_waits_only_between_retries() {
    let mut c = consumer(FakeBroker::new(vec![msg(0, "poison")]), &[(0, u32::MAX)], 3);
    let start = Instant::now();
    for _ in 0..3 {
        c.step().await.unwrap();
    }
    assert_eq!(start.elapsed(), Duration::from_millis(200));
}

#[tokio::test(start_paused = true)]
async fn max_attempts_of_one_dead_letters_on_the_first_failure() {
    let mut c = consumer(FakeBroker::new(vec![msg(0, "poison")]), &[(0, u32::MAX)], 1);
    let start = Instant::now();
    assert_eq!(
        c.step().await.unwrap(),
        Step::DeadLettered {
            offset: Offset(0),
            attempts: 1
        }
    );
    assert_eq!(start.elapsed(), Duration::ZERO);
}
