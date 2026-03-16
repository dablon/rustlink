use rustlink::network::{DEFAULT_BOOTSTRAP_NODES, P2PEvent};

#[allow(unused_imports)]
use rustlink::network;

#[test]
fn test_default_bootstrap_nodes_empty() {
    assert!(DEFAULT_BOOTSTRAP_NODES.is_empty());
}

#[test]
fn test_p2p_event_debug() {
    use libp2p::identity::PeerId;
    use libp2p::identity::Keypair;

    // Generate a valid peer ID using libp2p
    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    let event = P2PEvent::PeerDiscovered(peer_id);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("PeerDiscovered"));

    let event = P2PEvent::PeerConnected(peer_id);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("PeerConnected"));

    let event = P2PEvent::PeerDisconnected(peer_id);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("PeerDisconnected"));

    let event = P2PEvent::MessageReceived(peer_id, vec![1, 2, 3]);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("MessageReceived"));

    let event = P2PEvent::FileReceived(peer_id, "test.txt".to_string(), vec![1, 2, 3]);
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("FileReceived"));
}

#[test]
fn test_p2p_event_clone() {
    use libp2p::identity::PeerId;
    use libp2p::identity::Keypair;

    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    let event = P2PEvent::PeerDiscovered(peer_id);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::PeerDiscovered(_)));
}
