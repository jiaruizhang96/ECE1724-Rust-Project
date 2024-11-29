mod node;
mod behaviour;
use clap::{Arg, Command};
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use futures::StreamExt;
use node::Node;
use std::io::{Write};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Parse command-line arguments using Clap
    let matches = Command::new("ECEC1724 - Distributed File Storage System")
        .version("1.0")
        .about("A Distributed Hash Table (DHT) Storage System")
        .arg(
            Arg::new("listen")
                .short('l')
                .long("listen")
                .value_name("ADDRESS")
                .help("Start listening on a specific address")
                .default_value("/ip4/0.0.0.0/tcp/0")
        )
        .get_matches();

    // Create a new node
    let mut node = Node::new().await;

    // Start listening on the specified or default address
    let listen_addr = matches.get_one::<String>("listen").unwrap();
    node.start_listening(listen_addr);

    println!("Distributed Hash Table (DHT) Storage System");
    println!("Node initialized with PeerId: {}", node.peer_id);
    println!("Type 'help' for available commands");

    // Set up async stdin reader and fuse it
    let mut stdin = BufReader::new(io::stdin()).lines().fuse();

    // Main event loop
    loop {
        print!("p2p> ");
        std::io::stdout().flush()?; // make sure the prompt is displayed immediately

        futures::select! {
            // Handle user input
            line = stdin.next() => {
                match line {
                    Some(Ok(input)) => {
                        // Trim and split input
                        let parts: Vec<&str> = input.trim().split_whitespace().collect();
                        
                        // Process command
                        match parts.as_slice() {
                            ["help"] => {
                                println!("Available commands:");
                                println!("  put <key> <value>     - Store a key-value pair");
                                println!("  put -f <key> <file>   - Store a file");
                                println!("  get <key>             - Retrieve a key-value pair");
                                println!("  get -f <key>          - Retrieve a file");
                                println!("  listen <address>      - Start listening on an address");
                                println!("  help                  - Print this help message");
                                println!("  exit                  - Exit the program gracefully");
                            }
                            ["exit"] => break,
                            ["put", key, value] => {
                                node.put(key.to_string(), value.as_bytes().to_vec());
                                println!("Stored key-value pair: {} = {}", key, value);
                            },
                            ["put", "-f", key, file_path] => {
                                node.put_file(key.to_string(), file_path.to_string());
                                println!("Stored text file '{}' with unique key: {}", file_path, key);
                            },
                            ["get", key] => {
                                node.get(key.to_string());
                                println!("Searching for key: {}", key);
                            },
                            ["get", "-f", key] => {
                                node.get_file(key.to_string());
                                println!("Searching for text file with unique key: {}", key);
                            },
                            ["listen", addr] => {
                                node.start_listening(addr);
                                println!("Listening on: {}", addr);
                            },
                            _ => {
                                println!("Invalid command. Type 'help' for available commands.");
                            }
                        }
                    },
                    Some(Err(e)) => {
                        eprintln!("Error reading input: {:?}", e);
                        break;
                    },
                    None => {
                        println!("Stdin closed. Exiting...");
                        break;
                    }
                }
            },

            // Handle swarm events
            /*
            swarm events include 1) network-level 2) behaviour-specific
            1) network-level
                nodes listen on a new address, to know where peers can connect to your node
                connection closed, established
            2) behaviour-specific
                eg. Kademlia, mDNS, SwarmEvent::Behaviour(event)
                1. When server call put_record or get_record to process client requests, 
                    a Kademlia query is initiated internally.
                2. During the query, the progress and results are wrapped as KademliaEvent variants.
                3. These KademliaEvent variants are emitted as part of SwarmEvent::Behaviour.
                4. in main.rs, the events are propagated as SwarmEvent::Behaviour and caught by this loop 
            */
            event = node.swarm.next() => {
                match event {
                    Some(event) => match event {
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on: {:?}", address);
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("Connected to peer: {:?}", peer_id);
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                            println!("Disconnected from peer: {:?}, cause: {:?}", peer_id, cause);
                        }
                        _ => {}
                    },
                    None => {
                        println!("Swarm stopped. Exiting...");
                        break;
                    }
                }
            }
        }
    }

    println!("Exiting the File Storage System");
    Ok(())
}
