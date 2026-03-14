//! File transfer handler for RustLink

use anyhow::Result;
use sha2::{Sha256, Digest};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// File transfer metadata
#[derive(Debug, Clone)]
pub struct FileTransfer {
    pub id: String,
    pub filename: String,
    pub file_size: u64,
    pub file_hash: String,
    pub chunks: Vec<FileChunk>,
    pub progress: u64,
    pub status: TransferStatus,
}

/// File chunk for streaming transfer
#[derive(Debug, Clone)]
pub struct FileChunk {
    pub index: u32,
    pub data: Vec<u8>,
    pub hash: String,
}

/// Transfer status
#[derive(Debug, Clone, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Cancelled,
}

impl FileTransfer {
    /// Create a new file transfer from a file path
    pub async fn from_file(path: &Path, id: &str) -> Result<Self> {
        let mut file = File::open(path).await?;
        let metadata = file.metadata().await?;
        let file_size = metadata.len();
        
        // Read entire file to calculate hash
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        
        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        let file_hash = hex::encode(hasher.finalize());
        
        // Create chunks
        let chunk_size = 64 * 1024; // 64KB chunks
        let chunks: Vec<FileChunk> = contents
            .chunks(chunk_size)
            .enumerate()
            .map(|(index, data)| {
                let mut hasher = Sha256::new();
                hasher.update(data);
                let hash = hex::encode(hasher.finalize());
                
                FileChunk {
                    index: index as u32,
                    data: data.to_vec(),
                    hash,
                }
            })
            .collect();
        
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        
        Ok(Self {
            id: id.to_string(),
            filename,
            file_size,
            file_hash,
            chunks,
            progress: 0,
            status: TransferStatus::Pending,
        })
    }
    
    /// Verify chunk integrity
    pub fn verify_chunk(&self, index: u32, data: &[u8]) -> bool {
        if let Some(chunk) = self.chunks.get(index as usize) {
            let mut hasher = Sha256::new();
            hasher.update(data);
            let hash = hex::encode(hasher.finalize());
            hash == chunk.hash
        } else {
            false
        }
    }
    
    /// Get progress percentage
    pub fn progress_percent(&self) -> f64 {
        if self.file_size == 0 {
            return 100.0;
        }
        (self.progress as f64 / self.file_size as f64) * 100.0
    }
    
    /// Mark as completed
    pub fn mark_completed(&mut self) {
        self.status = TransferStatus::Completed;
        self.progress = self.file_size;
    }
    
    /// Mark as failed
    pub fn mark_failed(&mut self, reason: &str) {
        self.status = TransferStatus::Failed(reason.to_string());
    }
}

/// File receiver for handling incoming transfers
pub struct FileReceiver {
    pub id: String,
    pub filename: String,
    pub file_size: u64,
    pub expected_hash: String,
    pub received_chunks: Vec<FileChunk>,
    pub status: TransferStatus,
}

impl FileReceiver {
    /// Create a new file receiver
    pub fn new(id: &str, filename: &str, file_size: u64, file_hash: &str) -> Self {
        Self {
            id: id.to_string(),
            filename: filename.to_string(),
            file_size,
            expected_hash: file_hash.to_string(),
            received_chunks: Vec::new(),
            status: TransferStatus::Pending,
        }
    }
    
    /// Add a chunk
    pub fn add_chunk(&mut self, chunk: FileChunk) -> Result<()> {
        // Verify chunk
        let mut hasher = Sha256::new();
        hasher.update(&chunk.data);
        let hash = hex::encode(hasher.finalize());
        
        if hash != chunk.hash {
            return Err(anyhow::anyhow!("Chunk {} verification failed", chunk.index));
        }
        
        self.received_chunks.push(chunk);
        self.status = TransferStatus::InProgress;
        
        // Calculate progress
        self.progress = self.received_chunks
            .iter()
            .map(|c| c.data.len() as u64)
            .sum();
        
        Ok(())
    }
    
    /// Get progress percentage
    pub fn progress_percent(&self) -> f64 {
        if self.file_size == 0 {
            return 100.0;
        }
        (self.progress as f64 / self.file_size as f64) * 100.0
    }
    
    /// Finalize and save the file
    pub async fn finalize(&self, output_dir: &Path) -> Result<()> {
        if self.status != TransferStatus::Completed {
            return Err(anyhow::anyhow!("Transfer not completed"));
        }
        
        // Reassemble file
        let mut file_path = output_dir.to_path_buf();
        file_path.push(&self.filename);
        
        let mut file = File::create(&file_path).await?;
        
        for chunk in &self.received_chunks {
            file.write_all(&chunk.data).await?;
        }
        
        file.flush().await?;
        
        Ok(())
    }
}
