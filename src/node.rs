//use crate::store::Store; // Import Store from store.rs
use libp2p::{kad::{store::MemoryStore, Kademlia}};
use libp2p::swarm::{Swarm};
use libp2p::{identity, Multiaddr, PeerId, development_transport};
use futures::prelude::*;
use std::error::Error;
//use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//use libp2p::PeerId;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};

const MAX_STORAGE: usize = 100_000_000; // 0.1GB max storage

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a storage node in the network
pub struct Store {
    #[serde_as(as = "DisplayFromStr")]
    /// Unique PeerId of the storage node
    peer_id: PeerId,

    /// Maps file names to their absolute paths on disk
    stored_files: HashMap<String, PathBuf>,

    /// Remaining available storage space in bytes
    available_space: usize,

    /// Directory on disk where files are stored
    storage_directory: PathBuf,
}

impl Store {
    /// Creates a new `Store` instance with a dedicated storage directory
    pub fn new(peer_id: PeerId) -> Self {
        // Use the current file's directory and append the peer_id as the directory name
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let storage_directory = current_dir.join(peer_id.to_string());
    
        // Create the storage directory if it doesn't exist
        if !storage_directory.exists() {
            fs::create_dir_all(&storage_directory).expect("Failed to create storage directory");
        }
    
        Self {
            peer_id,
            stored_files: HashMap::new(),
            available_space: MAX_STORAGE,
            storage_directory,
        }
    }

    /// Returns the PeerId of this storage node
    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    /// Calculates and returns the amount of used storage space
    pub fn used_space(&self) -> usize {
        MAX_STORAGE - self.available_space
    }

    /// Returns the total storage capacity of the node
    pub fn total_space(&self) -> usize {
        MAX_STORAGE
    }

    /// Stores a file on the disk in the storage directory
    ///
    /// # Arguments
    /// * `filename` - The name of the file to be stored
    /// * `data` - The file data as a byte vector
    ///
    /// # Returns
    /// * `Ok(())` if the file is successfully stored
    /// * `Err(&'static str)` if there isn't enough space
    pub fn store_file(&mut self, filename: String, data: Vec<u8>) -> Result<(), &'static str> {
        if data.len() > self.available_space {
            return Err("Not enough space to store the file");
        }

        // Determine the full path for the file
        let file_path = self.storage_directory.join(&filename);

        // Write the file to disk
        let mut file = fs::File::create(&file_path).map_err(|_| "Failed to create file on disk")?;
        file.write_all(&data).map_err(|_| "Failed to write data to file on disk")?;

        // Update the stored_files map and available space
        self.stored_files.insert(filename, file_path);
        self.available_space -= data.len();
        Ok(())
    }

    /// Retrieves a file's data from disk by its name
    ///
    /// # Arguments
    /// * `filename` - The name of the file to retrieve
    ///
    /// # Returns
    /// * `Option<Vec<u8>>` containing the file data if it exists
    pub fn get_file(&self, filename: &str) -> Option<Vec<u8>> {
        self.stored_files.get(filename).and_then(|file_path| {
            let mut file = fs::File::open(file_path).ok()?;
            let mut data = Vec::new();
            file.read_to_end(&mut data).ok()?;
            Some(data)
        })
    }

    /// Removes a file from the disk and updates the store
    ///
    /// # Arguments
    /// * `filename` - The name of the file to remove
    ///
    /// # Returns
    /// * `Ok(())` if the file is successfully removed
    /// * `Err(&'static str)` if the file does not exist or cannot be removed
    pub fn remove_file(&mut self, filename: &str) -> Result<(), &'static str> {
        if let Some(file_path) = self.stored_files.remove(filename) {
            fs::remove_file(file_path).map_err(|_| "Failed to remove file from disk")?;
            Ok(())
        } else {
            Err("File not found")
        }
    }

    /// Returns the currently available storage space
    pub fn available_space(&self) -> usize {
        self.available_space
    }

    /// Returns a reference to all stored files
    pub fn stored_files(&self) -> &HashMap<String, PathBuf> {
        &self.stored_files
    }
}


