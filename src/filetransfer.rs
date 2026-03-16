use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileTransferMessage {
    /// Request to send a file
    Request {
        file_id: String,
        filename: String,
        file_size: u64,
        checksum: String, // SHA256
    },
    /// Accept file transfer
    Accept { file_id: String },
    /// Reject file transfer
    Reject { file_id: String, reason: String },
    /// Chunk of file data
    Chunk {
        file_id: String,
        chunk_index: u32,
        data: Vec<u8>,
    },
    /// Transfer completed
    Complete { file_id: String, checksum: String },
    /// Transfer failed
    Failed { file_id: String, reason: String },
}

impl FileTransferMessage {
    pub fn new_request(filename: &str, file_size: u64, data: &[u8]) -> Self {
        let checksum = calculate_checksum(data);
        Self::Request {
            file_id: uuid::Uuid::new_v4().to_string(),
            filename: filename.to_string(),
            file_size,
            checksum,
        }
    }

    pub fn new_accept(file_id: &str) -> Self {
        Self::Accept {
            file_id: file_id.to_string(),
        }
    }

    pub fn new_reject(file_id: &str, reason: &str) -> Self {
        Self::Reject {
            file_id: file_id.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn new_chunk(file_id: &str, chunk_index: u32, data: Vec<u8>) -> Self {
        Self::Chunk {
            file_id: file_id.to_string(),
            chunk_index,
            data,
        }
    }

    pub fn new_complete(file_id: &str, checksum: &str) -> Self {
        Self::Complete {
            file_id: file_id.to_string(),
            checksum: checksum.to_string(),
        }
    }

    pub fn new_failed(file_id: &str, reason: &str) -> Self {
        Self::Failed {
            file_id: file_id.to_string(),
            reason: reason.to_string(),
        }
    }

    pub fn serialize(&self) -> anyhow::Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn deserialize(data: &[u8]) -> anyhow::Result<Self> {
        Ok(serde_json::from_slice(data)?)
    }
}

/// Calculate SHA256 checksum of data
pub fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Split file data into chunks
pub fn split_into_chunks(data: &[u8]) -> Vec<Vec<u8>> {
    data.chunks(CHUNK_SIZE).map(|c| c.to_vec()).collect()
}

/// Verify file integrity
pub fn verify_integrity(data: &[u8], expected_checksum: &str) -> bool {
    calculate_checksum(data) == expected_checksum
}

/// File transfer progress tracker
#[derive(Debug, Clone)]
pub struct TransferProgress {
    pub file_id: String,
    pub filename: String,
    pub total_size: u64,
    pub received_chunks: u32,
    pub total_chunks: u32,
    pub status: TransferStatus,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}

impl TransferProgress {
    pub fn new(file_id: String, filename: String, total_size: u64, total_chunks: u32) -> Self {
        Self {
            file_id,
            filename,
            total_size,
            received_chunks: 0,
            total_chunks,
            status: TransferStatus::Pending,
            data: Vec::with_capacity(total_size as usize),
        }
    }

    pub fn add_chunk(&mut self, chunk: Vec<u8>) {
        self.data.extend(chunk);
        self.received_chunks += 1;
        if self.received_chunks == self.total_chunks {
            self.status = TransferStatus::Completed;
        } else {
            self.status = TransferStatus::InProgress;
        }
    }

    pub fn progress_percent(&self) -> f64 {
        if self.total_chunks == 0 {
            return 0.0;
        }
        (self.received_chunks as f64 / self.total_chunks as f64) * 100.0
    }

    pub fn mark_failed(&mut self, reason: &str) {
        self.status = TransferStatus::Failed(reason.to_string());
    }
}
