use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    // websocket server address
    let url = "ws://127.0.0.1:8081/chat";

    // establish websocket connection
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to server at {}", url);

    // split the websocket stream into a sender and receiver
    let (writer, mut reader) = ws_stream.split();

    // share the writer across tasks
    let writer = Arc::new(Mutex::new(writer));
    let writer_clone = Arc::clone(&writer);

    // Channel to notify when the connection is closed
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    // Task for handling incoming messages
    let shutdown_tx_clone = shutdown_tx.clone();

    // task for handling incoming messages
    task::spawn(async move {
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("Server: {}", text),
                Ok(Message::Close(_)) => {
                    println!("Connection closed by server.");
                    let _ = shutdown_tx_clone.send(()).await; // Notify shutdown
                    break;
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    let _ = shutdown_tx_clone.send(()).await; // Notify shutdown
                    break;
                }
                _ => (),
            }
        }
    });

    // handle user input and send messages
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        io::stdout().flush().await.expect("Failed to flush stdout"); // Flush stdout to ensure the prompt appears immediately
        tokio::select! {
            // Check if the connection is closed
            _ = shutdown_rx.recv() => {
                println!("Shutting down client input loop.");
                break;
            }

            // Process user input
            result=lines.next_line()=>{
                match result{
                    Ok(Some(line))=>{
                        let input = line.trim();

                        // command parsing
                        if input.is_empty() {
                        continue; // ignore empty input
                        }
                        // send the command as a WebSocket message
                        let mut writer = writer_clone.lock().await;
                        if let Err(e) = writer.send(Message::Text(input.to_string())).await {
                            eprintln!("Error sending message: {}", e);
                            break;
                        }
                    }
                    Ok(None)=>break, //End of input
                    Err(e)=>{
                        eprintln!("Error reading input: {}",e);
                        break;
                    }
                }
            }
        }
    }
    println!("Client closed.");
}
