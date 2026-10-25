use std::time::Duration;

use kata_resilience_177::{
    BackendId, Balancer, Config, ConfigError, Health, Outcome, UnknownBackend,
};

fn secs(s: u64) -> Duration {
    Duration::from_secs(s)
}

fn config() -> Config {
    Config {
        consecutive_failures: 3,
        base_ejection: secs(10),
        max_ejection: secs(30),
        trial_successes: 2,
        max_ejected_percent: 50,
    }
}

fn balancer(n: usize) -> Balancer {
    Balancer::new(n, config()).unwrap()
}

/// Report `outcome` for backend `id`, `times` times, at `now`.
fn report_n(b: &mut Balancer, id: BackendId, outcome: Outcome, times: u32, now: Duration) {
    for _ in 0..times {
        b.report(id, outcome, now).unwrap();
    }
}

/// Fake backends behind the balancer. `up[i]` says whether backend `i` answers.
struct Pool {
    balancer: Balancer,
    up: Vec<bool>,
    hits: Vec<u32>,
}

impl Pool {
    fn new(up: &[bool]) -> Self {
        Pool {
            balancer: balancer(up.len()),
            up: up.to_vec(),
            hits: vec![0; up.len()],
        }
    }

    /// `count` requests at `now`: pick, call the fake backend, report. Returns the picks.
    fn send(&mut self, count: usize, now: Duration) -> Vec<BackendId> {
        (0..count)
            .map(|_| {
                let id = self.balancer.pick(now);
                self.hits[id] += 1;
                let outcome = if self.up[id] {
                    Outcome::Success
                } else {
                    Outcome::Failure
                };
                self.balancer.report(id, outcome, now).unwrap();
                id
            })
            .collect()
    }
}

#[test]
fn new_rejects_a_bad_config() {
    assert_eq!(
        Balancer::new(0, config()).err(),
        Some(ConfigError::NoBackends)
    );
    let c = Config {
        consecutive_failures: 0,
        ..config()
    };
    assert_eq!(Balancer::new(3, c).err(), Some(ConfigError::ZeroFailures));
    let c = Config {
        trial_successes: 0,
        ..config()
    };
    assert_eq!(Balancer::new(3, c).err(), Some(ConfigError::ZeroTrial));
    let c = Config {
        max_ejected_percent: 100,
        ..config()
    };
    assert_eq!(
        Balancer::new(3, c).err(),
        Some(ConfigError::EjectsEveryBackend)
    );
}

#[test]
fn round_robin_over_healthy_backends() {
    let mut pool = Pool::new(&[true, true, true]);
    assert_eq!(pool.send(6, secs(0)), vec![0, 1, 2, 0, 1, 2]);
}

#[test]
fn an_unknown_backend_is_an_error() {
    let mut b = balancer(3);
    assert_eq!(
        b.report(3, Outcome::Success, secs(0)),
        Err(UnknownBackend(3))
    );
    assert_eq!(b.health(3, secs(0)), None);
}

#[test]
fn only_consecutive_failures_eject() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 2, secs(0));
    report_n(&mut b, 1, Outcome::Success, 1, secs(0));
    report_n(&mut b, 1, Outcome::Failure, 2, secs(0));
    assert_eq!(b.health(1, secs(0)), Some(Health::Healthy));
    report_n(&mut b, 1, Outcome::Failure, 1, secs(0));
    assert_eq!(
        b.health(1, secs(0)),
        Some(Health::Ejected { until: secs(10) })
    );
}

#[test]
fn an_ejected_backend_gets_no_traffic() {
    let mut pool = Pool::new(&[true, false, true]);
    assert_eq!(pool.send(9, secs(0)), vec![0, 1, 2, 0, 1, 2, 0, 1, 2]);
    assert_eq!(
        pool.balancer.health(1, secs(0)),
        Some(Health::Ejected { until: secs(10) })
    );
    assert_eq!(pool.send(4, secs(5)), vec![0, 2, 0, 2]);
    assert_eq!(pool.hits, vec![5, 3, 5]);
}

