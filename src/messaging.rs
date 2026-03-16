use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessage {
    /// Direct message to a peer
    Direct {
        msg_uuid: String,
        from: String,
        content: String,
        timestamp: i64,
    },
    /// Acknowledgment of delivery
    Acknowledgment {
        msg_uuid: String,
        status: DeliveryStatus,
    },
    /// Friend request
    FriendRequest {
        from: String,
        username: String,
    },
    /// Friend request response
    FriendResponse {
        from: String,
        accepted: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeliveryStatus {
    Sent,
    Delivered,
    Read,
}

impl ChatMessage {
    pub fn new_direct(from: String, content: String) -> Self {
        Self::Direct {
            msg_uuid: uuid::Uuid::new_v4().to_string(),
            from,
            content,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn new_ack(msg_uuid: String, status: DeliveryStatus) -> Self {
        Self::Acknowledgment { msg_uuid, status }
    }

    pub fn new_friend_request(from: String, username: String) -> Self {
        Self::FriendRequest { from, username }
    }

    pub fn new_friend_response(from: String, accepted: bool) -> Self {
        Self::FriendResponse { from, accepted }
    }

    pub fn serialize(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn deserialize(data: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = ChatMessage::new_direct(
            "QmPeerId123".to_string(),
            "Hello, world!".to_string(),
        );
        let data = msg.serialize().unwrap();
        let decoded = ChatMessage::deserialize(&data).unwrap();

        match decoded {
            ChatMessage::Direct { content, .. } => {
                assert_eq!(content, "Hello, world!");
            }
            _ => panic!("Expected Direct message"),
        }
    }
}
