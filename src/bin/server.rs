use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use tokio::sync::broadcast::error::RecvError;

async fn handle_connection(
    addr: SocketAddr,
    ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut write, mut read) = ws_stream.split();
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive messages from the client and broadcast them
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(text) = msg.as_text() {
                                println!("From client: {}", text);
                                if let Err(e) = bcast_tx.send(text.to_string()) {
                                    eprintln!("Error broadcasting message: {}", e);
                                    break;
                                }
                            }
                        } else if msg.is_close() {
                            println!("Client {addr:?} disconnected");
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message from {addr:?}: {e}");
                        break;
                    }
                    None => {
                        eprintln!("Client {addr:?} disconnected");
                        break;
                    }
                }
            }

            // Receive messages from the broadcast channel and send them to the client
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        write.send(Message::text(msg)).await?;
                    }
                    Err(RecvError::Closed) => {
                        break;
                    }
                    Err(RecvError::Lagged(_)) => {
                        eprintln!("Houston we have a lagging problem")
                    }
                }
            }
        }
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}