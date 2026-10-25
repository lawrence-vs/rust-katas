use std::sync::{Arc, Mutex};
use std::time::Duration;

use kata_async_178::{Config, Exit, Refused, Request, Service};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::{Instant, sleep, timeout};

const CONFIG: Config = Config {
    grace_delay: Duration::from_millis(500),
    deadline: Duration::from_secs(2),
};
/// The fake load balancer probes readiness on every 10th send tick (every 100 ms)
/// and ejects the pod after 3 failed probes in a row.
const SEND_EVERY: Duration = Duration::from_millis(10);
const PROBE_EVERY_N_TICKS: u64 = 10;
const FAILURE_THRESHOLD: u32 = 3;
const HUNG: Duration = Duration::from_secs(60);

fn req(id: u64, ms: u64) -> Request {
    Request {
        id,
        takes: Duration::from_millis(ms),
    }
}

/// Start `run`; returns the SIGTERM sender and a handle to how and when it exited.
fn start(svc: &Service) -> (oneshot::Sender<()>, JoinHandle<(Exit, Instant)>) {
    let (tx, rx) = oneshot::channel::<()>();
    let s = svc.clone();
    let exit = tokio::spawn(async move {
        let exit = s
            .run(
                async move {
                    let _ = rx.await;
                },
                CONFIG,
            )
            .await;
        (exit, Instant::now())
    });
    (tx, exit)
}

async fn finish(exit: JoinHandle<(Exit, Instant)>) -> (Exit, Instant) {
    timeout(HUNG, exit)
        .await
        .expect("the service never exited")
        .unwrap()
}

fn spawn_request(svc: &Service, r: Request) -> JoinHandle<Result<u64, Refused>> {
    let s = svc.clone();
    tokio::spawn(async move { s.handle(r).await })
}

#[derive(Debug, Default)]
struct Lb {
    sent: u64,
    ok: u64,
    refused: u64,
}

/// A load balancer: every 10 ms it sends a 50 ms request; every 100 ms it probes readiness
/// first, and it ejects the pod (stops sending) after 3 failed probes in a row.
fn load_balancer(svc: &Service) -> (Arc<Mutex<Lb>>, JoinHandle<()>) {
    let stats = Arc::new(Mutex::new(Lb::default()));
    let (s, st) = (svc.clone(), stats.clone());
    let task = tokio::spawn(async move {
        let mut failures = 0u32;
        let mut requests = Vec::new();
        let mut tick = tokio::time::interval(SEND_EVERY);
        for n in 0u64.. {
            tick.tick().await;
            if n % PROBE_EVERY_N_TICKS == 0 {
                if s.ready() {
                    failures = 0;
                } else {
                    failures += 1;
                }
                if failures == FAILURE_THRESHOLD {
                    break;
                }
            }
            let (s, st) = (s.clone(), st.clone());
            requests.push(tokio::spawn(async move {
                let result = s.handle(req(n + 1, 50)).await;
                let mut st = st.lock().unwrap();
                st.sent += 1;
                match result {
                    Ok(_) => st.ok += 1,
                    Err(Refused) => st.refused += 1,
                }
            }));
        }
        for r in requests {
            r.await.unwrap();
        }
    });
    (stats, task)
}

#[tokio::test(start_paused = true)]
async fn a_running_service_is_ready_live_and_serving() {
    let svc = Service::new();
    assert!(svc.ready());
    assert!(svc.live());
    assert_eq!(svc.handle(req(1, 5)).await, Ok(1));
}

#[tokio::test(start_paused = true)]
async fn readiness_goes_false_as_soon_as_sigterm_arrives() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    sigterm.send(()).unwrap();
    sleep(Duration::from_millis(1)).await;

    assert!(
        !svc.ready(),
        "readiness was still true after SIGTERM, so the load balancer keeps sending"
    );
    assert_eq!(finish(exit).await.0, Exit::Clean);
}

