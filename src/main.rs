mod node;
use crate::node::CoordinatorServer;
//mod store;
//use store::Store;
use libp2p::PeerId;
use std::fs;
use std::sync::{Arc, Mutex};

/*
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = CoordinatorServer::new().await?;
    server.add_client(PeerId::random(), "Client 1".to_string());
    server.add_storage_node(PeerId::random(), "127.0.0.1:8080".to_string());

    println!("Clients: {:?}", server.list_clients());
    println!("Storage Nodes: {:?}", server.list_storage_nodes());

    server.run().await
}

fn main() {
    // Create a new Store instance wrapped in Arc<Mutex> with a unique PeerId
    let peer_id = PeerId::random();
    let store = Arc::new(Mutex::new(Store::new(peer_id)));

    // Example file name and content
    let file_name = "example.txt";
    let file_content = "Hello, P2P world!".as_bytes().to_vec();

    // Test storing a file
    {
        let mut store_guard = store.lock().unwrap(); // Acquire the lock for mutable access
        match store_guard.store_file(file_name.to_string(), file_content.clone()) {
            Ok(_) => println!("File '{}' stored successfully.", file_name),
            Err(e) => println!("Failed to store file '{}': {}", file_name, e),
        }
    }

    // Test retrieving the file
    {
        let store_guard = store.lock().unwrap(); // Acquire the lock for read access
        match store_guard.get_file(file_name) {
            Some(data) => {
                // Convert the file data to a UTF-8 string for display
                let content = String::from_utf8_lossy(&data);
                println!("Retrieved file content: {}", content);
            }
            None => println!("File '{}' not found.", file_name),
        }
    }

    // Test removing the file
    {
        let mut store_guard = store.lock().unwrap(); // Acquire the lock for mutable access
        match store_guard.remove_file(file_name) {
            Ok(_) => println!("File '{}' removed successfully.", file_name),
            Err(e) => println!("Failed to remove file '{}': {}", file_name, e),
        }
    }
}
*/

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = CoordinatorServer::new().await?;

    let storage_node_id = PeerId::random();
    server.add_storage_node(storage_node_id);

    let file_name = "example.txt".to_string();
    let file_content = b"Hello, distributed storage!".to_vec();

    // Upload a file
    match server.upload_file(&storage_node_id, file_name.clone(), file_content.clone()) {
        Ok(_) => println!("File uploaded successfully."),
        Err(e) => println!("Failed to upload file: {}", e),
    }

    // Download the file
    match server.download_file(&file_name) {
        Ok(data) => println!("Downloaded file content: {}", String::from_utf8_lossy(&data)),
        Err(e) => println!("Failed to download file: {}", e),
    }

    // Remove the file
    match server.remove_file(&file_name) {
        Ok(_) => println!("File removed successfully."),
        Err(e) => println!("Failed to remove file: {}", e),
    }

    Ok(())
}
