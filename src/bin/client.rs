use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            // Read user messages from standard input and send them to the server
            line = stdin.next_line() => {
                if let Some(line) = line? {
                    write.send(Message::text(line)).await?;
                }
            }

            // Receive messages from the server and display them
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        // Display the message received from the server
                        if let Some(text) = msg.as_text() {
                            println!("Galih's Computer - From server: {}", text);
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message from server: {}", e);
                        break;
                    }
                    None => {
                        // Server closed the connection, terminate the loop
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}