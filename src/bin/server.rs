use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    // TODO: For a hint, see the description of the task below.
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                if let Some(Ok(msg)) = incoming {
                    if msg.is_text() {
                        let text = msg.as_text().unwrap();
                        println!("From client {addr:?} {text:?}");
                        let _ = bcast_tx.send(text.to_string());
                    }
                } else {
                    break;
                }
            }
            msg = bcast_rx.recv() => {
                if let Ok(text) = msg {
                    ws_stream.send(Message::text(text)).await?;
                } else {
                    break;
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await.unwrap();

            handle_connection(addr, ws_stream, bcast_tx).await.unwrap();
        });
    }
}