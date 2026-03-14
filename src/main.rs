use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod identity;
mod messaging;
mod network;
mod storage;

use cli::Commands;
use identity::IdentityManager;
use network::P2PNode;
use storage::Storage;

fn get_data_dir() -> PathBuf {
    directories::ProjectDirs::from("com", "rustlink", "RustLink")
        .map(|d| d.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn setup_logging() {
    let data_dir = get_data_dir();
    std::fs::create_dir_all(&data_dir).ok();
    
    let log_dir = data_dir.join("logs");
    std::fs::create_dir_all(&log_dir).ok();
    
    let file_appender = tracing_appender::rolling::daily(&log_dir, "rustlink.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Keep guard alive for the lifetime of the program
    std::mem::forget(_guard);
    
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    
    info!("Starting RustLink v{}", env!("CARGO_PKG_VERSION"));
    
    let opts = cli::Opts::parse();
    let data_dir = get_data_dir();
    
    std::fs::create_dir_all(&data_dir)?;
    
    let db_path = data_dir.join("rustlink.db");
    let storage = Storage::new(&db_path)?;
    let mut identity = IdentityManager::new(&data_dir)?;
    
    match opts.command {
        Commands::Register { username } => {
            info!("Registering new user: {}", username);
            identity.create_identity(&username)?;
            storage.save_identity(&identity.get_peer_id(), &username)?;
            info!("Identity created successfully!");
            println!("✓ Registered as {} (Peer ID: {})", username, identity.get_peer_id());
        }
        Commands::Login => {
            if let Some(id) = identity.load_identity()? {
                info!("Loaded identity: {}", id);
                println!("✓ Logged in as (Peer ID: {})", id);
            } else {
                println!("No identity found. Run 'rustlink register <username>' first.");
            }
        }
        Commands::Status => {
            if let Some(id) = identity.load_identity()? {
                println!("Peer ID: {}", id);
                println!("Status: Online");
            } else {
                println!("Not logged in.");
            }
        }
        Commands::Friends => {
            let friends = storage.get_friends()?;
            if friends.is_empty() {
                println!("No friends yet. Use 'add-friend <username>' to add friends.");
            } else {
                println!("Friends ({}):", friends.len());
                for friend in friends {
                    println!("  - {} ({})", friend.username, friend.peer_id);
                }
            }
        }
        Commands::AddFriend { username } => {
            let _peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;
            
            info!("Adding friend: {}", username);
            // TODO: Query DHT for peer, then add
            println!("Friend request sent to {} (DHT lookup would happen here)", username);
            storage.add_friend(&username, &format!("pending-{}", username))?;
        }
        Commands::Chat { username } => {
            let _peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;
            
            println!("Opening chat with {} (P2P connection would be established)", username);
            
            // Show message history
            let messages = storage.get_messages(&username)?;
            for msg in messages.iter().take(10) {
                println!("{}: {}", msg.from, msg.content);
            }
            
            // Interactive chat (simplified for now)
            println!("\nChat started. Type your message and press Enter to send.");
            println!("Press Ctrl+C to exit.\n");
        }
        Commands::SendFile { file, to } => {
            let _peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;
            
            info!("Sending file {} to {}", file.display(), to);
            println!("File transfer to {} would start here", to);
        }
        Commands::Run {} => {
            // Start the full P2P node
            let peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("Not logged in. Run 'register' first."))?;
            
            info!("Starting P2P node with peer ID: {}", peer_id);
            
            let mut node = P2PNode::new(peer_id, storage.clone()).await?;
            node.start().await?;
        }
        Commands::Version => {
            println!("RustLink v{}", env!("CARGO_PKG_VERSION"));
        }
    }
    
    Ok(())
}
