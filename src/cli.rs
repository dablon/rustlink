use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// RustLink - P2P CLI Social App
/// Decentralized communication without servers
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
    /// Create a new identity and register username
    Register {
        /// Username to register
        username: String,
    },
    
    /// Load existing identity
    Login,
    
    /// Show current identity status
    Status,
    
    /// List all friends
    Friends,
    
    /// Add a friend by username
    AddFriend {
        /// Username of the friend to add
        username: String,
    },
    
    /// Open chat with a friend
    Chat {
        /// Username to chat with
        username: String,
    },
    
    /// Send a file to a friend
    SendFile {
        /// Path to the file to send
        file: PathBuf,
        
        /// Username of the recipient
        to: String,
    },
    
    /// Start the P2P node (daemon mode)
    Run,
    
    /// Show version information
    Version,
}
