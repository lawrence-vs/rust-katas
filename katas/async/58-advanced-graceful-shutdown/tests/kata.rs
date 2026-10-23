use std::sync::Arc;
use std::time::Duration;

use kata_async_58::{Closed, Job, Ledger, Report, Settler};
use tokio::time::{Instant, sleep, timeout};

const GRACE: Duration = Duration::from_secs(1);
const HUNG: Duration = Duration::from_secs(5);

fn job(id: u64, ms: u64) -> Job {
    Job {
        id,
        takes: Duration::from_millis(ms),
    }
}

fn settled(ledger: &Ledger) -> Vec<u64> {
    let mut ids = ledger.lock().unwrap().clone();
    ids.sort();
    ids
}

async fn shut(settler: Settler) -> Report {
    timeout(HUNG, settler.shutdown(GRACE))
        .await
        .expect("shutdown hung on a channel that never closed")
}

#[tokio::test(start_paused = true)]
async fn in_flight_settlements_finish_before_shutdown_returns() {
    let ledger: Ledger = Arc::new(Default::default());
    let settler = Settler::start(3, ledger.clone());
    let sub = settler.submitter();
    for id in 1..=3 {
        sub.submit(job(id, 100)).await.unwrap();
    }
    drop(sub);
    sleep(Duration::from_millis(10)).await;

    let report = shut(settler).await;

    assert_eq!(
        settled(&ledger),
        vec![1, 2, 3],
        "in-flight settlements were lost: settled {:?} of [1, 2, 3]",
        settled(&ledger)
    );
    assert_eq!(report, Report::default());
}

#[tokio::test(start_paused = true)]
async fn shutdown_returns_while_a_submitter_is_still_alive() {
    let ledger: Ledger = Arc::new(Default::default());
    let settler = Settler::start(2, ledger.clone());
    let sub = settler.submitter();
    sub.submit(job(7, 100)).await.unwrap();
    sleep(Duration::from_millis(10)).await;
    let t = Instant::now();

    shut(settler).await;

    assert_eq!(settled(&ledger), vec![7]);
    assert!(t.elapsed() < GRACE, "shutdown waited past its grace period");
    assert_eq!(
        sub.submit(job(8, 1)).await,
        Err(Closed),
        "a submit after shutdown was accepted"
    );
}

#[tokio::test(start_paused = true)]
async fn a_stuck_settlement_is_aborted_after_the_grace_period() {
    let ledger: Ledger = Arc::new(Default::default());
    let settler = Settler::start(2, ledger.clone());
    let sub = settler.submitter();
    sub.submit(job(1, 100)).await.unwrap();
    sub.submit(job(2, 3_600_000)).await.unwrap();
    sleep(Duration::from_millis(10)).await;
    let t = Instant::now();

    let report = shut(settler).await;

    assert_eq!(report.aborted, 1, "the stuck worker was not aborted");
    assert!(
        t.elapsed() <= GRACE + Duration::from_millis(10),
        "shutdown took {:?}",
        t.elapsed()
    );
    sleep(Duration::from_secs(2 * 3600)).await;
    assert_eq!(
        settled(&ledger),
        vec![1],
        "a timed-out worker kept running after shutdown"
    );
}

#[tokio::test(start_paused = true)]
async fn queued_jobs_that_never_started_are_handed_back() {
    let ledger: Ledger = Arc::new(Default::default());
    let settler = Settler::start(1, ledger.clone());
    let sub = settler.submitter();
    for id in 1..=5 {
        sub.submit(job(id, 100)).await.unwrap();
    }
    sleep(Duration::from_millis(10)).await;

    let report = shut(settler).await;

    assert_eq!(
        settled(&ledger),
        vec![1],
        "a job started after shutdown was signalled"
    );
    assert_eq!(
        report.unstarted,
        vec![2, 3, 4, 5],
        "queued jobs were lost instead of handed back"
    );
}
