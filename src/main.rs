mod node;
mod behaviour;
use async_std::io::{self, BufReader};
use async_std::task;
use futures::StreamExt;
use libp2p::{
    PeerId, Swarm,
    kad::{Kademlia, record::{Key, Record, store::MemoryStore}, Quorum},
    mdns::Mdns,
    development_transport, identity,
};
use behaviour::Behaviour;
use node::Node;
use async_std::io::BufReadExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Create a new node
    let mut node = Node::new().await;

    // Start listening on a default address
    node.start_listening("/ip4/0.0.0.0/tcp/0");

    println!("Node initialized. Enter commands (e.g., PUT <key> <value>, GET <key>, LISTEN <addr>):");

    // Set up stdin for user input
    let stdin = BufReader::new(io::stdin()).lines();
    let mut fused_stdin = stdin.fuse(); // Make it a FusedStream

    // Main event loop
    loop {
        futures::select! {
            // Handle user input
            line = fused_stdin.next() => {
                match line {
                    Some(Ok(input)) => node.handle_input(input).await,
                    Some(Err(e)) => {
                        eprintln!("Error reading input: {:?}", e);
                        break;
                    }
                    None => {
                        println!("Stdin closed. Exiting...");
                        break;
                    }
                }
            }

            // Handle swarm events
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
                        libp2p::swarm::SwarmEvent::Behaviour(event) => {
                            println!("Behaviour event: {:?}", event);
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

    Ok(())
}
