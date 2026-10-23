//! 24 advanced: split reads from writes
//!
//! Think further, from the kata README:
//! 1. A client pulls its network cable: no Close frame, no FIN. The room is quiet. How long until your `handle` notices, and which half notices first?
//! 2. A client sends a Ping. tungstenite queues the Pong for you, but it only goes out when that connection is next written to or flushed. With the halves in different tasks, when does that happen on a quiet room?
//! 3. The talker sends "hi". Does it get its own message back? Should it, and whose job is it to decide?
//! 4. 10,000 clients: two tasks per client or one `select!` per client. What does each cost, and is `StreamExt::next` safe to drop half-way through a frame?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further
// Channels stand in for the two halves of a websocket, so the shape of the split is visible.

use tokio::sync::mpsc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // One channel per direction: the read half pushes incoming frames, the write half drains replies.
    let (to_client, mut from_server) = mpsc::channel::<String>(8);
    let (to_server, mut outgoing) = mpsc::channel::<String>(8);

    // The writer task runs on its own, so a slow read never stops replies going out.
    let writer = tokio::spawn(async move {
        while let Some(msg) = outgoing.recv().await {
            println!("writer sends: {msg}");
        }
    });

    // Think further 3: a message from one talker goes to the room, not echoed back to that talker.
    to_client.send("hi".to_string()).await.ok();
    if let Some(frame) = from_server.recv().await {
        println!("reader got: {frame}");
        to_server.send(format!("ack {frame}")).await.ok();
    }

    // Think further 1: when the peer vanishes, the read side only notices after a read or a timeout.
    drop(to_client);
    println!("read side sees end: {}", from_server.recv().await.is_none());

    drop(to_server);
    writer.await.ok();
}
