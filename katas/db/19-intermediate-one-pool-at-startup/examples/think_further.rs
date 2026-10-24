//! 19 intermediate: one pool, made at startup
//!
//! Think further, from the kata README:
//! 1. `setup` allows one connection. If it allowed five, would every connection in the pool see the same in-memory database? Don't guess: find out how sqlx opens `sqlite::memory:` by reading `SqliteConnectOptions` in the sqlx-sqlite source.
//! 2. The pool's one connection is checked out by a task that then awaits a slow HTTP call before giving it back. What does the next `add_user` do, and for how long? Which `PoolOptions` setting decides?
//! 3. Two `add_user` calls must both succeed or both fail. The pool gives each call whichever connection is free. What do you need so both inserts run on the same connection?
//! 4. `PoolOptions` has `idle_timeout` and `max_lifetime`. What could they do to an in-memory database that lives only as long as its connections?

// Scratch file for the Think further items. Delete before the PR.
// Run with: cargo run --example think_further
// No sqlx here: a Mutex stands in for a pool of one connection, to show the shape of Think further 2 and 3.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Connection {
    rows: Vec<String>,
}

#[derive(Clone)]
struct Pool {
    conn: Arc<Mutex<Connection>>,
}

impl Pool {
    fn new() -> Self {
        Pool {
            conn: Arc::new(Mutex::new(Connection { rows: Vec::new() })),
        }
    }

    // Think further 2: while one caller holds the connection, every other add_user waits.
    fn add_user(&self, name: &str, slow: bool) {
        let mut conn = self.conn.lock().unwrap_or_else(|e| e.into_inner());
        if slow {
            thread::sleep(Duration::from_millis(50));
        }
        conn.rows.push(name.to_string());
    }
}

fn main() {
    let pool = Pool::new();

    // Think further 2: the slow caller holds the only connection, so the next call waits for it.
    let slow = {
        let pool = pool.clone();
        thread::spawn(move || pool.add_user("ada", true))
    };
    thread::sleep(Duration::from_millis(5));
    let started = std::time::Instant::now();
    pool.add_user("grace", false);
    println!("second add waited {} ms", started.elapsed().as_millis());
    slow.join().ok();

    // Think further 3: two inserts that must succeed or fail together need one connection for both.
    let mut conn = pool.conn.lock().unwrap_or_else(|e| e.into_inner());
    conn.rows.push("first".to_string());
    conn.rows.push("second".to_string());
    println!("rows written on one connection: {:?}", conn.rows);
}
