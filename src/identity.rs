use anyhow::{Context, Result};
use libp2p::identity::{Keypair, PeerId};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tracing::info;

pub struct IdentityManager {
    keypair: Option<Keypair>,
    peer_id: Option<PeerId>,
    data_dir: PathBuf,
}

impl IdentityManager {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let data_dir = data_dir.to_path_buf();
        fs::create_dir_all(&data_dir)?;

        Ok(Self {
            keypair: None,
            peer_id: None,
            data_dir,
        })
    }

    /// Check if identity exists
    pub fn has_identity(&self) -> bool {
        self.data_dir.join("identity.key").exists()
    }

    /// Create a new identity with username
    pub fn create_identity(&mut self, username: &str) -> Result<String> {
        let keypair = Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());

        info!("Generating new identity for user: {}", username);

        // Save the keypair
        let key_path = self.data_dir.join("identity.key");
        let key_bytes = keypair
            .to_protobuf_encoding()
            .context("Failed to encode identity key")?;
        fs::write(&key_path, &key_bytes).context("Failed to save identity key")?;

        // Save username
        let username_path = self.data_dir.join("username.txt");
        fs::write(&username_path, username)?;

        self.keypair = Some(keypair);
        self.peer_id = Some(peer_id);

        Ok(peer_id.to_string())
    }

    /// Load existing identity from disk
    pub fn load_identity(&mut self) -> Result<Option<String>> {
        let key_path = self.data_dir.join("identity.key");

        if !key_path.exists() {
            return Ok(None);
        }

        let key_bytes = fs::read(&key_path).context("Failed to read identity key")?;

        let keypair =
            Keypair::from_protobuf_encoding(&key_bytes).context("Failed to decode identity key")?;

        let peer_id = PeerId::from(keypair.public());

        self.keypair = Some(keypair);
        self.peer_id = Some(peer_id);

        info!("Loaded identity: {}", peer_id);
        Ok(Some(peer_id.to_string()))
    }

    /// Get the current peer ID
    #[allow(dead_code)]
    pub fn get_peer_id(&self) -> String {
        self.peer_id
            .map(|p| p.to_string())
            .unwrap_or_else(|| "Not initialized".to_string())
    }

    /// Get username
    pub fn get_username(&self) -> Option<String> {
        let username_path = self.data_dir.join("username.txt");
        fs::read_to_string(username_path).ok()
    }
}
