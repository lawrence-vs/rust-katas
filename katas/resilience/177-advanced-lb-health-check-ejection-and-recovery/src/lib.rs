use std::time::Duration;

/// A backend's place in the pool: 0, 1, 2, ...
pub type BackendId = usize;

/// Outlier detection settings for the pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Eject a backend after this many failures in a row.
    pub consecutive_failures: u32,
    /// How long a backend's first ejection lasts.
    pub base_ejection: Duration,
    /// No ejection lasts longer than this.
    pub max_ejection: Duration,
    /// Successes in a row a backend back from ejection needs before it gets full traffic.
    pub trial_successes: u32,
    /// At most this percent of the pool may be ejected at once, rounded down.
    pub max_ejected_percent: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    NoBackends,
    ZeroFailures,
    ZeroTrial,
    EjectsEveryBackend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownBackend(pub BackendId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Success,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Health {
    /// In the rotation.
    Healthy,
    /// Out of the rotation until `until`.
    Ejected { until: Duration },
    /// Back from an ejection: one request at a time, until `successes` reaches `trial_successes`.
    Trial { successes: u32 },
}

/// CLUE: A backend that was ejected twice: should its next ejection last as long as the first?
#[derive(Debug)]
struct Backend {
    health: Health,
    failures: u32,
}

impl Backend {
    fn available(&self, now: Duration) -> bool {
        match self.health {
            Health::Healthy | Health::Trial { .. } => true,
            Health::Ejected { until } => until <= now,
        }
    }
}

/// A round-robin load balancer that ejects failing backends.
#[derive(Debug)]
pub struct Balancer {
    config: Config,
    backends: Vec<Backend>,
    next: usize,
}

impl Balancer {
    pub fn new(backends: usize, config: Config) -> Result<Self, ConfigError> {
        if backends == 0 {
            return Err(ConfigError::NoBackends);
        }
        if config.consecutive_failures == 0 {
            return Err(ConfigError::ZeroFailures);
        }
        if config.trial_successes == 0 {
            return Err(ConfigError::ZeroTrial);
        }
        if config.max_ejected_percent >= 100 {
            return Err(ConfigError::EjectsEveryBackend);
        }
        let backends = (0..backends)
            .map(|_| Backend {
                health: Health::Healthy,
                failures: 0,
            })
            .collect();
        Ok(Balancer {
            config,
            backends,
            next: 0,
        })
    }

    /// The backend for the next request, in round-robin order, skipping ejected ones.
    /// CLUE: The ejection just ran out. How much traffic should a backend that was down a moment ago get?
    pub fn pick(&mut self, now: Duration) -> BackendId {
        let n = self.backends.len();
        for step in 0..n {
            let id = (self.next + step) % n;
            if self.backends.get(id).is_some_and(|b| b.available(now)) {
                self.next = (id + 1) % n;
                return id;
            }
        }
        // Nothing is available: plain round robin over every backend.
        let id = self.next % n;
        self.next = (id + 1) % n;
        id
    }

    /// Record how a request sent to `id` went.
    /// CLUE: Three backends fail at once. Is it more likely that all three broke, or that something in front of them did?
    pub fn report(
        &mut self,
        id: BackendId,
        outcome: Outcome,
        now: Duration,
    ) -> Result<(), UnknownBackend> {
        let config = self.config;
        let backend = self.backends.get_mut(id).ok_or(UnknownBackend(id))?;
        match outcome {
            Outcome::Success => {
                backend.failures = 0;
                backend.health = Health::Healthy;
            }
            Outcome::Failure => {
                backend.failures = backend.failures.saturating_add(1);
                if backend.failures >= config.consecutive_failures {
                    backend.failures = 0;
                    backend.health = Health::Ejected {
                        until: now.saturating_add(config.base_ejection),
                    };
                }
            }
        }
        Ok(())
    }

    /// The health of `id` at `now`, or `None` for an unknown backend.
    pub fn health(&self, id: BackendId, now: Duration) -> Option<Health> {
        self.backends.get(id).map(|b| match b.health {
            Health::Ejected { until } if until <= now => Health::Healthy,
            h => h,
        })
    }
}
