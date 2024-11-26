use libp2p::{
    PeerId, Swarm,
    kad::{Kademlia, record::{Key, Record, store::MemoryStore}, Quorum},
    mdns::Mdns,
    development_transport, identity,
};
use crate::behaviour::Behaviour;
use async_std::{io, task};
use futures::FutureExt;
use async_std::io::BufReadExt;
//use libp2p::NetworkBehaviour;
use futures::StreamExt;
use futures::Stream;
pub struct Node {
    pub peer_id: PeerId,
    pub swarm: Swarm<Behaviour>, // The main swarm managing networking
}

impl Node {
    /// Create a new node
    pub async fn new() -> Self {
        // Generate identity
        let keypair = identity::Keypair::generate_secp256k1();
        let peer_id = PeerId::from(keypair.public());
        println!("Generated PeerId: {:?}", peer_id);

        // Set up transport
        let transport = development_transport(keypair.clone())
            .await
            .expect("Failed to create transport");

        // Create Kademlia
        let store = MemoryStore::new(peer_id.clone());
        let kademlia = Kademlia::new(peer_id.clone(), store);

        // Set up mDNS： used to find nodes on the same network
        let mdns = Mdns::new(Default::default())
            .await
            .expect("Failed to initialize mDNS");

        // Combine behaviours
        // in behaviour.rs, struct Behaviour has 2 attributes
        let behaviour = Behaviour { kademlia, mdns };

        // Create swarm
        let swarm = Swarm::new(transport, behaviour, peer_id.clone());

        Node { peer_id, swarm }
    }
    /// Start listening on a specified address
    pub fn start_listening(&mut self, addr: &str) {
        self.swarm
            .listen_on(addr.parse().expect("Invalid multiaddr"))
            .expect("Failed to start listening");
        println!("Node is listening on {:?}", addr);
    }
    
    /// Store a key-value pair in the DHT
    pub fn put(&mut self, key: String, value: Vec<u8>) {
        let record = Record {
            key: Key::new(&key),
            value,
            publisher: None,
            expires: None,
        };

        self.swarm
            .behaviour_mut()
            .kademlia
            .put_record(record, Quorum::One)
            .expect("Failed to store record");
    }

    /// Retrieve a value for a given key from the DHT
    pub fn get(&mut self, key: String) {
        let key = libp2p::kad::record::Key::new(&key);
        // triggers Kademlia to initiate a GetRecord or PutRecord query
        // see behaviour.rs, OutboundQueryCompleted 
        // Quorum is an enum in the libp2p::kad module that 
        // defines the minimum number of peers that must respond to a query for it to succeed.
        // for now it's 1, only 1 peer needs to return the record for the query to succeed.
        // in case we tested the system with less than 2 peers. 
        self.swarm
            .behaviour_mut()
            .kademlia
            .get_record(&key, libp2p::kad::Quorum::One);
    }

    /// Handle user input commands
    pub async fn handle_input(&mut self, line: String) {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        match parts.as_slice() {
            // Command to store a key-value pair
            ["PUT", key, value] => {
                self.put(key.to_string(), value.as_bytes().to_vec());
                println!("Stored record with key: {}", key);
            }

            // Command to retrieve a value by key
            ["GET", key] => {
                self.get(key.to_string());
                println!("Searching for key: {}", key);
            }

            // Command to start listening on a new address
            ["LISTEN", addr] => {
                self.start_listening(addr);
                println!("Started listening on {}", addr);
            }

            // Exit command for gracefully stopping the program
            ["EXIT"] => {
                println!("Exit command received. Stopping...");
                std::process::exit(0);
            }

            // Unknown or invalid command
            _ => {
                println!(
                    "Unknown command. Use one of the following:\n\
                    PUT <key> <value> - Store a key-value pair in the DHT\n\
                    GET <key> - Retrieve a value from the DHT\n\
                    LISTEN <address> - Start listening on a specified address\n\
                    EXIT - Exit the program"
                );
            }
        }
    }
    
}
