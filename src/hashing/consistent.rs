use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Manages a hash ring with nodes and their hash values.
struct HashRing {
    nodes: HashSet<u64>,
    hash_map: HashMap<u64, u64>,
}

impl HashRing {
    /// Initializes a new HashRing.
    pub fn new() -> Self {
        HashRing {
            nodes: HashSet::new(),
            hash_map: HashMap::new(),
        }
    }

    /// Adds a node to the hash ring and records its hash value.
    pub fn add_node(&mut self, node_id: u64) {
        let hash_value = self.calculate_hash(node_id);
        self.nodes.insert(node_id);
        self.hash_map.insert(node_id, hash_value);
        println!("Added node ID {} with hash {}", node_id, hash_value);
    }

    /// Removes a node from the hash ring and its hash value.
    pub fn remove_node(&mut self, node_id: u64) {
        if self.nodes.remove(&node_id) {
            self.hash_map.remove(&node_id);
            println!("Removed node ID {}", node_id);
        } else {
            println!("Node ID {} not found", node_id);
        }
    }

    /// Calculates the hash value for a given node ID.
    fn calculate_hash(&self, node_id: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);
        hasher.finish()
    }

    /// Reads node IDs from a file and adds them to the hash ring.
    pub fn load_nodes_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if let Ok(node_id) = line.parse::<u64>() {
                self.add_node(node_id);
            } else {
                eprintln!("Invalid node ID encountered: {}", line);
            }
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut ring = HashRing::new();
    // Specify the file path to load node IDs
    ring.load_nodes_from_file("path/to/your/nodes.txt")?;

    // Additional code to interact with the hash ring can be added here

    Ok(())
}
