# Distributed Key-Value Storage System

This project is a distributed key-value storage system written in Rust. It uses the libp2p library for peer-to-peer networking and a distributed hash table (DHT) for storing and retrieving file data.

---

## Features

1. **Node Discovery and Peer-to-Peer Networking**
   - Implements node discovery using the **mDNS protocol**, which automatically discovers peers in the local network and integrates them into the Kademlia Distributed Hash Table (DHT).
   - The Kademlia DHT enables efficient key-value operations, such as storing and retrieving records, and logs the success or failure of these operations for debugging and monitoring.

2. **File Chunking and Distribution**
   - Implements file storage by splitting large files into smaller chunks, which are individually stored in the DHT.
   - Metadata is stored to track the total number of chunks and their sequence. Chunks are retrieved in order, ensuring proper reassembly into the original file.

3. **Data Redundancy for Reliability**
   - Ensures reliability by maintaining data redundancy through quorum-based storage. Each record is replicated across multiple nodes in the network using a quorum size of `3` to tolerate potential node failures.


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
   cargo run --bin kv_storage
   ```
   Each node will initialize and start listening on a randomly assigned port.
### Step 2: Store a File
1. In one of the terminals, run:
    ```bash
    PUT -f <unique_file_name> <absolute_path_to_file>
    ```
    It is recommended to use a text file, eg. ```<unique_file_name>.txt``` for testing purpose. 
    Each file stored will use its unique filename when retrieved.
### Step 3: Retrieve the file
1. In one of the terminals, run:
    ```bash
    GET -f <unique_file_name> 
    ```
    The retrieved file will be written to current directory with name ```<unique_file_name>.txt```
### Step 4: Exiting the Program
1. To gracefully exit the program, run:
    ```bash
    EXIT
    ```

# for authentication:
first register
then get signature through:
cargo run --bin client -- --username <user1> --target_key <The_file_key_to_sign>
then get access permission through:
permission <file_key> <the_users_public_key>
then can call get or put through:
put/get -f <file_key> <the_users_public_key> <the_signature_of_this_user_on_the_file_key>

on another node:
need to get permission for this file_key and user again through:
permission <file_key> <the_users_public_key>
and then can call get through:
get -f <file_key> <the_users_public_key> <the_signature_of_this_user_on_the_file_key>
