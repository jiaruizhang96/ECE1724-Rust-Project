use libp2p::NetworkBehaviour;
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::kad::{
    record::store::MemoryStore,
    Kademlia,
    KademliaEvent,
    QueryResult,
    Record,
    PeerRecord,
    PutRecordOk,
};
use libp2p::swarm::NetworkBehaviourEventProcess;
use std::io::Write;

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    // From mDNS: Discovers peers and adds them to Kademlia.
    pub kademlia: Kademlia<MemoryStore>,
    // From Kademlia: Logs key-value operations, such as retrieval or storage success/failure.
    pub mdns: Mdns,
}

// Handle mDNS events: Triggered when new peers are found on the local network.
impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        /*
        1. check if event is MdnsEvent::Discovered
        2. if True, destructure it to extract list and execute the block {}
        if let MdnsEvent::Discovered(list) = event: 
            checks if the value of event matches the variant MdnsEvent::Discovered
        If event is of type MdnsEvent::Discovered, MdnsEvent is an enum,
            DiscoveredList(list) is one possible variants 
        then the block inside {} executes
            the variable list becomes available 
        */
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                //  Adds the discovered peer’s address to the 
                // Kademlia node for further communication.
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

// Handle Kademlia events
// https://tidelabs.github.io/tidechain/src/libp2p_kad/behaviour.rs.html#2479
impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord { record: Record { key, value, .. }, .. } in ok.records {
                        let key = String::from_utf8_lossy(key.as_ref()).to_string();
                        let value = value.clone();
                        // Check if the key is a traditional key-value pair or part of a file storage system
                        if !key.contains('_') {
                            // Case 1: Traditional key-value pair
                            let value_str = String::from_utf8_lossy(&value);
                            println!(
                                "Retrieved traditional key-value pair: Key = '{}', Value = '{}'",
                                key, value_str
                            );
                            continue; // Skip further processing for this key
                        }

                        let parts: Vec<&str> = key.split('_').collect();
                        if parts.len() == 2 {
                            // Case 2: Key contains 1 underscore, this represents total chunk metadata
                            let file_key = parts[0];
                            let total_chunk_number: usize = String::from_utf8_lossy(&value)
                                .parse()
                                .unwrap_or_else(|_| {
                                    eprintln!("Invalid total chunk number in key '{}'", key);
                                    0
                                });

                            if total_chunk_number == 0 {
                                eprintln!("Total chunk number is 0 for key '{}'.", key);
                                continue;
                            }

                            // Retrieve the first chunk
                            let first_chunk_key = libp2p::kad::record::Key::new(&format!(
                                "{}_{}_{}",
                                file_key, 0, total_chunk_number
                            ));
                            self.kademlia
                                .get_record(&first_chunk_key, libp2p::kad::Quorum::One);
                            println!(
                                "Retrieving first chunk for file '{}', total chunks: {}.",
                                file_key, total_chunk_number
                            );
                        } else if parts.len() == 3 {
                            // Case 3: Key contains 2 underscores, this represents a file chunk
                            // Parse the file key, current chunk number, and total chunk number
                            let file_key = parts[0];
                            let current_chunk_number: usize = parts[1].parse().unwrap_or_else(|_| {
                                eprintln!("Invalid current chunk number in key '{}'", key);
                                0
                            });
                            let total_chunk_number: usize = parts[2].parse().unwrap_or_else(|_| {
                                eprintln!("Invalid total chunk number in key '{}'", key);
                                0
                            });

                            // Write the current chunk to disk
                            let file_path = std::env::current_dir()
                                .expect("Failed to get current directory")
                                .join(format!("{}.txt", file_key));

                            let mut open_options = std::fs::OpenOptions::new();
                            if current_chunk_number == 0 {
                                // Overwrite the file if it's the first chunk
                                open_options.create(true).write(true).truncate(true);
                            } else {
                                // Append to the file for other chunks
                                open_options.create(true).append(true);
                            }

                            match open_options.open(&file_path) {
                                Ok(mut file) => {
                                    if let Err(e) = file.write_all(&value) {
                                        eprintln!(
                                            "Failed to write chunk {} to file '{}': {:?}",
                                            current_chunk_number, file_path.display(), e
                                        );
                                    } else {
                                        println!(
                                            "Successfully wrote chunk {} to file '{}'.",
                                            current_chunk_number, file_path.display()
                                        );
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to open file '{}': {:?}", file_path.display(), e);
                                }
                            }

                            // Check if we have reached the last chunk
                            if current_chunk_number + 1 == total_chunk_number {
                                println!(
                                    "All chunks retrieved for file '{}'. File written to '{}'.",
                                    file_key, file_path.display()
                                );
                            } else {
                                // Retrieve the next chunk
                                let next_chunk_key = libp2p::kad::record::Key::new(&format!(
                                    "{}_{}_{}",
                                    file_key,
                                    current_chunk_number + 1,
                                    total_chunk_number
                                ));
                                self.kademlia
                                    .get_record(&next_chunk_key, libp2p::kad::Quorum::One);
                                println!(
                                    "Retrieving next chunk: {} for file '{}'",
                                    current_chunk_number + 1,
                                    file_key
                                );
                            }
                        } else {
                            eprintln!("Unexpected key format: '{}'", key);
                        }



                        
                    }
                }

                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!(
                        "Successfully added record with key: '{}'",
                        String::from_utf8_lossy(key.as_ref())
                    );
                }

                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to retrieve record: {:?}", err);
                }

                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to add record: {:?}", err);
                }

                _ => {}
            },
            _ => {}
        }
    }
}

