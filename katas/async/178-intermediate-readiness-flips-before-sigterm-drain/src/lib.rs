use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tokio::sync::watch;
use tokio::time::timeout;

/// How the service shuts down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// How long the service keeps accepting after readiness goes false, so the load balancer has time to notice.
    pub grace_delay: Duration,
    /// The orchestrator's kill deadline, counted from SIGTERM (`terminationGracePeriodSeconds`).
    pub deadline: Duration,
}

/// One request from the load balancer. Serving it takes `takes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Request {
    pub id: u64,
    pub takes: Duration,
}

/// The connection was refused: the service had stopped accepting. The load balancer sees a connection error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refused;

/// How `run` ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    /// Every in-flight request finished.
    Clean,
    /// The deadline came first; this many requests were still running.
    Killed { in_flight: usize },
}

struct Shared {
    /// CLUE: The load balancer, the kubelet and the accept check all read this one flag. Should all three change at the same moment?
    serving: AtomicBool,
    in_flight: watch::Sender<usize>,
}

/// Counts one request as in flight until it is dropped.
struct InFlight<'a>(&'a watch::Sender<usize>);

impl<'a> InFlight<'a> {
    fn enter(count: &'a watch::Sender<usize>) -> Self {
        count.send_modify(|n| *n += 1);
        InFlight(count)
    }
}

impl Drop for InFlight<'_> {
    fn drop(&mut self) {
        self.0.send_modify(|n| *n = n.saturating_sub(1));
    }
}

/// A service behind a load balancer. Clones share one state.
#[derive(Clone)]
pub struct Service {
    shared: Arc<Shared>,
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

impl Service {
    pub fn new() -> Service {
        Service {
            shared: Arc::new(Shared {
                serving: AtomicBool::new(true),
                in_flight: watch::channel(0).0,
            }),
        }
    }

    /// The readiness probe (`GET /readyz`): should the load balancer send traffic here?
    pub fn ready(&self) -> bool {
        self.shared.serving.load(Ordering::SeqCst)
    }

    /// The liveness probe (`GET /livez`): is the process healthy, or should the kubelet restart it?
    pub fn live(&self) -> bool {
        self.shared.serving.load(Ordering::SeqCst)
    }

    /// Serve one request, or refuse the connection.
    pub async fn handle(&self, req: Request) -> Result<u64, Refused> {
        let _in_flight = InFlight::enter(&self.shared.in_flight);
        if !self.shared.serving.load(Ordering::SeqCst) {
            return Err(Refused);
        }
        tokio::time::sleep(req.takes).await;
        Ok(req.id)
    }

    /// Serve until `sigterm` resolves, then shut down and say how it ended.
    ///
    /// CLUE: The load balancer probes readiness every few seconds and needs several failures in a row to eject the pod. What meets a request it sends in that gap?
    ///
    /// CLUE: The kill deadline is counted from SIGTERM. When does this timeout's clock start?
    pub async fn run<F: Future<Output = ()>>(&self, sigterm: F, config: Config) -> Exit {
        sigterm.await;
        self.shared.serving.store(false, Ordering::SeqCst);
        let mut idle = self.shared.in_flight.subscribe();
        let drained = timeout(config.deadline, idle.wait_for(|n| *n == 0))
            .await
            .is_ok();
        if drained {
            Exit::Clean
        } else {
            Exit::Killed {
                in_flight: *idle.borrow(),
            }
        }
    }
}
