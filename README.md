# DoS_Competency_Test

P2P Node with Tower Service
This project implements a simple P2P (peer-to-peer) network using libp2p and a Tower service to handle request/response (REQ/REP) messages. The goal is to demonstrate communication between peers using a custom protocol, avoiding libp2p's default gossip protocol.

Features
libp2p for peer-to-peer communication

Tower service to handle incoming REQ/REP messages

Request/Response protocol to exchange messages between peers

mDNS (Multicast DNS) for discovering peers on the network

QUIC transport for secure and fast communication

External address handling to manage peer-to-peer network visibility

Project Structure
src/
├── main.rs         # Entry point with swarm event loop
├── service.rs      # Tower service handling greet messages
├── protocol.rs     # Request/response types

Dependencies
libp2p: Peer-to-peer networking library

futures: For working with async tasks and streams

tokio: Asynchronous runtime for Rust

serde: Serialization and deserialization of messages

tower: A library for building services and middleware

tracing-subscriber: For logging and monitoring


Setup
1)Clone the repository:
git clone  https://github.com/R27-pixel/DoS_Competency_Test.git
cd p2p-node


2)Add dependencies: The Cargo.toml file includes the necessary dependencies for building the P2P node and Tower service.

3)Build the project:
cargo build

4)Run the P2P node:
In terminal
cargo run

Output
![Screenshot 2025-04-19 002120](https://github.com/user-attachments/assets/0950f7f4-8cf0-4a09-a15b-4abc251ad871)

In second Terminal
cargo run

Output
![Screenshot 2025-04-19 002157](https://github.com/user-attachments/assets/9e2193d5-4721-4b53-90aa-db99b536d2f6)

Usage
Once the node is running, it will:

1)Discover peers via mDNS.

2)Establish connections with discovered peers.

3)Exchange GreetRequest and GreetResponse messages.

4)Respond to incoming requests

