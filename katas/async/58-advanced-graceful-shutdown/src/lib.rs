use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;

/// Ids of settled jobs, in the order they settled.
pub type Ledger = Arc<std::sync::Mutex<Vec<u64>>>;

type Queue = Arc<tokio::sync::Mutex<mpsc::Receiver<Job>>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub takes: Duration,
}

/// Settle one job: it takes `job.takes`, then lands in the ledger. Don't change it.
pub async fn settle(job: Job, ledger: &Ledger) {
    tokio::time::sleep(job.takes).await;
    ledger
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(job.id);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Closed;

#[derive(Clone)]
pub struct Submitter {
    tx: mpsc::Sender<Job>,
}

impl Submitter {
    pub async fn submit(&self, job: Job) -> Result<(), Closed> {
        self.tx.send(job).await.map_err(|_| Closed)
    }
}

/// What `shutdown` hands back.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Ids that were queued but never started, in submit order.
    pub unstarted: Vec<u64>,
    /// Workers that were still running when the grace period ran out.
    pub aborted: usize,
}

pub struct Settler {
    submitter: Submitter,
}

impl Settler {
    /// Start `workers` workers sharing one queue. Call it inside a runtime.
    ///
    /// CLUE: `tokio::spawn` returns a `JoinHandle`. When you drop it, is the task stopped or let go?
    pub fn start(workers: usize, ledger: Ledger) -> Settler {
        let (tx, rx) = mpsc::channel(64);
        let queue: Queue = Arc::new(tokio::sync::Mutex::new(rx));
        for _ in 0..workers {
            tokio::spawn(worker(queue.clone(), ledger.clone()));
        }
        Settler {
            submitter: Submitter { tx },
        }
    }

    pub fn submitter(&self) -> Submitter {
        self.submitter.clone()
    }

    /// Stop the pool: in-flight jobs finish, workers are joined within `grace`,
    /// stragglers are aborted and counted, queued jobs are handed back.
    ///
    /// CLUE: When this returns, which of three places is every submitted job in?
    pub async fn shutdown(self, _grace: Duration) -> Report {
        Report::default()
    }
}

/// CLUE: `recv` returns `None` only when every `Sender` is gone. Who still holds one when shutdown starts?
///
/// CLUE: A worker parked in `recv` can't see a shutdown it isn't listening for. What else could it wait on at the same time?
async fn worker(queue: Queue, ledger: Ledger) {
    loop {
        let Some(job) = queue.lock().await.recv().await else {
            return;
        };
        settle(job, &ledger).await;
    }
}
