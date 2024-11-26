use std::str::from_utf8;
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
            // KademliaEvent::OutboundQueryCompleted: 
            //  Fired when a query (e.g., Get or Put operations) completes.
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                QueryResult::GetRecord(Ok(ok)) => {
                    // If successful, iterates through the retrieved records
                    // and prints their keys and values (converted to UTF-8 strings for readability).
                    for PeerRecord { record: Record { key, value, .. }, .. } in ok.records {
                        println!(
                            "Retrieved: Key = {:?}, Value = {:?}",
                            from_utf8(key.as_ref()).unwrap(),
                            from_utf8(&value).unwrap()
                        );
                    }
                }
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to retrieve record: {:?}", err);
                }
                // Logs success when a record is stored.
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!("Added record with key: {:?}", from_utf8(key.as_ref()).unwrap());
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
