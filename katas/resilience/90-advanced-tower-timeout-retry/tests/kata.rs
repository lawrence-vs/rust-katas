use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use kata_resilience_90::{Config, Lookup, UpstreamError, fetch};
use tokio::time::{Instant, sleep, timeout};
use tower::timeout::error::Elapsed;
use tower::{BoxError, service_fn};

fn cfg() -> Config {
    Config {
        per_try_timeout: Duration::from_secs(1),
        max_retries: 3,
        first_backoff: Duration::from_millis(100),
    }
}

/// Run `fetch` under a 60 s guard on the paused clock.
async fn guarded<S>(svc: S) -> (Result<String, BoxError>, Duration)
where
    S: tower::Service<Lookup, Response = String, Error = UpstreamError> + Clone,
{
    let start = Instant::now();
    let res = timeout(
        Duration::from_secs(60),
        fetch(svc, Lookup { id: 7 }, &cfg()),
    )
    .await
    .expect("fetch hung for 60s");
    (res, start.elapsed())
}

#[tokio::test(start_paused = true)]
async fn first_try_succeeds() {
    let calls = Arc::new(AtomicU32::new(0));
    let c = calls.clone();
    let svc = service_fn(move |_: Lookup| {
        let c = c.clone();
        async move {
            sleep(Duration::from_millis(10)).await;
            c.fetch_add(1, Ordering::SeqCst);
            Ok::<_, UpstreamError>("rec-7".to_string())
        }
    });
    let (res, _) = guarded(svc).await;
    assert_eq!(res.ok(), Some("rec-7".to_string()));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn transient_then_success() {
    let calls = Arc::new(AtomicU32::new(0));
    let c = calls.clone();
    let svc = service_fn(move |_: Lookup| {
        let c = c.clone();
        async move {
            sleep(Duration::from_millis(10)).await;
            if c.fetch_add(1, Ordering::SeqCst) < 2 {
                Err(UpstreamError::Unavailable)
            } else {
                Ok("rec-7".to_string())
            }
        }
    });
    let (res, elapsed) = guarded(svc).await;
    assert_eq!(res.ok(), Some("rec-7".to_string()));
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert!(
        elapsed >= Duration::from_millis(300),
        "no backoff between retries: {elapsed:?}"
    );
}

#[tokio::test(start_paused = true)]
async fn permanent_error_not_retried() {
    let calls = Arc::new(AtomicU32::new(0));
    let c = calls.clone();
    let svc = service_fn(move |_: Lookup| {
        let c = c.clone();
        async move {
            sleep(Duration::from_millis(10)).await;
            c.fetch_add(1, Ordering::SeqCst);
            Err::<String, _>(UpstreamError::NotFound)
        }
    });
    let (res, _) = guarded(svc).await;
    let err = res.err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        err.as_ref().and_then(|e| e.downcast_ref::<UpstreamError>()),
        Some(&UpstreamError::NotFound)
    );
}

#[tokio::test(start_paused = true)]
async fn gives_up_after_max_retries() {
    let calls = Arc::new(AtomicU32::new(0));
    let c = calls.clone();
    let svc = service_fn(move |_: Lookup| {
        let c = c.clone();
        async move {
            sleep(Duration::from_millis(10)).await;
            c.fetch_add(1, Ordering::SeqCst);
            Err::<String, _>(UpstreamError::Unavailable)
        }
    });
    let (res, elapsed) = guarded(svc).await;
    let err = res.err();
    assert_eq!(calls.load(Ordering::SeqCst), 4);
    assert_eq!(
        err.as_ref().and_then(|e| e.downcast_ref::<UpstreamError>()),
        Some(&UpstreamError::Unavailable)
    );
    assert!(elapsed >= Duration::from_millis(700), "elapsed {elapsed:?}");
}

#[tokio::test(start_paused = true)]
async fn hung_upstream_times_out_each_try() {
    let calls = Arc::new(AtomicU32::new(0));
    let c = calls.clone();
    let svc = service_fn(move |_: Lookup| {
        let c = c.clone();
        async move {
            c.fetch_add(1, Ordering::SeqCst);
            std::future::pending::<Result<String, UpstreamError>>().await
        }
    });
    let (res, elapsed) = guarded(svc).await;
    let err = res.err();
    assert!(err.as_ref().is_some_and(|e| e.is::<Elapsed>()));
    assert_eq!(calls.load(Ordering::SeqCst), 4);
    assert!(elapsed >= Duration::from_secs(4), "elapsed {elapsed:?}");
    assert!(elapsed < Duration::from_secs(60), "elapsed {elapsed:?}");
}
