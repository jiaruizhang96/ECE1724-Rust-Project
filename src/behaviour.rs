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
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns,
}

// Handle mDNS events
impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

// Handle Kademlia events
impl NetworkBehaviourEventProcess<KademliaEvent> for Behaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::OutboundQueryCompleted { result, .. } => match result {
                QueryResult::GetRecord(Ok(ok)) => {
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
