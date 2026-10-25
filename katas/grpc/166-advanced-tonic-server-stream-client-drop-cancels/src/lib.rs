use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use tokio::sync::mpsc;
use tokio_stream::Stream;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("statement");
}

use pb::statement_server::Statement;
use pb::{Entry, WatchRequest};

/// How many entries may wait for a slow client.
pub const STREAM_BUFFER: usize = 4;

const TICK: Duration = Duration::from_millis(1);

/// What the tests can see of the producers: entries made, and producers still alive.
#[derive(Default)]
pub struct Stats {
    produced: AtomicU64,
    running: AtomicUsize,
}

impl Stats {
    /// Entries the producer has made so far.
    pub fn produced(&self) -> u64 {
        self.produced.load(Ordering::SeqCst)
    }

    /// Producer tasks still alive.
    pub fn running(&self) -> usize {
        self.running.load(Ordering::SeqCst)
    }
}

/// Counts one producer as alive until the guard is dropped.
struct Alive(Arc<Stats>);

impl Alive {
    fn new(stats: Arc<Stats>) -> Self {
        stats.running.fetch_add(1, Ordering::SeqCst);
        Alive(stats)
    }
}

impl Drop for Alive {
    fn drop(&mut self) {
        self.0.running.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct StatementService {
    stats: Arc<Stats>,
}

impl StatementService {
    pub fn new(stats: Arc<Stats>) -> Self {
        Self { stats }
    }
}

#[tonic::async_trait]
impl Statement for StatementService {
    type WatchStream = Pin<Box<dyn Stream<Item = Result<Entry, Status>> + Send>>;

    /// CLUE: A client reads one entry and stops. Where do the next thousand entries wait, and how big can that place grow?
    /// CLUE: The client is gone. What does the producer's `send` return now, and what does this loop do with it?
    /// CLUE: Which tokio channel has a capacity, and what does its sender do when it is full?
    async fn watch(&self, _: Request<WatchRequest>) -> Result<Response<Self::WatchStream>, Status> {
        let (tx, rx) = mpsc::unbounded_channel();
        let alive = Alive::new(self.stats.clone());
        let stats = self.stats.clone();
        tokio::spawn(async move {
            let _alive = alive;
            let mut seq = 0u64;
            loop {
                tokio::time::sleep(TICK).await;
                seq += 1;
                stats.produced.fetch_add(1, Ordering::SeqCst);
                let _ = tx.send(Ok(Entry { seq }));
            }
        });
        Ok(Response::new(Box::pin(UnboundedReceiverStream::new(rx))))
    }
}
