use anyhow::Result;
use libp2p::futures::StreamExt;
use libp2p::kad::store::MemoryStore;
use libp2p::kad::Behaviour as Kademlia;
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::Transport;
use libp2p::{identity::PeerId, Multiaddr};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::storage::Storage;

/// Default bootstrap nodes (empty by default - configure via --bootstrap)
pub const DEFAULT_BOOTSTRAP_NODES: &[&str] = &[];

pub struct P2PNode {
    pub swarm: Arc<Mutex<Swarm<Kademlia<MemoryStore>>>>,
    pub peer_id: PeerId,
    pub storage: Storage,
    event_tx: broadcast::Sender<P2PEvent>,
}

#[derive(Debug, Clone)]
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

        // Generate keypair
        let keypair = libp2p::identity::Keypair::generate_ed25519();

        // Create TCP transport
        let tcp_transport = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default());

        let transport = tcp_transport
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(libp2p::noise::Config::new(&keypair)?)
            .multiplex(libp2p::yamux::Config::default())
            .boxed();

        // Create Kademlia DHT
        let store = MemoryStore::new(peer_id);
        let kademlia = Kademlia::new(peer_id, store);

        // Create Swarm
        let swarm_config = libp2p::swarm::Config::without_executor();
        let mut swarm = Swarm::new(transport, kademlia, peer_id, swarm_config);

        let (event_tx, _) = broadcast::channel(100);

        // Listen on all interfaces
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
        swarm.listen_on(listen_addr)?;

        info!("P2P node initialized with Kademlia DHT");

        Ok(Self {
            swarm: Arc::new(Mutex::new(swarm)),
            peer_id,
            storage,
            event_tx,
        })
    }

    /// Add a bootstrap node dynamically
    pub async fn add_bootstrap_node(&self, addr: Multiaddr) -> Result<()> {
        let mut swarm = self.swarm.lock().await;
        if let Some(peer_id) = addr.iter().find_map(|p| match p {
            libp2p::multiaddr::Protocol::P2p(h) => Some(h),
            _ => None,
        }) {
            info!("Adding bootstrap node: {}", peer_id);
            swarm.behaviour_mut().add_address(&peer_id, addr);
        }
        Ok(())
    }

    /// Start DHT bootstrap
    pub async fn bootstrap(&self) -> Result<()> {
        let mut swarm = self.swarm.lock().await;
        swarm.behaviour_mut().bootstrap()?;
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting P2P event loop");

        let swarm = self.swarm.clone();
        let event_tx = self.event_tx.clone();

        // Initial bootstrap
        {
            let mut swarm_guard = swarm.lock().await;
            swarm_guard.behaviour_mut().bootstrap()?;
        }

        // Spawn event loop in background
        tokio::spawn(async move {
            let mut swarm = swarm.lock().await;

            loop {
                tokio::select! {
                    event = swarm.next() => {
                        match event {
                            Some(SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                                info!("Connected: {}", peer_id);
                                let _ = event_tx.send(P2PEvent::PeerConnected(peer_id));
                            }
                            Some(SwarmEvent::ConnectionClosed { peer_id, .. }) => {
                                info!("Disconnected: {}", peer_id);
                                let _ = event_tx.send(P2PEvent::PeerDisconnected(peer_id));
                            }
                            Some(SwarmEvent::NewListenAddr { address, .. }) => {
                                info!("Listening on: {}", address);
                            }
                            Some(SwarmEvent::Behaviour(event)) => {
                                use libp2p::kad::Event;
                                match event {
                                    Event::RoutingUpdated { peer, .. } => {
                                        info!("Peer discovered via DHT: {}", peer);
                                        let _ = event_tx.send(P2PEvent::PeerDiscovered(peer));
                                    }
                                    Event::UnroutablePeer { peer, .. } => {
                                        warn!("Unroutable peer: {}", peer);
                                    }
                                    _ => {}
                                }
                            }
                            None => break,
                            _ => {}
                        }
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_secs(60)) => {
                        info!("Heartbeat - Kademlia bootstrap");
                        swarm.behaviour_mut().bootstrap().ok();
                    }
                }
            }
        });

        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<P2PEvent> {
        self.event_tx.subscribe()
    }

    pub async fn get_listen_addresses(&self) -> Vec<Multiaddr> {
        let swarm = self.swarm.lock().await;
        swarm.listeners().cloned().collect()
    }

    /// Put a value in the DHT (for username registration)
    pub async fn put_value(&self, key: &str, value: Vec<u8>) -> Result<()> {
        use libp2p::kad::Record;
        let mut swarm = self.swarm.lock().await;
        let key = libp2p::kad::RecordKey::new(&key.as_bytes());
        let record = Record {
            key,
            value,
            publisher: Some(self.peer_id),
            expires: None,
        };
        swarm
            .behaviour_mut()
            .put_record(record, libp2p::kad::Quorum::One)?;
        Ok(())
    }

    /// Get a value from the DHT (for username lookup)
    pub async fn get_value(&self, key: &str) {
        let mut swarm = self.swarm.lock().await;
        let key = libp2p::kad::RecordKey::new(&key.as_bytes());
        swarm.behaviour_mut().get_record(key);
    }

    /// Add a known peer's address
    pub async fn add_address(&self, peer: &PeerId, addr: Multiaddr) {
        let mut swarm = self.swarm.lock().await;
        swarm.behaviour_mut().add_address(peer, addr);
    }

    /// Dial a peer by address
    pub async fn dial(&self, addr: Multiaddr) -> Result<()> {
        let mut swarm = self.swarm.lock().await;
        swarm.dial(addr)?;
        Ok(())
    }

    /// Dial a peer by PeerId (will attempt to find them via DHT first)
    #[allow(dead_code)]
    pub async fn dial_peer(&self, peer_id: PeerId) -> Result<()> {
        let mut swarm = self.swarm.lock().await;
        swarm.dial(peer_id)?;
        Ok(())
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }
}
