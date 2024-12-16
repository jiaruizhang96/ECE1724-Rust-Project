mod node;
mod behaviour;
mod client;
use clap::{Arg, Command};
use async_std::io::{self, BufReader};
use async_std::prelude::*;
use futures::StreamExt;
use node::Node;
use hex;
use std::io::{Write};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Parse command-line arguments using Clap
    let matches = Command::new("ECEC1724 - Distributed File Storage System")
        .version("1.0")
        .about("A Distributed Hash Table (DHT) Storage System with Authentication")
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
                                println!("  register <username> [--admin]        - Register a new user");
                                println!("  put <key> <value> <pk> <sig>         - Store a key-value pair");
                                println!("  put -f <key> <file_path> <pk> <sig>  - Store a file");
                                println!("  get <key> <pk> <sig>                 - Retrieve a key-value pair");
                                println!("  get -f <key> <pk> <sig>              - Retrieve a file");
                                println!("  sign <username> <key>                - Sign a file");
                                println!("  permission <key> <pk>                - Permit access to a file");
                                println!("  listen <address>                     - Start listening on an address");
                                println!("  help                                 - Print this help message");
                                println!("  exit                                 - Exit the program gracefully");
                            }
                            ["register", username, flag] if *flag == "--admin" => {
                                match node.user_manager.register_user(username, true) {
                                    Ok(public_key) => {
                                        println!("Registered admin user: {}", username);
                                        println!("Public Key (hex): {}", hex::encode(&public_key));
                                    }
                                    Err(e) => println!("Registration failed: {}", e),
                                }
                            }
                            ["register", username] => {
                                match node.user_manager.register_user(username, false) {
                                    Ok(public_key) => {
                                        println!("Registered user: {}", username);
                                        println!("Public Key (hex): {}", hex::encode(&public_key));
                                    }
                                    Err(e) => println!("Registration failed: {}", e),
                                }
                            }
                            ["put", key, value, public_key, signature] => {
                                let pk_bytes = match hex::decode(public_key) {
                                    Ok(pk) => pk,
                                    Err(_) => {
                                        println!("Invalid public key format");
                                        continue;
                                    }
                                };
                                let sig_bytes = match hex::decode(signature) {
                                    Ok(sig) => sig,
                                    Err(_) => {
                                        println!("Invalid signature format");
                                        continue;
                                    }
                                };
                                
                                if node.put(key.to_string(), value.as_bytes().to_vec(), pk_bytes, sig_bytes) {
                                    println!("Successfully stored key-value pair");
                                } else {
                                    println!("Operation failed");
                                }
                            },
                            ["get", key, public_key, signature] => {
                                let pk_bytes = match hex::decode(public_key) {
                                    Ok(pk) => pk,
                                    Err(_) => {
                                        println!("Invalid public key format");
                                        continue;
                                    }
                                };
                                let sig_bytes = match hex::decode(signature) {
                                    Ok(sig) => sig,
                                    Err(_) => {
                                        println!("Invalid signature format");
                                        continue;
                                    }
                                };
                                
                                if node.get(key.to_string(), pk_bytes, sig_bytes) {
                                    println!("Retrieving key-value pair");
                                } else {
                                    println!("Retrieval failed");
                                }
                            },
                            ["put", "-f", key, file_path, public_key, signature] => {
                                let pk_bytes = match hex::decode(public_key) {
                                    Ok(pk) => pk,
                                    Err(_) => {
                                        println!("Invalid public key format");
                                        continue;
                                    }
                                };
                                let sig_bytes = match hex::decode(signature) {
                                    Ok(sig) => sig,
                                    Err(_) => {
                                        println!("Invalid signature format");
                                        continue;
                                    }
                                };
                                
                                if node.put_file(key.to_string(), file_path.to_string(), pk_bytes, sig_bytes) {
                                    println!("Successfully stored file: {}", file_path);
                                } else {
                                    println!("File storage operation failed");
                                }
                            },
                            ["get", "-f", key, public_key, signature] => {
                                let pk_bytes = match hex::decode(public_key) {
                                    Ok(pk) => pk,
                                    Err(_) => {
                                        println!("Invalid public key format");
                                        continue;
                                    }
                                };
                                let sig_bytes = match hex::decode(signature) {
                                    Ok(sig) => sig,
                                    Err(_) => {
                                        println!("Invalid signature format");
                                        continue;
                                    }
                                };
                                
                                if node.get_file(key.to_string(), pk_bytes, sig_bytes) {
                                    println!("Retrieving file with key: {}", key);
                                } else {
                                    println!("File retrieval failed");
                                }
                            },
                            ["permission", key, public_key] => {
                                let pk_bytes = match hex::decode(public_key) {
                                    Ok(pk) => pk,
                                    Err(_) => {
                                        println!("Invalid public key format");
                                        continue;
                                    }
                                };
                                
                                match node.user_manager.add_key_permission(key, &pk_bytes) {
                                    Ok(_) => println!("Permission granted for key: {}", key),
                                    Err(e) => println!("Failed to grant permission: {}", e),
                                }
                            },
                            ["sign", username, message] => {
                                match client::sign_message(username, message) {
                                    Ok((public_key, signature, _)) => {
                                        println!("Public Key (hex): {}", hex::encode(&public_key));
                                        println!("Signature (hex): {}", hex::encode(&signature));
                                    }
                                    Err(e) => {
                                        println!("Signing failed: {}", e);
                                    }
                                }
                            },
                            ["listen", addr] => {
                                node.start_listening(addr);
                                println!("Listening on: {}", addr);
                            },
                            ["exit"] => break,
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