pub struct CoordinatorServer {
    pub peer_id: PeerId,
    pub swarm: Swarm<Kademlia<MemoryStore>>,
    pub clients: Arc<Mutex<HashMap<PeerId, String>>>,
    pub storage_nodes: Arc<Mutex<HashMap<PeerId, Store>>>, // Key = PeerId, Value = Store
    pub file_locations: Arc<Mutex<HashMap<String, PeerId>>>, // Key = filename, Value = PeerId of storage node
}

impl CoordinatorServer {
    /// Creates a new CoordinatorServer instance
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // Generate local keypair and PeerId
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());
        println!("Local node peer id: {:?}", peer_id);

        // Create a transport stack
        let transport = development_transport(local_key.clone()).await?;

        // Create a Kademlia behaviour with a memory store
        let store = MemoryStore::new(peer_id.clone());
        let kademlia = Kademlia::new(peer_id.clone(), store);

        // Build the swarm
        let swarm = Swarm::new(transport, kademlia, peer_id);

        Ok(Self {
            peer_id,
            swarm,
            clients: Arc::new(Mutex::new(HashMap::new())),
            storage_nodes: Arc::new(Mutex::new(HashMap::new())),
            file_locations: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Runs the server's main event loop
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(event) = self.swarm.next().await {
            match event {
                libp2p::swarm::SwarmEvent::Behaviour(kad_event) => {
                    println!("Kademlia event: {:?}", kad_event);
                }
                libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    println!("Connection established with {:?}", peer_id);
                }
                libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                    if let Some(err) = cause {
                        println!("Connection closed with {:?} due to {:?}", peer_id, err);
                    } else {
                        println!("Connection closed with {:?}", peer_id);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Adds a client to the coordinator
    pub fn add_client(&self, peer_id: PeerId, name: String) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(peer_id, name);
    }

    /// Adds a storage node to the coordinator
    pub fn add_storage_node(&self, peer_id: PeerId) {
        let mut storage_nodes = self.storage_nodes.lock().unwrap();
        storage_nodes.insert(peer_id, Store::new(peer_id));
    }

    /// Retrieves the list of connected clients
    pub fn list_clients(&self) -> HashMap<PeerId, String> {
        let clients = self.clients.lock().unwrap();
        clients.clone()
    }

    /// Retrieves the list of connected storage nodes
    pub fn list_storage_nodes(&self) -> HashMap<PeerId, usize> {
        let storage_nodes = self.storage_nodes.lock().unwrap();
        storage_nodes
            .iter()
            .map(|(peer_id, store)| (peer_id.clone(), store.available_space()))
            .collect()
    }
    
    /// Uploads a file to a specific storage node
    pub fn upload_file(
        &self,
        node_id: &PeerId,
        filename: String,
        data: Vec<u8>,
    ) -> Result<(), &'static str> {
        let mut storage_nodes = self.storage_nodes.lock().unwrap();
        if let Some(node) = storage_nodes.get_mut(node_id) {
            node.store_file(filename.clone(), data)?;
            let mut file_locations = self.file_locations.lock().unwrap();
            file_locations.insert(filename, node_id.clone());
            Ok(())
        } else {
            Err("Storage node not found")
        }
    }
    
    /// Downloads a file from the storage node that stores it
    pub fn download_file(&self, filename: &str) -> Result<Vec<u8>, &'static str> {
        let file_locations = self.file_locations.lock().unwrap();
        if let Some(node_id) = file_locations.get(filename) {
            let storage_nodes = self.storage_nodes.lock().unwrap();
            if let Some(node) = storage_nodes.get(node_id) {
                node.get_file(filename)
                    .ok_or("File not found on the storage node")
            } else {
                Err("Storage node not found")
            }
        } else {
            Err("File not found in the network")
        }
    }

    /// Removes a file from the storage node and the file_locations map
    pub fn remove_file(&self, filename: &str) -> Result<(), &'static str> {
        let mut file_locations = self.file_locations.lock().unwrap();
        if let Some(node_id) = file_locations.remove(filename) {
            let mut storage_nodes = self.storage_nodes.lock().unwrap();
            if let Some(node) = storage_nodes.get_mut(&node_id) {
                node.remove_file(filename)
            } else {
                Err("Storage node not found")
            }
        } else {
            Err("File not found in the network")
        }
    }

}
