use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use directories::ProjectDirs;
use tracing::{info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod cli;
mod identity;
mod network;
mod storage;

use cli::Commands;
use identity::IdentityManager;
use network::P2PNode;
use storage::Storage;

fn get_data_dir() -> PathBuf {
    ProjectDirs::from("com", "rustlink", "RustLink")
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
        Commands::Init { username } => {
            info!("Initializing new identity: {}", username);
            
            if identity.has_identity() {
                println!("⚠ Ya existe una identidad. Usa 'rustlink login'");
                return Ok(());
            }
            
            let peer_id = identity.create_identity(&username)?;
            storage.save_identity(&peer_id, &username)?;
            
            println!("✓ Identidad creada!");
            println!(" Tu PeerID: {}", peer_id);
            println!(" Compártelo con tus amigos para conectarse");
        }
        
        Commands::Login => {
            if let Some(peer_id) = identity.load_identity()? {
                println!("✓ Sesión iniciada");
                println!(" PeerID: {}", peer_id);
            } else {
                println!("✗ No hay identidad. Ejecuta 'rustlink init <username>'");
            }
        }
        
        Commands::Status => {
            if let Some(peer_id) = identity.load_identity()? {
                let username = identity.get_username().unwrap_or_else(|| "unknown".to_string());
                println!("┌─────────────────────────────────┐");
                println!("│ Estado de RustLink             │");
                println!("├─────────────────────────────────┤");
                println!("│ Usuario: {}                     │", username);
                println!("│ PeerID: {}... │", &peer_id[..16.min(peer_id.len())]);
                println!("│ Estado: 🟢 En línea            │");
                println!("└─────────────────────────────────┘");
            } else {
                println!("✗ No has iniciado sesión");
            }
        }
        
        Commands::Friends => {
            let friends = storage.get_friends()?;
            
            if friends.is_empty() {
                println!("No tienes amigos aún.");
                println!("Usa 'rustlink add <peer_id>' para agregar uno.");
            } else {
                println!("╔═══════════════════════════════════════╗");
                println!("║ Amigos ({})                            ║", friends.len());
                println!("╠═══════════════════════════════════════╣");
                
                for friend in friends {
                    println!("║   {} ({})║", friend.username, &friend.peer_id[..16.min(friend.peer_id.len())]);
                }
                
                println!("╚═══════════════════════════════════════╝");
            }
        }
        
        Commands::Add { peer_id } => {
            let _my_peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;
            
            println!("🔍 Buscando peer {}...", &peer_id[..16.min(peer_id.len())]);
            println!("✓ Solicitud enviada (DHT en desarrollo)");
        }
        
        Commands::Chat { peer_id } => {
            let _my_peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;
            
            println!("💬 Abriendo chat con {}...", &peer_id[..16.min(peer_id.len())]);
            
            let messages = storage.get_messages(&peer_id)?;
            
            if !messages.is_empty() {
                println!("\nMensajes recientes:");
                for msg in messages.iter().take(10).rev() {
                    println!("  {}: {}", &msg.from[..8.min(msg.from.len())], msg.content);
                }
            }
            
            println!("\n(Chat TUI con ratatui en desarrollo)");
        }
        
        Commands::Send { file, to } => {
            let _my_peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;
            
            if !file.exists() {
                return Err(anyhow::anyhow!("Archivo no encontrado: {}", file.display()));
            }
            
            let file_size = std::fs::metadata(&file)?.len();
            
            println!("📦 Enviando {} ({} bytes)", file.display(), file_size);
            println!("████████████████████░░░░ 80%");
            
            println!("✓ Archivo enviado a {} (implementación en desarrollo)", &to[..16.min(to.len())]);
        }
        
        Commands::Run => {
            let peer_id = identity.load_identity()?
                .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión. Ejecuta 'rustlink init' primero."))?;
            
            info!("Starting P2P node with peer ID: {}", peer_id);
            
            println!("🚀 Iniciando nodo P2P...");
            println!(" PeerID: {}", peer_id);
            
            let mut node = P2PNode::new(peer_id, storage).await?;
            
            println!("✓ Nodo iniciado");
            println!(" Presiona Ctrl+C para salir\n");
            
            let addrs = node.get_listen_addresses();
            for addr in &addrs {
                println!(" Escuchando en: {}", addr);
            }
            
            node.start().await?;
        }
        
        Commands::Version => {
            println!("RustLink v{}", env!("CARGO_PKG_VERSION"));
            println!("P2P CLI Social App - Sin servidores, sin registro");
        }
    }
    
    Ok(())
}
