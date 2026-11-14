use std::sync::atomic::{AtomicBool, Ordering};

/// A client session with a gateway.
#[derive(Debug, Default)]
pub struct Session {
    started: AtomicBool,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }

    /// Marks the session as started.
    /// CLUE: Calling an `async fn` gives you a value. When does the code inside it run?
    async fn start_session(&self) {
        self.started.store(true, Ordering::SeqCst);
    }

    /// Starts the session.
    /// CLUE: Build this crate and read every warning. What does the compiler say about this line?
    /// CLUE: Who polls the value made on this line, and when is it dropped?
    pub async fn run(&self) {
        self.start_session();
    }
}
