use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// RustLink CLI
#[derive(Parser)]
#[clap(name = "rustlink")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "P2P CLI Social App - Chat and share files without servers")]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new identity
    Init {
        /// Username for this identity
        username: String,
    },
    
    /// Load existing identity
    Login,
    
    /// Show current status
    Status,
    
    /// List friends
    Friends,
    
    /// Add a friend by peer ID
    Add {
        /// Peer ID of the friend
        peer_id: String,
    },
    
    /// Open chat with a friend
    Chat {
        /// Peer ID or username to chat with
        peer_id: String,
    },
    
    /// Send a file to a friend
    Send {
        /// Path to the file
        file: PathBuf,
        
        /// Peer ID of the recipient
        to: String,
    },
    
    /// Start P2P node (daemon mode)
    Run,
    
    /// Show version
    Version,
}
