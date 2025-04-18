# P2P Node with Tower Service

A robust peer-to-peer networking implementation using libp2p with a Tower service architecture to handle request/response communication patterns.

## Overview

This project demonstrates a clean implementation of a peer-to-peer network that uses a custom protocol for direct communication between nodes rather than libp2p's default gossip protocol. The implementation leverages Tower services for handling asynchronous request/response patterns in a structured manner.

## Features

- **libp2p Foundation**: Built on the libp2p networking stack for peer discovery and connectivity
- **Tower Service Architecture**: Implements a Tower service to handle incoming requests and responses
- **Custom Protocol**: Uses a request/response protocol for direct peer communication
- **Automatic Peer Discovery**: Utilizes mDNS for peer discovery on local networks
- **QUIC Transport**: Features secure and efficient communication using the QUIC protocol
- **External Address Management**: Handles external networking addresses for proper peer visibility

## Architecture

The project is organized into three main components:

- **Main Runtime**: Manages the libp2p swarm and event loop
- **Service Handler**: Implements Tower service functionality for processing messages
- **Protocol Definition**: Defines custom request/response types and serialization

## Project Structure

```
src/
  ├── main.rs         # Entry point and swarm event handler
  ├── service.rs      # Tower service implementation for message handling
  ├── protocol.rs     # Request/response protocol types and serialization
```

## Dependencies

- **libp2p**: Core peer-to-peer networking capabilities
- **tower**: Service abstraction for building network applications
- **tokio**: Asynchronous runtime for Rust
- **futures**: Tools for working with asynchronous code
- **serde**: Serialization/deserialization framework
- **tracing-subscriber**: Logging and monitoring functionality

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/R27-pixel/DoS_Competency_Test.git
   cd p2p-node
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Running the Node

To start a P2P node:

```bash
cargo run
```

For testing peer-to-peer functionality, open a second terminal and run another instance:

```bash
cargo run
```

## How It Works

1. **Node Initialization**: Each node starts up and configures its libp2p swarm
2. **Peer Discovery**: mDNS discovers other peers on the local network
3. **Connection Establishment**: Nodes establish connections with discovered peers
4. **Message Exchange**: Nodes exchange GreetRequest and GreetResponse messages using the custom protocol
5. **Service Handling**: The Tower service processes incoming requests and generates appropriate responses

## Example Output

When running two nodes on the same machine, you'll see them discover each other and exchange greeting messages:

First Terminal

![Node 1 Output](https://github.com/user-attachments/assets/0950f7f4-8cf0-4a09-a15b-4abc251ad871)

Second Terminal

![Node 2 Output](https://github.com/user-attachments/assets/9e2193d5-4721-4b53-90aa-db99b536d2f6)

## Technical Details

### Network Components

- **Transport**: QUIC protocol for secure and efficient packet delivery
- **Discovery**: mDNS for automatic peer detection on local networks
- **Message Format**: Custom protocol messages serialized with serde
- **Service Architecture**: Tower request/response pattern for handling communication

### Message Types

- **GreetRequest**: Initial message sent from one peer to another
- **GreetResponse**: Response returned when a greeting is received
