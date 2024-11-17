mod authentication;
mod cli;
mod monitor;

use authentication::AuthService;
use monitor::{Monitor, Node};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize AuthService
    let auth_service = AuthService::new().await;

    // Start CLI
    tokio::spawn(async move {
        cli::run_cli().await.unwrap();
    });

    // Start Monitoring
    let mut monitor = Monitor::new(10); // Check every 10 seconds
    monitor.add_node(Node {
        id: "node1".to_string(),
        address: "127.0.0.1:8080".to_string(),
        is_alive: true,
    });
    monitor.add_node(Node {
        id: "node2".to_string(),
        address: "127.0.0.1:8081".to_string(),
        is_alive: true,
    });

    monitor.start_monitoring().await;

    Ok(())
}
