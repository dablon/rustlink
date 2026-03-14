use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::storage::Storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: i64,
    pub msg_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    File,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub message: ChatMessage,
    pub signature: Option<Vec<u8>>,
}

pub struct MessagingService {
    storage: Storage,
    pending_messages: Arc<RwLock<HashMap<String, Vec<ChatMessage>>>>,
}

impl MessagingService {
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            pending_messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Send a message to a peer
    pub async fn send_message(
        &self,
        from_peer: &str,
        to_peer: &str,
        content: String,
        msg_type: MessageType,
    ) -> Result<ChatMessage> {
        let message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: from_peer.to_string(),
            to: to_peer.to_string(),
            content,
            timestamp: chrono::Utc::now().timestamp(),
            msg_type,
        };
        
        // Store in local database
        self.storage.save_message(
            &message.id,
            &message.from,
            &message.to,
            message.content.as_bytes(),
        )?;
        
        info!("Message {} stored locally", message.id);
        
        Ok(message)
    }
    
    /// Get messages with a specific peer
    pub async fn get_messages(&self, peer_id: &str) -> Result<Vec<ChatMessage>> {
        let messages = self.storage.get_messages(peer_id)?;
        
        Ok(messages
            .into_iter()
            .map(|m| ChatMessage {
                id: m.id.to_string(),
                from: m.from,
                to: m.to,
                content: m.content,
                timestamp: chrono::Utc::now().timestamp(),
                msg_type: MessageType::Text,
            })
            .collect())
    }
    
    /// Queue a message for delivery when peer comes online
    pub async fn queue_message(&self, peer_id: &str, message: ChatMessage) {
        let mut pending = self.pending_messages.write().await;
        pending
            .entry(peer_id.to_string())
            .or_insert_with(Vec::new)
            .push(message);
    }
    
    /// Get queued messages for a peer
    pub async fn get_queued_messages(&self, peer_id: &str) -> Vec<ChatMessage> {
        let pending = self.pending_messages.read().await;
        pending.get(peer_id).cloned().unwrap_or_default()
    }
    
    /// Clear queued messages for a peer (after successful delivery)
    pub async fn clear_queued_messages(&self, peer_id: &str) {
        let mut pending = self.pending_messages.write().await;
        pending.remove(peer_id);
    }
}