#[tokio::test(start_paused = true)]
async fn the_service_keeps_serving_through_the_grace_delay() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    sigterm.send(()).unwrap();
    sleep(CONFIG.grace_delay - Duration::from_millis(10)).await;

    assert!(!svc.ready());
    assert_eq!(
        svc.handle(req(1, 5)).await,
        Ok(1),
        "a request was refused during the grace delay"
    );
    assert_eq!(finish(exit).await.0, Exit::Clean);
}

#[tokio::test(start_paused = true)]
async fn the_load_balancer_sees_no_connection_errors_during_a_rolling_update() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    let (stats, lb) = load_balancer(&svc);
    sleep(Duration::from_millis(1005)).await;
    sigterm.send(()).unwrap();
    let sigterm_at = Instant::now();

    timeout(HUNG, lb)
        .await
        .expect("the load balancer never ejected the pod")
        .unwrap();
    let (exit, exited_at) = finish(exit).await;
    let lb = stats.lock().unwrap();

    assert_eq!(lb.sent, 130);
    assert_eq!(
        lb.refused, 0,
        "the load balancer got connection errors during shutdown: {} of {} requests refused",
        lb.refused, lb.sent
    );
    assert_eq!(lb.ok, lb.sent);
    assert_eq!(exit, Exit::Clean);
    let took = exited_at.duration_since(sigterm_at);
    assert!(
        took >= CONFIG.grace_delay,
        "the service exited {took:?} after SIGTERM, before its grace delay"
    );
    assert!(took <= CONFIG.deadline);
}

#[tokio::test(start_paused = true)]
async fn liveness_stays_true_until_exit() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    let busy = spawn_request(&svc, req(1, 800));
    sleep(Duration::from_millis(10)).await;
    sigterm.send(()).unwrap();

    let mut probes = 0;
    while !exit.is_finished() && probes < 1000 {
        assert!(
            svc.live(),
            "liveness went false during shutdown at probe {probes}, so the kubelet would restart the pod"
        );
        probes += 1;
        sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(finish(exit).await.0, Exit::Clean);
    assert_eq!(busy.await.unwrap(), Ok(1));
}

#[tokio::test(start_paused = true)]
async fn in_flight_requests_finish_before_exit() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    let busy = spawn_request(&svc, req(1, 800));
    sleep(Duration::from_millis(10)).await;
    sigterm.send(()).unwrap();
    let sigterm_at = Instant::now();

    let (exit, exited_at) = finish(exit).await;

    assert_eq!(exit, Exit::Clean);
    assert!(
        exited_at.duration_since(sigterm_at) >= Duration::from_millis(790),
        "the service exited before its in-flight request finished"
    );
    assert_eq!(busy.await.unwrap(), Ok(1));
}

#[tokio::test(start_paused = true)]
async fn exit_happens_within_the_termination_deadline() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    let _stuck = spawn_request(&svc, req(1, 3_600_000));
    sleep(Duration::from_millis(10)).await;
    sigterm.send(()).unwrap();
    let sigterm_at = Instant::now();

    let (exit, exited_at) = finish(exit).await;
    let took = exited_at.duration_since(sigterm_at);

    assert_eq!(exit, Exit::Killed { in_flight: 1 });
    assert!(
        took <= CONFIG.deadline,
        "the service exited {took:?} after SIGTERM, past the {:?} termination deadline",
        CONFIG.deadline
    );
    assert!(took >= CONFIG.deadline - Duration::from_millis(10));
}

#[tokio::test(start_paused = true)]
async fn after_the_grace_delay_new_requests_are_refused() {
    let svc = Service::new();
    let (sigterm, exit) = start(&svc);
    let busy = spawn_request(&svc, req(1, 800));
    sleep(Duration::from_millis(10)).await;
    sigterm.send(()).unwrap();
    sleep(CONFIG.grace_delay + Duration::from_millis(10)).await;

    assert_eq!(
        svc.handle(req(2, 5)).await,
        Err(Refused),
        "a request was accepted after the grace delay, during the drain"
    );
    assert_eq!(busy.await.unwrap(), Ok(1));
    assert_eq!(finish(exit).await.0, Exit::Clean);
}
