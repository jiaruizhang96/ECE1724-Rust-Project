
## Modules and Their Responsibilities

### 1. `main.rs`
- The main entry point of the application that initializes and runs the modules for the project.

### 2. `network/`
- **`mod.rs`**: Declares the submodules for the network functionality.
- **`discovery.rs`**: Handles node discovery and the P2P setup using `libp2p` and Kademlia DHT.
- **`communication.rs`**: Contains functions for sending and receiving messages between nodes.
- **`handling.rs`**: Processes incoming network requests and routes them accordingly.

### 3. `hashing/`
- **`mod.rs`**: Declares the submodules for hashing.
- **`consistent.rs`**: Implements consistent hashing and node management functions for data distribution.

### 4. `storage/`
- **`mod.rs`**: Declares the submodules for storage.
- **`chunking.rs`**: Contains logic for chunking files into smaller pieces and distributing them across nodes.
- **`replication.rs`**: Manages data replication for fault tolerance and redundancy.
- **`retrieval.rs`**: Implements data retrieval and reconstruction from distributed chunks.

### 5. `client/`
- **`mod.rs`**: Declares the submodules for client.
- **`client.rs`**: Contains logic for client-side communication.

### 6. `utils.rs`
- Contains utility functions for common operations, such as logging network activity.

## Features To Implement
- **Peer-to-Peer Networking**: Establishes a decentralized network using `libp2p` with Kademlia DHT for efficient node discovery and data routing.
- **Consistent Hashing**: Distributes data evenly across nodes, supporting scalability and load balancing.
- **Data Storage and Replication**: Ensures data redundancy by replicating data chunks across multiple nodes for enhanced fault tolerance.
- **File Chunking and Distribution**: Splits large files into smaller chunks and distributes them to nodes, supporting parallel processing and data recovery.


### Running the Project
1. Clone this repository.
2. Run `cargo build` to compile the project.

### Running the Project
- Start the server by running `cargo run server`.
- Start the client by running `cargo run client`.
