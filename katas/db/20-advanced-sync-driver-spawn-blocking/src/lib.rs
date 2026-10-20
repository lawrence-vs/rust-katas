use std::time::Duration;

/// Stands in for a synchronous database driver (rusqlite, diesel): it blocks the calling thread.
pub fn legacy_lookup(id: u32) -> String {
    std::thread::sleep(Duration::from_millis(100));
    format!("user-{id}")
}

/// Looks a user up from async code.
/// CLUE: A tokio worker runs many tasks on one thread. What do they do while this thread sleeps?
/// CLUE: Which tokio function moves blocking work onto a thread that is allowed to block?
pub async fn lookup(id: u32) -> String {
    legacy_lookup(id)
}
