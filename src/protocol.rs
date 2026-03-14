//! Chat protocol for RustLink
//! Protocol: /rustlink/chat/1.0.0

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Chat message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessage {
    /// Direct text message
    Text {
        id: String,
        from: String,
        to: String,
        content: String,
        timestamp: u64,
    },
    /// File transfer request
    FileOffer {
        id: String,
        from: String,
        to: String,
        filename: String,
        file_size: u64,
        file_hash: String,
        timestamp: u64,
    },
    /// File transfer acceptance
    FileAccept {
        id: String,
        offer_id: String,
        from: String,
        to: String,
        timestamp: u64,
    },
    /// File transfer rejection
    FileReject {
        id: String,
        offer_id: String,
        from: String,
        to: String,
        reason: String,
        timestamp: u64,
    },
    /// Delivery receipt
    Receipt {
        id: String,
        message_id: String,
        status: ReceiptStatus,
        timestamp: u64,
    },
    /// Friend request
    FriendRequest {
        id: String,
        from: String,
        username: String,
        timestamp: u64,
    },
    /// Friend request response
    FriendResponse {
        id: String,
        request_id: String,
        from: String,
        accepted: bool,
        timestamp: u64,
    },
}

/// Receipt status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReceiptStatus {
    Sent,
    Delivered,
    Read,
}

impl ChatMessage {
    /// Create a new text message
    pub fn new_text(from: &str, to: &str, content: &str) -> Self {
        ChatMessage::Text {
            id: uuid::Uuid::new_v4().to_string(),
            from: from.to_string(),
            to: to.to_string(),
            content: content.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create a file offer
    pub fn new_file_offer(from: &str, to: &str, filename: &str, file_size: u64, file_hash: &str) -> Self {
        ChatMessage::FileOffer {
            id: uuid::Uuid::new_v4().to_string(),
            from: from.to_string(),
            to: to.to_string(),
            filename: filename.to_string(),
            file_size,
            file_hash: file_hash.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create a delivery receipt
    pub fn new_receipt(message_id: &str, from: &str, to: &str, status: ReceiptStatus) -> Self {
        ChatMessage::Receipt {
            id: uuid::Uuid::new_v4().to_string(),
            message_id: message_id.to_string(),
            status,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create a friend request
    pub fn new_friend_request(from: &str, username: &str) -> Self {
        ChatMessage::FriendRequest {
            id: uuid::Uuid::new_v4().to_string(),
            from: from.to_string(),
            username: username.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Get message ID
    pub fn id(&self) -> &str {
        match self {
            ChatMessage::Text { id, .. } => id,
            ChatMessage::FileOffer { id, .. } => id,
            ChatMessage::FileAccept { id, .. } => id,
            ChatMessage::FileReject { id, .. } => id,
            ChatMessage::Receipt { id, .. } => id,
            ChatMessage::FriendRequest { id, .. } => id,
            ChatMessage::FriendResponse { id, .. } => id,
        }
    }
    
    /// Get sender peer ID
    pub fn from(&self) -> &str {
        match self {
            ChatMessage::Text { from, .. } => from,
            ChatMessage::FileOffer { from, .. } => from,
            ChatMessage::FileAccept { from, .. } => from,
            ChatMessage::FileReject { from, .. } => from,
            ChatMessage::Receipt { .. } => "",
            ChatMessage::FriendRequest { from, .. } => from,
            ChatMessage::FriendResponse { from, .. } => from,
        }
    }
    
    /// Get recipient peer ID
    pub fn to(&self) -> &str {
        match self {
            ChatMessage::Text { to, .. } => to,
            ChatMessage::FileOffer { to, .. } => to,
            ChatMessage::FileAccept { to, .. } => to,
            ChatMessage::FileReject { to, .. } => to,
            ChatMessage::Receipt { .. } => "",
            ChatMessage::FriendRequest { .. } => "",
            ChatMessage::FriendResponse { .. } => "",
        }
    }
    
    /// Serialize message to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
    
    /// Deserialize message from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(data)?)
    }
}

/// Protocol constants
pub const CHAT_PROTOCOL: &str = "/rustlink/chat/1.0.0";
pub const FILE_PROTOCOL: &str = "/rustlink/filetransfer/1.0.0";
