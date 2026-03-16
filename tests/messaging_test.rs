use rustlink::messaging::{ChatMessage, DeliveryStatus};

#[test]
fn test_message_serialization() {
    let msg = ChatMessage::new_direct("QmPeerId123".to_string(), "Hello, world!".to_string());
    let data = msg.serialize().unwrap();
    let decoded = ChatMessage::deserialize(&data).unwrap();

    match decoded {
        ChatMessage::Direct { content, .. } => {
            assert_eq!(content, "Hello, world!");
        }
        _ => panic!("Expected Direct message"),
    }
}

#[test]
fn test_new_direct() {
    let msg = ChatMessage::new_direct("sender".to_string(), "Test message".to_string());

    match msg {
        ChatMessage::Direct {
            msg_uuid,
            from,
            content,
            timestamp,
        } => {
            assert_eq!(from, "sender");
            assert_eq!(content, "Test message");
            assert!(!msg_uuid.is_empty());
            assert!(timestamp > 0);
        }
        _ => panic!("Expected Direct message"),
    }
}

#[test]
fn test_new_ack() {
    let msg = ChatMessage::new_ack("msg-123".to_string(), DeliveryStatus::Delivered);

    match msg {
        ChatMessage::Acknowledgment { msg_uuid, status } => {
            assert_eq!(msg_uuid, "msg-123");
            assert_eq!(status, DeliveryStatus::Delivered);
        }
        _ => panic!("Expected Acknowledgment"),
    }
}

#[test]
fn test_new_friend_request() {
    let msg = ChatMessage::new_friend_request("peer123".to_string(), "john".to_string());

    match msg {
        ChatMessage::FriendRequest { from, username } => {
            assert_eq!(from, "peer123");
            assert_eq!(username, "john");
        }
        _ => panic!("Expected FriendRequest"),
    }
}

#[test]
fn test_new_friend_response_accepted() {
    let msg = ChatMessage::new_friend_response("peer123".to_string(), true);

    match msg {
        ChatMessage::FriendResponse { from, accepted } => {
            assert_eq!(from, "peer123");
            assert!(accepted);
        }
        _ => panic!("Expected FriendResponse"),
    }
}

#[test]
fn test_new_friend_response_rejected() {
    let msg = ChatMessage::new_friend_response("peer123".to_string(), false);

    match msg {
        ChatMessage::FriendResponse { from, accepted } => {
            assert_eq!(from, "peer123");
            assert!(!accepted);
        }
        _ => panic!("Expected FriendResponse"),
    }
}

#[test]
fn test_ack_all_statuses() {
    for status in [
        DeliveryStatus::Sent,
        DeliveryStatus::Delivered,
        DeliveryStatus::Read,
    ] {
        let msg = ChatMessage::new_ack("test-uuid".to_string(), status.clone());
        if let ChatMessage::Acknowledgment { status: s, .. } = msg {
            assert_eq!(s, status);
        } else {
            panic!("Expected Acknowledgment");
        }
    }
}

#[test]
fn test_serialize_roundtrip_all_types() {
    // Direct
    let direct = ChatMessage::new_direct("from".to_string(), "content".to_string());
    let data = direct.serialize().unwrap();
    let decoded = ChatMessage::deserialize(&data).unwrap();
    assert!(matches!(decoded, ChatMessage::Direct { .. }));

    // Acknowledgment
    let ack = ChatMessage::new_ack("uuid".to_string(), DeliveryStatus::Read);
    let data = ack.serialize().unwrap();
    let decoded = ChatMessage::deserialize(&data).unwrap();
    assert!(matches!(decoded, ChatMessage::Acknowledgment { .. }));

    // FriendRequest
    let req = ChatMessage::new_friend_request("peer".to_string(), "user".to_string());
    let data = req.serialize().unwrap();
    let decoded = ChatMessage::deserialize(&data).unwrap();
    assert!(matches!(decoded, ChatMessage::FriendRequest { .. }));

    // FriendResponse
    let resp = ChatMessage::new_friend_response("peer".to_string(), true);
    let data = resp.serialize().unwrap();
    let decoded = ChatMessage::deserialize(&data).unwrap();
    assert!(matches!(decoded, ChatMessage::FriendResponse { .. }));
}
