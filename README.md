# Distributed Key-Value Storage System

This project is a distributed key-value storage system written in Rust. It uses the libp2p library for peer-to-peer networking and a distributed hash table (DHT) for storing and retrieving file data.

---

## Features

- **Distributed Text File Storage:** Store text files in the DHT across N servers.
- **File Retrieval:** Retrieve files stored in the DHT by their unique file names.
- **Peer-to-Peer Communication:** Nodes communicate using a libp2p-based network.

---

## Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install). 
2. Clone this repository:
   ```bash
   git clone <git@github.com:your_github_username/ECE1724-Rust-Project.git>
   cd <ECE1724-Rust-Project>
   git checkout kv_storage
    ```
## Running the Project
### Step 1: Start Nodes
1. Open four or more terminal windows. Each data is stored with three replicas, at least four servers must be running in the distributed system.
2. Run the program in each terminal to start nodes:
   ```bash
   cargo run
   ```
   Each node will initialize and start listening on a randomly assigned port.
### Step 2: Store a File
1. In one of the terminals, run:
    ```bash
    PUT -f <unique_file_name> <absolute_path_to_file>
    ```
    It is recommended to use a text file, eg. ``` unique_file_name.txt``` for testing purpose. 
    Each file stored will use its unique filename when retrieved.
### Step 3: Retrieve the file
1. In one of the terminals, run:
    ```bash
    GET -f <unique_file_name> 
    ```
    The retrieved file will be written to current directory with name ```unique_file_name.txt```
### Step 4: Exiting the Program
1. To gracefully exit the program, run:
    ```bash
    EXIT
    ```