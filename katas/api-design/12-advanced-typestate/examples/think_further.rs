//! 12 advanced: typestate: consume self
//!
//! Think further, from the kata README:
//! 1. A real `connect` can fail. What should it return, and what happens to the closed connection the caller passed in when it fails? Can they retry with it?
//! 2. An open connection goes out of scope without `close`. What runs? What can the type system force here, and what can't it? (Rust's types promise "used at most once", not "used exactly once".)
//! 3. Add transactions: `begin` and `commit`. Which methods should a connection inside a transaction have, and what should `close` do in the middle of one?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

use std::marker::PhantomData;

struct Closed;
struct Open;
struct Connection<State> {
    id: u32,
    state: PhantomData<State>,
}

#[derive(Debug)]
struct ConnectError;

impl Connection<Closed> {
    fn new(id: u32) -> Self {
        Connection {
            id,
            state: PhantomData,
        }
    }

    // Think further 1: a failed connect hands the closed connection back, so the caller can retry.
    fn connect(
        self,
        reachable: bool,
    ) -> Result<Connection<Open>, (Connection<Closed>, ConnectError)> {
        if reachable {
            Ok(Connection {
                id: self.id,
                state: PhantomData,
            })
        } else {
            Err((self, ConnectError))
        }
    }
}

impl Connection<Open> {
    fn send(&self, msg: &str) -> String {
        format!("conn {} sent {msg}", self.id)
    }

    // Think further 2: close consumes the connection, so it can't be used again afterward.
    fn close(self) -> Connection<Closed> {
        Connection {
            id: self.id,
            state: PhantomData,
        }
    }
}

fn main() {
    let conn = Connection::new(1);
    match conn.connect(false) {
        Ok(_) => println!("connected"),
        Err((closed, err)) => {
            println!("failed {err:?}, retrying with conn {}", closed.id);
            match closed.connect(true) {
                Ok(open) => {
                    println!("{}", open.send("hello"));
                    let _closed_again = open.close();
                }
                Err((_, err)) => println!("still failing: {err:?}"),
            }
        }
    }

    // Think further 3 would add begin/commit as more states; a Closed connection has no send method.
}
