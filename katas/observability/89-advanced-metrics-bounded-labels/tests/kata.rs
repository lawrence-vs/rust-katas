use std::time::Duration;

use kata_observability_89::{Kind, Outcome, Request, describe, record_request};
use metrics_util::debugging::{DebugValue, DebuggingRecorder};

type Series = (Vec<(String, String)>, Option<metrics::Unit>, DebugValue);

/// Runs `f` under a thread-local recorder and returns the series called `name`.
fn series(name: &str, f: impl FnOnce()) -> Vec<Series> {
    let recorder = DebuggingRecorder::new();
    let snapshotter = recorder.snapshotter();
    metrics::with_local_recorder(&recorder, || {
        describe();
        f();
    });
    let all: Vec<_> = snapshotter.snapshot().into_vec();
    let names: Vec<String> = all
        .iter()
        .map(|(key, _, _, _)| key.key().name().to_string())
        .collect();
    let found: Vec<Series> = all
        .into_iter()
        .filter(|(key, _, _, _)| key.key().name() == name)
        .map(|(key, unit, _, value)| {
            let mut labels: Vec<(String, String)> = key
                .key()
                .labels()
                .map(|l| (l.key().to_string(), l.value().to_string()))
                .collect();
            labels.sort();
            (labels, unit, value)
        })
        .collect();
    assert!(!found.is_empty(), "no series named {name}; got {names:?}");
    found
}

fn req(account_id: u64, kind: Kind) -> Request {
    Request { account_id, kind }
}

#[test]
fn a_thousand_accounts_make_at_most_six_series() {
    let outcomes = [Outcome::Ok, Outcome::Rejected, Outcome::Failed];
    let kinds = [Kind::Read, Kind::Write];
    let found = series("app_requests_total", || {
        for id in 0..1000u64 {
            let outcome = outcomes[(id % 3) as usize];
            let kind = kinds[(id % 2) as usize];
            record_request(&req(id, kind), outcome, Duration::from_millis(5));
        }
    });
    let n = found.len();
    assert!(
        n <= 6,
        "one series per account: {n} series for 1000 requests"
    );
    assert!(
        found
            .iter()
            .all(|(labels, _, _)| labels.iter().all(|(k, _)| k != "account")),
        "no label key may be `account`"
    );
}

#[test]
fn counter_labels_are_outcome_and_kind() {
    let found = series("app_requests_total", || {
        record_request(
            &req(7, Kind::Write),
            Outcome::Rejected,
            Duration::from_millis(1),
        );
    });
    assert_eq!(found.len(), 1);
    let (labels, _, value) = &found[0];
    let want = vec![
        ("kind".to_string(), "write".to_string()),
        ("outcome".to_string(), "rejected".to_string()),
    ];
    assert_eq!(labels, &want, "labels were {labels:?}");
    assert_eq!(*value, DebugValue::Counter(1));
}

#[test]
fn the_same_outcome_and_kind_add_up() {
    let found = series("app_requests_total", || {
        for id in [1, 2, 3] {
            record_request(&req(id, Kind::Read), Outcome::Ok, Duration::from_millis(1));
        }
    });
    assert_eq!(found.len(), 1, "same labels must share one series");
    assert_eq!(found[0].2, DebugValue::Counter(3));
}

#[test]
fn durations_are_recorded_in_seconds() {
    let found = series("app_request_duration_seconds", || {
        record_request(&req(1, Kind::Read), Outcome::Ok, Duration::from_millis(250));
    });
    let DebugValue::Histogram(values) = &found[0].2 else {
        panic!("app_request_duration_seconds is not a histogram");
    };
    let got: Vec<f64> = values.iter().map(|v| v.0).collect();
    assert!(
        got.len() == 1 && (got[0] - 0.25).abs() < 1e-9,
        "duration recorded as {got:?}, expected 0.25 seconds"
    );
}

#[test]
fn duration_is_described_in_seconds() {
    let found = series("app_request_duration_seconds", || {
        record_request(&req(1, Kind::Read), Outcome::Ok, Duration::from_millis(250));
    });
    assert_eq!(
        found[0].1,
        Some(metrics::Unit::Seconds),
        "histogram has no unit of seconds"
    );
}
