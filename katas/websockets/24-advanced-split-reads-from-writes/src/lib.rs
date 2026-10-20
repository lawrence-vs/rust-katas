use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

/// A chat room: every text a client sends goes to every client.
pub async fn serve(listener: TcpListener) {
    let (room, _) = broadcast::channel::<String>(64);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle(stream, room.clone()));
    }
}

/// CLUE: This loop waits for the client to speak before it delivers anything. What does a client that only listens get?
/// CLUE: What does `StreamExt::split` give you, and could each half run on its own?
async fn handle(stream: TcpStream, room: broadcast::Sender<String>) {
    let Ok(mut ws) = tokio_tungstenite::accept_async(stream).await else {
        return;
    };
    let mut inbox = room.subscribe();
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => {
                let _ = room.send(text.to_string());
            }
            Some(Ok(_)) => continue,
            _ => return,
        }
        while let Ok(msg) = inbox.try_recv() {
            if ws.send(Message::text(msg)).await.is_err() {
                return;
            }
        }
    }
}
