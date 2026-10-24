use std::cell::Cell;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Duration, Instant};

use kata_resilience_97::{Breaker, CallError, Clock, Config, State};

/// A clock the test moves by hand. Clones share the same time.
#[derive(Clone)]
struct FakeClock(Rc<Cell<Instant>>);

impl FakeClock {
    fn new() -> Self {
        FakeClock(Rc::new(Cell::new(Instant::now())))
    }

    fn advance(&self, by: Duration) {
        self.0.set(self.0.get() + by);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        self.0.get()
    }
}

fn config(threshold: u32, cool_down: Duration) -> Config {
    Config {
        failure_threshold: NonZeroU32::new(threshold).unwrap(),
        cool_down,
    }
}

/// One request. The dependency counts the attempt, then answers `outcome`.
async fn request(
    breaker: &mut Breaker<FakeClock>,
    calls: &Cell<u32>,
    outcome: Result<u32, &'static str>,
) -> Result<u32, CallError<&'static str>> {
    breaker
        .call(|| {
            calls.set(calls.get() + 1);
            async move { outcome }
        })
        .await
}

/// Make `threshold` failing requests in a row.
async fn trip(breaker: &mut Breaker<FakeClock>, calls: &Cell<u32>, threshold: u32) {
    for _ in 0..threshold {
        let _ = request(breaker, calls, Err("down")).await;
    }
}

#[tokio::test]
async fn closed_passes_calls_through() {
    let mut breaker = Breaker::new(FakeClock::new(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);

    assert_eq!(request(&mut breaker, &calls, Ok(7)).await, Ok(7));
    assert_eq!(calls.get(), 1);
    assert_eq!(breaker.state(), State::Closed);
}

#[tokio::test]
async fn a_success_resets_the_failure_count() {
    let mut breaker = Breaker::new(FakeClock::new(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);

    for outcome in [Err("down"), Err("down"), Ok(1), Err("down"), Err("down")] {
        let _ = request(&mut breaker, &calls, outcome).await;
    }
    assert_eq!(breaker.state(), State::Closed);
}

#[tokio::test]
async fn one_request_is_one_attempt() {
    let mut breaker = Breaker::new(FakeClock::new(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);

    assert_eq!(
        request(&mut breaker, &calls, Err("down")).await,
        Err(CallError::Inner("down"))
    );
    assert_eq!(calls.get(), 1);
}

#[tokio::test]
async fn opens_after_threshold_consecutive_failures() {
    let mut breaker = Breaker::new(FakeClock::new(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);

    trip(&mut breaker, &calls, 3).await;
    assert_eq!(breaker.state(), State::Open);
}

#[tokio::test]
async fn open_fails_fast_without_calling_dependency() {
    let mut breaker = Breaker::new(FakeClock::new(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);
    trip(&mut breaker, &calls, 3).await;

    for _ in 0..10 {
        assert_eq!(
            request(&mut breaker, &calls, Ok(1)).await,
            Err(CallError::Open)
        );
    }
    assert_eq!(calls.get(), 3);
}

#[tokio::test]
async fn half_open_after_cool_down() {
    let clock = FakeClock::new();
    let mut breaker = Breaker::new(clock.clone(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);
    trip(&mut breaker, &calls, 3).await;

    clock.advance(Duration::from_millis(29_999));
    assert_eq!(breaker.state(), State::Open);
    assert_eq!(
        request(&mut breaker, &calls, Ok(1)).await,
        Err(CallError::Open)
    );
    assert_eq!(calls.get(), 3);

    clock.advance(Duration::from_millis(1));
    assert_eq!(breaker.state(), State::HalfOpen);
    assert_eq!(request(&mut breaker, &calls, Ok(1)).await, Ok(1));
    assert_eq!(calls.get(), 4);
}

#[tokio::test]
async fn half_open_success_closes() {
    let clock = FakeClock::new();
    let mut breaker = Breaker::new(clock.clone(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);
    trip(&mut breaker, &calls, 3).await;
    assert_eq!(breaker.state(), State::Open);

    clock.advance(Duration::from_secs(30));
    assert_eq!(request(&mut breaker, &calls, Ok(1)).await, Ok(1));
    assert_eq!(breaker.state(), State::Closed);

    assert_eq!(
        request(&mut breaker, &calls, Err("down")).await,
        Err(CallError::Inner("down"))
    );
    assert_eq!(breaker.state(), State::Closed);
}

#[tokio::test]
async fn half_open_failure_reopens_for_a_full_cool_down() {
    let clock = FakeClock::new();
    let mut breaker = Breaker::new(clock.clone(), config(3, Duration::from_secs(30)));
    let calls = Cell::new(0);
    trip(&mut breaker, &calls, 3).await;

    clock.advance(Duration::from_secs(30));
    assert_eq!(
        request(&mut breaker, &calls, Err("down")).await,
        Err(CallError::Inner("down"))
    );
    assert_eq!(breaker.state(), State::Open);

    clock.advance(Duration::from_millis(29_999));
    assert_eq!(
        request(&mut breaker, &calls, Ok(1)).await,
        Err(CallError::Open)
    );
    assert_eq!(calls.get(), 4);

    clock.advance(Duration::from_millis(1));
    assert_eq!(breaker.state(), State::HalfOpen);
}

#[tokio::test]
async fn outage_calls_are_bounded() {
    let clock = FakeClock::new();
    let mut breaker = Breaker::new(clock.clone(), config(5, Duration::from_secs(30)));
    let calls = Cell::new(0);

    for _ in 0..1000 {
        let _ = request(&mut breaker, &calls, Err("down")).await;
        clock.advance(Duration::from_millis(100));
    }
    assert_eq!(calls.get(), 8);
}
