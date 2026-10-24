use std::future::Future;
use std::num::NonZeroU32;
use std::time::Duration;

use thiserror::Error;

/// Position of a message in its partition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Offset(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub offset: Offset,
    pub key: String,
    pub payload: String,
}

/// What goes on the dead-letter topic: the message, how many tries it got, and the last error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadLetter {
    pub original: Message,
    pub attempts: u32,
    /// The Display of the last `HandleError`.
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct HandleError(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("broker: {0}")]
pub struct BrokerError(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConsumerError {
    #[error(transparent)]
    Broker(#[from] BrokerError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryPolicy {
    pub max_attempts: NonZeroU32,
    pub backoff: Duration,
    pub dead_letter_topic: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// `poll` returned None.
    Idle,
    /// The handler succeeded and the offset is committed.
    Processed { offset: Offset },
    /// The `attempt`-th failure; the consumer slept `backoff` and will try again.
    Retrying { offset: Offset, attempt: u32 },
    /// Published to the dead-letter topic, then committed.
    DeadLettered { offset: Offset, attempts: u32 },
}

pub trait Broker {
    /// The first uncommitted message. Returned again on every poll until it is committed; None when caught up.
    fn poll(&mut self) -> impl Future<Output = Result<Option<Message>, BrokerError>> + Send;
    /// Marks `offset` done; the next poll returns the message after it.
    fn commit(&mut self, offset: Offset) -> impl Future<Output = Result<(), BrokerError>> + Send;
    fn publish(
        &mut self,
        topic: &str,
        letter: &DeadLetter,
    ) -> impl Future<Output = Result<(), BrokerError>> + Send;
}

pub trait Handler {
    fn handle(&mut self, msg: &Message) -> impl Future<Output = Result<(), HandleError>> + Send;
}

pub struct Consumer<B, H> {
    broker: B,
    handler: H,
    policy: RetryPolicy,
    /// CLUE: Whose attempts are these? What happens to the count when the head message changes?
    attempts: u32,
}

impl<B: Broker, H: Handler> Consumer<B, H> {
    pub fn new(broker: B, handler: H, policy: RetryPolicy) -> Self {
        Consumer {
            broker,
            handler,
            policy,
            attempts: 0,
        }
    }

    /// CLUE: Publish then commit, or commit then publish: which one can lose a message if the process dies between the two?
    /// At most one handler call per step.
    pub async fn step(&mut self) -> Result<Step, ConsumerError> {
        let Some(msg) = self.broker.poll().await? else {
            return Ok(Step::Idle);
        };
        match self.handler.handle(&msg).await {
            Ok(()) => {
                self.broker.commit(msg.offset).await?;
                self.attempts = 0;
                Ok(Step::Processed { offset: msg.offset })
            }
            // CLUE: The broker hands you this offset again until you commit it. What does the message behind it do meanwhile?
            Err(_) => {
                self.attempts += 1;
                tokio::time::sleep(self.policy.backoff).await;
                Ok(Step::Retrying {
                    offset: msg.offset,
                    attempt: self.attempts,
                })
            }
        }
    }

    pub fn broker(&self) -> &B {
        &self.broker
    }

    pub fn handler(&self) -> &H {
        &self.handler
    }
}
