use std::io;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Lines answered so far, across every connection.
#[derive(Debug, Default)]
struct Served(AtomicU64);

impl Served {
    fn next(&self) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed) + 1
    }
}

/// Accepts clients forever. Each sends one line and gets back `#<n> <line>\n`.
/// CLUE: While `handle` waits for the first client's line, who is calling `accept`?
/// CLUE: `tokio::spawn` needs a `'static` future. Does a future that holds `&served` qualify?
pub async fn serve(listener: TcpListener) -> io::Result<()> {
    let served = Served::default();
    loop {
        let (socket, _) = listener.accept().await?;
        handle(socket, &served).await?;
    }
}

/// CLUE: If each connection had its own `Served`, what number would every client see?
/// CLUE: One client resets mid-reply. Who sees this error, and should the whole server stop?
async fn handle(socket: TcpStream, served: &Served) -> io::Result<()> {
    let mut conn = BufReader::new(socket);
    let mut line = String::new();
    if conn.read_line(&mut line).await? == 0 {
        return Ok(());
    }
    let n = served.next();
    conn.get_mut()
        .write_all(format!("#{n} {}\n", line.trim_end()).as_bytes())
        .await
}
