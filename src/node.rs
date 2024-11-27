use libp2p::{
    PeerId, Swarm,
    kad::{Kademlia, record::{Key, Record, store::MemoryStore}, Quorum},
    mdns::Mdns,
    development_transport, identity,
};
use crate::behaviour::Behaviour;
use std::num::NonZeroUsize;

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
        let behaviour = Behaviour { kademlia, mdns};

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
        let quorum = Quorum::N(NonZeroUsize::new(3).expect("Quorum value must be non-zero"));

        self.swarm
            .behaviour_mut()
            .kademlia
            .put_record(record.clone(), quorum)
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

    /// Store a file in the DHT by splitting it into chunks and storing each chunk separately
    pub fn put_file(&mut self, file_key: String, file_path: String) {
        use std::fs;

        // Read the file content
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read file '{}': {:?}", file_path, e);
                return;
            }
        };

        // Split the file content into 500-character chunks
        let chunks: Vec<String> = file_content
            .chars()
            .collect::<Vec<char>>()
            .chunks(500)
            .map(|chunk| chunk.iter().collect())
            .collect();

        let total_chunks = chunks.len();
        

        // Store each chunk in the DHT
        for (counter, chunk) in chunks.iter().enumerate() {
            let chunk_key = format!("{}_{}_{}", file_key, counter, total_chunks);
            let record = Record {
                key: Key::new(&chunk_key),
                value: chunk.as_bytes().to_vec(),
                publisher: None,
                expires: None,
            };

            self.swarm
                .behaviour_mut()
                .kademlia
                .put_record(record, libp2p::kad::Quorum::One)
                .expect("Failed to store record");
            println!(
                "Stored chunk {} of {} for file '{}' under key '{}'.",
                counter + 1,
                total_chunks,
                file_key,
                chunk_key
            );
        }

        println!(
            "Stored file '{}' in {} chunks. Each chunk stored with keys '<file_key>_<chunk_number>_<total_chunks>'.",
            file_path, total_chunks
        );
    let total_chunks_key = format!("{}_total", file_key);
    let record = Record {
        key: Key::new(&total_chunks_key),
        value: total_chunks.to_string().as_bytes().to_vec(),
        publisher: None,
        expires: None,
    };

    self.swarm
        .behaviour_mut()
        .kademlia
        .put_record(record, libp2p::kad::Quorum::One)
        .expect("Failed to store total chunks metadata");

    println!(
        "Stored file '{}' in {} chunks. Each chunk stored with keys '<file_key>_<chunk_number>_<total_chunks>'. Total chunks metadata stored under key '{}'.",
        file_path, total_chunks, total_chunks_key
    );
    }

    /// Retrieve a file from the DHT by reconstructing it from its chunks
    pub fn get_file(&mut self, file_key: String) {
    
        // Initiate retrieval 
        let chunk_key = format!("{}_total", file_key,); 
        let chunk_key = libp2p::kad::record::Key::new(&chunk_key);
    
        self.swarm
            .behaviour_mut()
            .kademlia
            .get_record(&chunk_key, libp2p::kad::Quorum::One);
    
        println!(
            "Initiated retrieval for file '{}'",
            file_key
        );
    }
    
    /// Handle user input commands
    pub async fn handle_input(&mut self, line: String) {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        match parts.as_slice() {
            // Command to store a key-value pair (PUT -k)
            ["PUT", "-k", key, value] => {
                self.put(key.to_string(), value.as_bytes().to_vec());
                println!("Stored record with key: {}", key);
            }

            // Command to store a text file (PUT -f)
            ["PUT", "-f", unique_txt_file_key, txt_file_path] => {
                self.put_file(unique_txt_file_key.to_string(), txt_file_path.to_string());
                println!("Stored text file with unique key: {}", unique_txt_file_key);
            }

            // Command to retrieve a value by key (GET -k)
            ["GET", "-k", key] => {
                self.get(key.to_string());
                println!("Searching for key: {}", key);
            }

            // Command to retrieve a text file (GET -f)
            ["GET", "-f", unique_txt_file_key] => {
                self.get_file(unique_txt_file_key.to_string());
                println!("Searching for text file with unique key: {}", unique_txt_file_key);
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
                    PUT -f <unique_txt_file_key> <txt_file_path> - Store a text file in the DHT\n\
                    GET -f <unique_txt_file_key> - Retrieve a text file from the DHT\n\
                    LISTEN <address> - Start listening on a specified address\n\
                    EXIT - Exit the program"
                );
            }
        }
    }

    
}
