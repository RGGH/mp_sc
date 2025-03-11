use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use uuid::Uuid;

type Tx = mpsc::UnboundedSender<String>;
type Clients = Arc<Mutex<HashMap<Uuid, Tx>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("WebSocket server running on ws://127.0.0.1:8080");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients = clients.clone();
        tokio::spawn(handle_connection(stream, clients));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, clients: Clients) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {:?}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let client_id = Uuid::new_v4(); // Unique identifier

    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.insert(client_id, tx.clone()); // ✅ Use UUID as key
    }

    println!("New client connected: {}", client_id);

    let clients_read = clients.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Ok(text) = msg.to_text() {
                println!("Received: {}", text);
                let clients_lock = clients_read.lock().unwrap();
                for (_, client) in clients_lock.iter() {
                    let _ = client.send(text.to_string());
                }
            }
        }

        let mut clients_lock = clients.lock().unwrap();
        clients_lock.remove(&client_id); // ✅ Now removing works
        println!("Client {} disconnected", client_id);
    });

    while let Some(msg) = rx.recv().await {
        if let Err(e) = write.send(Message::Text(msg)).await {
            eprintln!("Failed to send message: {:?}", e);
            break;
        }
    }
}

