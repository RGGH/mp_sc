# WebSocket Server in Rust


## Overview
This project is a simple WebSocket server implemented in Rust using `tokio`, `tokio-tungstenite`, and `futures-util`. It supports multiple clients and broadcasts messages to all connected clients.

## Features
- Handles multiple WebSocket connections concurrently
- Assigns a unique `UUID` to each client
- Broadcasts received messages to all connected clients
- Automatically removes disconnected clients

## Requirements
Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Dependencies
The project uses the following dependencies:

```toml
[dependencies]
futures-util = "0.3"
tokio = { version = "1", features = ["full"] }
tokio-tungstenite = "0.20"
uuid = { version = "1", features = ["v4"] }
```

## Installation & Usage
Clone the repository and navigate into the project folder:

```sh
git clone https://github.com/yourusername/websocket-server-rust.git
cd websocket-server-rust
```

Build and run the server:

```sh
cargo run
```

The WebSocket server will start at `ws://127.0.0.1:8080`.

## Testing with `wscat`
To test the WebSocket server, you can use `wscat`, a simple WebSocket client. Install it using:

```sh
npm install -g wscat
```

Then, connect to the WebSocket server:

```sh
wscat -c ws://127.0.0.1:8080
```

You can now send messages, and they will be broadcasted to all connected clients.

## Code Explanation
The main logic is structured as follows:

1. **Listening for Connections**
   - A `TcpListener` listens on port `8080` for incoming WebSocket connections.

2. **Handling WebSocket Connections**
   - Each client gets a unique `UUID` upon connecting.
   - Messages from a client are broadcasted to all other connected clients.
   - Disconnected clients are removed from the client list.

## License
This project is licensed under the MIT License.

## Contributing
Feel free to submit issues and pull requests to improve the project!

![Screenshot from 2025-03-11 23-16-33](https://github.com/user-attachments/assets/a8d5687e-967f-4d33-a39c-5fb65a268331)


---
Enjoy building with Rust! 🚀


