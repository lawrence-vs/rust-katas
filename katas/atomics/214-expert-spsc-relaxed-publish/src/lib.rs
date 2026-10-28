use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A single-producer, single-consumer ring buffer. Usable capacity is `cap - 1`.
pub struct Spsc<T> {
    slots: Box<[UnsafeCell<Option<T>>]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

// SAFETY: one thread pushes and one thread pops; each slot is handed over through `head`/`tail`.
unsafe impl<T: Send> Sync for Spsc<T> {}

impl<T> Spsc<T> {
    pub fn with_capacity(cap: usize) -> Self {
        assert!(cap >= 2, "need at least one usable slot");
        let slots = (0..cap).map(|_| UnsafeCell::new(None)).collect();
        Self {
            slots,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Producer only. Returns the value back if the buffer is full.
    /// CLUE: The consumer reads `slots[head]` after it sees `tail` move. What must `tail`'s store guarantee?
    pub fn push(&self, value: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next = (tail + 1) % self.slots.len();
        if next == self.head.load(Ordering::Acquire) {
            return Err(value);
        }
        unsafe { *self.slots[tail].get() = Some(value) };
        self.tail.store(next, Ordering::Relaxed);
        Ok(())
    }

    /// Consumer only. Returns `None` when empty.
    pub fn pop(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }
        let value = unsafe { (*self.slots[head].get()).take() };
        self.head
            .store((head + 1) % self.slots.len(), Ordering::Release);
        value
    }
}
