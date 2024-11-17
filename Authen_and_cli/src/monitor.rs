use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

// struct representing a node
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub is_alive: bool,
}

// holds a node for health check
pub struct Monitor {
    nodes: HashMap<String, Node>,
    check_interval: Duration,
}

impl Monitor {
    pub fn new(check_interval_secs: u64) -> Self {
        Monitor {
            nodes: HashMap::new(),
            check_interval: Duration::from_secs(check_interval_secs),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    // periodically checks the status of each node and updates their status. 
    // if a node is down, it triggers data redistribution.
    pub async fn start_monitoring(&mut self) {
        let (tx, mut rx) = mpsc::channel(32);

        // spawn a task to periodically check node status
        let nodes_clone = self.nodes.clone();
        let interval = self.check_interval;

        tokio::spawn(async move {
            loop {
                for node in nodes_clone.values() {
                    // TODO: Implement actual health check logic here
                    // for now, simulate with a random boolean
                    let is_alive = true; // Replace with actual check
                    tx.send((node.id.clone(), is_alive)).await.unwrap();
                }
                sleep(interval).await;
            }
        });

        // process health check results
        while let Some((node_id, is_alive)) = rx.recv().await {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.is_alive = is_alive;
                if !is_alive {
                    println!("Node {} is down. Initiating data redistribution.", node_id);
                    // TODO: call data redistribution function here
                }
            }
        }
    }
}
