use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId(pub i64);

/// The broker's id for one command; a redelivery carries the same id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fill {
    pub command: CommandId,
    pub order: OrderId,
    pub qty: i64,
}

/// The event downstream consumers see; `command` lets them drop duplicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderFilled {
    pub command: CommandId,
    pub order: OrderId,
    pub qty: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("sink unavailable")]
pub struct SinkDown;

/// The message broker. Every call that returns Ok delivers the event.
pub trait Sink {
    fn publish(&mut self, event: &OrderFilled) -> Result<(), SinkDown>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashPoint {
    /// In `handle`: after the order row is updated, before anything else is written or published.
    AfterStateChange,
    /// In `publish_pending`: after the sink accepted an event, before it is recorded as published.
    AfterPublish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Applied,
    Duplicate,
}

#[derive(Debug, thiserror::Error)]
pub enum OutboxError {
    #[error("database error")]
    Db(#[from] sqlx::Error),
    #[error("order {0:?} not found")]
    UnknownOrder(OrderId),
    #[error("simulated crash at {0:?}")]
    Crashed(CrashPoint),
    #[error("sink unavailable")]
    Sink(#[from] SinkDown),
}

/// CLUE: Only `orders` exists. Where would the handler remember a command it already applied, or an event it still owes the sink?
pub async fn setup() -> Result<SqlitePool, OutboxError> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("CREATE TABLE orders (id INTEGER PRIMARY KEY, filled INTEGER NOT NULL)")
        .execute(&pool)
        .await?;
    Ok(pool)
}

pub struct Handler<S> {
    pool: SqlitePool,
    sink: S,
    crash: Option<CrashPoint>,
}

impl<S: Sink> Handler<S> {
    pub fn new(pool: SqlitePool, sink: S) -> Self {
        Self {
            pool,
            sink,
            crash: None,
        }
    }

    pub fn sink(&self) -> &S {
        &self.sink
    }

    /// Arms one crash: the next time the code reaches `at`, it fails once and disarms.
    pub fn crash_once(&mut self, at: CrashPoint) {
        self.crash = Some(at);
    }

    fn crash_if(&mut self, at: CrashPoint) -> Result<(), OutboxError> {
        if self.crash == Some(at) {
            self.crash = None;
            return Err(OutboxError::Crashed(at));
        }
        Ok(())
    }

    pub async fn open_order(&self, order: OrderId) -> Result<(), OutboxError> {
        sqlx::query("INSERT INTO orders (id, filled) VALUES (?, 0)")
            .bind(order.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn filled(&self, order: OrderId) -> Result<Option<i64>, OutboxError> {
        let filled = sqlx::query_scalar("SELECT filled FROM orders WHERE id = ?")
            .bind(order.0)
            .fetch_optional(&self.pool)
            .await?;
        Ok(filled)
    }

    /// CLUE: Each `execute(&self.pool)` commits on its own. What's in the database if the process dies on the line after the UPDATE?
    /// CLUE: The broker redelivers any command it didn't see acknowledged. What does a second `handle` with the same `CommandId` do to `filled`?
    pub async fn handle(&mut self, fill: Fill) -> Result<Outcome, OutboxError> {
        let updated = sqlx::query("UPDATE orders SET filled = filled + ? WHERE id = ?")
            .bind(fill.qty)
            .bind(fill.order.0)
            .execute(&self.pool)
            .await?;
        if updated.rows_affected() == 0 {
            return Err(OutboxError::UnknownOrder(fill.order));
        }
        self.crash_if(CrashPoint::AfterStateChange)?;
        self.sink.publish(&OrderFilled {
            command: fill.command,
            order: fill.order,
            qty: fill.qty,
        })?;
        Ok(Outcome::Applied)
    }

    /// Publishes every unpublished event in commit order; returns how many it published.
    ///
    /// CLUE: Nothing is ever pending because `handle` publishes itself. Where does the event go when the sink is down at that moment?
    pub async fn publish_pending(&mut self) -> Result<usize, OutboxError> {
        Ok(0)
    }
}