#[test]
fn one_lucky_probe_is_not_full_traffic() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));

    report_n(&mut b, 1, Outcome::Success, 1, secs(10));
    let h = b.health(1, secs(10));
    assert_eq!(
        h,
        Some(Health::Trial { successes: 1 }),
        "a backend got full traffic back after one lucky probe: {h:?}"
    );

    report_n(&mut b, 1, Outcome::Success, 1, secs(10));
    assert_eq!(b.health(1, secs(10)), Some(Health::Healthy));
}

#[test]
fn an_expired_ejection_starts_a_trial() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));
    assert_eq!(
        b.health(1, Duration::from_millis(9_999)),
        Some(Health::Ejected { until: secs(10) })
    );
    assert_eq!(
        b.health(1, secs(10)),
        Some(Health::Trial { successes: 0 }),
        "an ejected backend came back without a trial"
    );
}

#[test]
fn a_trial_backend_takes_one_request_at_a_time() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));

    // Nobody reports back, so backend 1's trial request stays in flight.
    let picks: Vec<BackendId> = (0..5).map(|_| b.pick(secs(10))).collect();
    assert_eq!(
        picks,
        vec![0, 1, 2, 0, 2],
        "a backend on trial took more than one request at a time"
    );

    report_n(&mut b, 1, Outcome::Success, 1, secs(10));
    let picks: Vec<BackendId> = (0..3).map(|_| b.pick(secs(10))).collect();
    assert_eq!(picks, vec![0, 1, 2]);
}

#[test]
fn a_full_trial_restores_full_traffic() {
    let mut pool = Pool::new(&[true, false, true]);
    pool.send(9, secs(0));
    pool.up[1] = true;

    pool.send(6, secs(10));
    assert_eq!(pool.balancer.health(1, secs(10)), Some(Health::Healthy));
    assert_eq!(pool.send(6, secs(10)), vec![0, 1, 2, 0, 1, 2]);
}

#[test]
fn a_failed_probe_doubles_the_ejection_up_to_the_max() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));

    report_n(&mut b, 1, Outcome::Failure, 1, secs(10));
    let h = b.health(1, secs(10));
    assert_eq!(
        h,
        Some(Health::Ejected { until: secs(30) }),
        "a failed probe did not eject the backend again for twice as long: {h:?}"
    );

    report_n(&mut b, 1, Outcome::Failure, 1, secs(30));
    let h = b.health(1, secs(30));
    assert_eq!(
        h,
        Some(Health::Ejected { until: secs(60) }),
        "an ejection did not stop growing at max_ejection: {h:?}"
    );
}

#[test]
fn a_recovered_backend_starts_over_at_the_base_ejection() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));
    report_n(&mut b, 1, Outcome::Failure, 1, secs(10));
    report_n(&mut b, 1, Outcome::Success, 2, secs(30));
    assert_eq!(b.health(1, secs(30)), Some(Health::Healthy));

    report_n(&mut b, 1, Outcome::Failure, 3, secs(30));
    assert_eq!(
        b.health(1, secs(30)),
        Some(Health::Ejected { until: secs(40) })
    );
}

#[test]
fn a_late_reply_from_an_ejected_backend_changes_nothing() {
    let mut b = balancer(3);
    report_n(&mut b, 1, Outcome::Failure, 3, secs(0));
    report_n(&mut b, 1, Outcome::Success, 1, secs(5));
    assert_eq!(
        b.health(1, secs(5)),
        Some(Health::Ejected { until: secs(10) }),
        "a reply from an ejected backend put it back in the rotation"
    );
}

#[test]
fn never_ejects_more_than_the_max_percent() {
    let mut pool = Pool::new(&[false, false, false]);
    pool.send(9, secs(0));

    let health: Vec<Option<Health>> = (0..3).map(|id| pool.balancer.health(id, secs(1))).collect();
    assert_eq!(
        health,
        vec![
            Some(Health::Ejected { until: secs(10) }),
            Some(Health::Healthy),
            Some(Health::Healthy)
        ],
        "more than max_ejected_percent of the pool was ejected"
    );
    assert_eq!(pool.send(4, secs(1)), vec![1, 2, 1, 2]);
}

#[test]
fn a_single_backend_is_never_ejected() {
    let mut pool = Pool::new(&[false]);
    assert_eq!(pool.send(5, secs(0)), vec![0; 5]);
    assert_eq!(
        pool.balancer.health(0, secs(0)),
        Some(Health::Healthy),
        "the only backend was ejected"
    );
}
