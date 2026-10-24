//! A one-shot fill confirmation: one task awaits `Confirmed`, another calls
//! `Confirmer::confirm`. Std only; tokio drives the tests.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::task::{Context, Poll, Waker};

struct Shared {
    fill_id: Option<u64>,
    waker: Option<Waker>,
}

fn lock(shared: &Mutex<Shared>) -> MutexGuard<'_, Shared> {
    shared.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Makes a connected pair: complete one side, await the other.
pub fn confirmation() -> (Confirmer, Confirmed) {
    let shared = Arc::new(Mutex::new(Shared {
        fill_id: None,
        waker: None,
    }));
    (
        Confirmer {
            shared: Arc::clone(&shared),
        },
        Confirmed { shared },
    )
}

pub struct Confirmer {
    shared: Arc<Mutex<Shared>>,
}

impl Confirmer {
    /// Completes the confirmation; consumes self (one-shot).
    ///
    /// CLUE: What does `confirm` look for before it can tell anyone?
    /// CLUE: Next time, this future may be polled from a different task. Whose waker should `confirm` wake?
    pub fn confirm(self, fill_id: u64) {
        let waker = {
            let mut s = lock(&self.shared);
            s.fill_id = Some(fill_id);
            s.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

pub struct Confirmed {
    shared: Arc<Mutex<Shared>>,
}

impl Future for Confirmed {
    type Output = u64;

    /// CLUE: When `poll` returns `Pending`, who is going to call it again?
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u64> {
        let s = lock(&self.shared);
        match s.fill_id {
            Some(id) => Poll::Ready(id),
            None => Poll::Pending,
        }
    }
}
