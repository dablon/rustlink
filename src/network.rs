use anyhow::Result;
use libp2p::futures::StreamExt;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Behaviour as Kademlia;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::Transport;
use libp2p::{identity::PeerId, Multiaddr};
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::storage::Storage;

pub struct P2PNode {
    #[allow(dead_code)]
    swarm: Swarm<Kademlia<MemoryStore>>,
    #[allow(dead_code)]
    peer_id: PeerId,
    #[allow(dead_code)]
    storage: Storage,
    event_tx: broadcast::Sender<P2PEvent>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum P2PEvent {
    PeerDiscovered(PeerId),
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    MessageReceived(PeerId, Vec<u8>),
    FileReceived(PeerId, String, Vec<u8>),
}

#[allow(dead_code)]
impl P2PNode {
    pub async fn new(peer_id_str: String, storage: Storage) -> Result<Self> {
        let peer_id = peer_id_str.parse()?;

        info!("Initializing P2P node with peer ID: {}", peer_id);

        let keypair = libp2p::identity::Keypair::generate_ed25519();

        let tcp_transport = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default());

        let transport = tcp_transport
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&keypair)?)
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        let store = MemoryStore::new(peer_id);
        let kad_config = libp2p::kad::Config::default();
        let kademlia = Kademlia::with_config(peer_id, store, kad_config);

        let swarm_config = libp2p::swarm::Config::without_executor();
        let mut swarm = Swarm::new(transport, kademlia, peer_id, swarm_config);

        let (event_tx, _) = broadcast::channel(100);

        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
        swarm.listen_on(listen_addr)?;

        info!("P2P node initialized");

        Ok(Self {
            swarm,
            peer_id,
            storage,
            event_tx,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting P2P event loop");

        self.swarm.behaviour_mut().bootstrap()?;

        loop {
            tokio::select! {
                event = self.swarm.next() => {
                    match event {
                        Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                            info!("Connected: {}", peer_id);
                            let _ = self.event_tx.send(P2PEvent::PeerConnected(peer_id));
                        }
                        Some(SwarmEvent::ConnectionClosed { peer_id, .. }) => {
                            info!("Disconnected: {}", peer_id);
                            let _ = self.event_tx.send(P2PEvent::PeerDisconnected(peer_id));
                        }
                        Some(SwarmEvent::NewListenAddr { address, .. }) => {
                            info!("Listening on: {}", address);
                        }
                        Some(SwarmEvent::Behaviour(event)) => {
                            use libp2p::kad::Event;
                            match event {
                                Event::RoutingUpdated { peer, .. } => {
                                    info!("Peer discovered: {}", peer);
                                    let _ = self.event_tx.send(P2PEvent::PeerDiscovered(peer));
                                }
                                Event::UnroutablePeer { peer, .. } => {
                                    warn!("Unroutable: {}", peer);
                                }
                                _ => {}
                            }
                        }
                        None => break,
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(60)) => {
                    info!("Heartbeat - bootstrap");
                    self.swarm.behaviour_mut().bootstrap().ok();
                }
            }
        }

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<P2PEvent> {
        self.event_tx.subscribe()
    }

    pub fn get_listen_addresses(&self) -> Vec<Multiaddr> {
        self.swarm.listeners().cloned().collect()
    }

    pub fn put_value(&mut self, key: &str, value: Vec<u8>) {
        use libp2p::kad::Record;
        let key = libp2p::kad::RecordKey::new(&key.as_bytes());
        let record = Record {
            key,
            value,
            publisher: Some(self.peer_id),
            expires: None,
        };
        let _ = self
            .swarm
            .behaviour_mut()
            .put_record(record, libp2p::kad::Quorum::One);
    }

    pub fn get_value(&mut self, key: &str) {
        let key = libp2p::kad::RecordKey::new(&key.as_bytes());
        self.swarm.behaviour_mut().get_record(key);
    }

    pub fn add_address(&mut self, peer: &PeerId, addr: Multiaddr) {
        self.swarm.behaviour_mut().add_address(peer, addr);
    }

    pub async fn dial(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm.dial(addr)?;
        Ok(())
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }
}
