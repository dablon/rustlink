use rustlink::network::{P2PEvent, DEFAULT_BOOTSTRAP_NODES};

#[allow(unused_imports)]
use rustlink::network;

#[test]
fn test_default_bootstrap_nodes_empty() {
    assert!(DEFAULT_BOOTSTRAP_NODES.is_empty());
}

#[test]
fn test_p2p_event_debug() {
    use libp2p::identity::Keypair;
    use libp2p::identity::PeerId;

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
    use libp2p::identity::Keypair;
    use libp2p::identity::PeerId;

    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    let event = P2PEvent::PeerDiscovered(peer_id);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::PeerDiscovered(_)));
}

#[test]
fn test_p2p_event_clone_all_variants() {
    use libp2p::identity::Keypair;
    use libp2p::identity::PeerId;

    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    // PeerDiscovered
    let event = P2PEvent::PeerDiscovered(peer_id);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::PeerDiscovered(_)));

    // PeerConnected
    let event = P2PEvent::PeerConnected(peer_id);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::PeerConnected(_)));

    // PeerDisconnected
    let event = P2PEvent::PeerDisconnected(peer_id);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::PeerDisconnected(_)));

    // MessageReceived
    let event = P2PEvent::MessageReceived(peer_id, vec![1, 2, 3]);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::MessageReceived(_, _)));

    // FileReceived
    let event = P2PEvent::FileReceived(peer_id, "test.txt".to_string(), vec![]);
    let cloned = event.clone();
    assert!(matches!(cloned, P2PEvent::FileReceived(_, _, _)));
}

#[test]
fn test_p2p_event_debug_format() {
    use libp2p::identity::Keypair;
    use libp2p::identity::PeerId;

    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    // Test all Debug formats
    let events = vec![
        P2PEvent::PeerDiscovered(peer_id),
        P2PEvent::PeerConnected(peer_id),
        P2PEvent::PeerDisconnected(peer_id),
        P2PEvent::MessageReceived(peer_id, vec![]),
        P2PEvent::FileReceived(peer_id, "file".to_string(), vec![]),
    ];

    for event in events {
        let debug_str = format!("{:?}", event);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_p2p_event_debug_all_variants() {
    use libp2p::identity::Keypair;
    use libp2p::identity::PeerId;

    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    // Test Debug for all variants
    let event1 = P2PEvent::PeerDiscovered(peer_id);
    format!("{:?}", event1);

    let event2 = P2PEvent::PeerConnected(peer_id);
    format!("{:?}", event2);

    let event3 = P2PEvent::PeerDisconnected(peer_id);
    format!("{:?}", event3);

    let event4 = P2PEvent::MessageReceived(peer_id, vec![1, 2, 3]);
    format!("{:?}", event4);

    let event5 = P2PEvent::FileReceived(peer_id, "test.txt".to_string(), vec![4, 5, 6]);
    format!("{:?}", event5);

    // Just ensure no panics
    assert!(true);
}
