use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Use modules from the library crate
use rustlink::cli::{Commands, Opts};
use rustlink::handlers::{handle_add, handle_chat, handle_friends, handle_init, handle_login, handle_send, handle_status, handle_version};
use rustlink::identity::IdentityManager;
use rustlink::network::P2PNode;
use rustlink::storage::Storage;

// Use lib functions - setup_logging is specific to binary (writes to files)
fn setup_logging() {
    let data_dir = rustlink::get_data_dir();
    std::fs::create_dir_all(&data_dir).ok();

    let log_dir = data_dir.join("logs");
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(&log_dir, "rustlink.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(_guard);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .try_init();
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();

    info!("Starting RustLink v{}", rustlink::get_version());

    let opts = Opts::parse();
    let data_dir = rustlink::get_data_dir();

    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("rustlink.db");
    let storage = Storage::new(&db_path)?;
    let mut identity = IdentityManager::new(&data_dir)?;

    match opts.command {
        Commands::Init { username } => {
            match handle_init(&storage, &mut identity, &username) {
                Ok(peer_id) => {
                    println!("✓ Identidad creada!");
                    println!(" Tu PeerID: {}", peer_id);
                    println!(" Compártelo con tus amigos para conectarse");
                }
                Err(e) => {
                    println!("⚠ {}", e);
                }
            }
        }

        Commands::Login => {
            match handle_login(&mut identity)? {
                Some(peer_id) => {
                    println!("✓ Sesión iniciada");
                    println!(" PeerID: {}", peer_id);
                }
                None => {
                    println!("✗ No hay identidad. Ejecuta 'rustlink init <username>'");
                }
            }
        }

        Commands::Status => {
            match handle_status(&mut identity)? {
                Some((peer_id, username)) => {
                    println!("┌─────────────────────────────────┐");
                    println!("│ Estado de RustLink             │");
                    println!("├─────────────────────────────────┤");
                    println!("│ Usuario: {}                     │", username);
                    println!("│ PeerID: {}... │", &peer_id[..16.min(peer_id.len())]);
                    println!("│ Estado: 🟢 En línea            │");
                    println!("└─────────────────────────────────┘");
                }
                None => {
                    println!("✗ No has iniciado sesión");
                }
            }
        }

        Commands::Friends => {
            let friends = handle_friends(&storage)?;

            if friends.is_empty() {
                println!("No tienes amigos aún.");
                println!("Usa 'rustlink add <peer_id>' para agregar uno.");
            } else {
                println!("╔═══════════════════════════════════════╗");
                println!("║ Amigos ({})                            ║", friends.len());
                println!("╠═══════════════════════════════════════╣");

                for friend in friends {
                    println!(
                        "║   {} ({})║",
                        friend.username,
                        &friend.peer_id[..16.min(friend.peer_id.len())]
                    );
                }

                println!("╚═══════════════════════════════════════╝");
            }
        }

        Commands::Add { peer_id } => {
            match handle_add(&mut identity, &peer_id) {
                Ok(_) => {
                    println!("🔍 Buscando peer {}...", &peer_id[..16.min(peer_id.len())]);
                    println!("✓ Solicitud enviada (DHT en desarrollo)");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Commands::Chat { peer_id } => {
            match handle_chat(&storage, &mut identity, &peer_id) {
                Ok(messages) => {
                    println!(
                        "💬 Abriendo chat con {}...",
                        &peer_id[..16.min(peer_id.len())]
                    );

                    if !messages.is_empty() {
                        println!("\nMensajes recientes:");
                        for msg in messages.iter().take(10).rev() {
                            println!("  {}: {}", &msg.from[..8.min(msg.from.len())], msg.content);
                        }
                    }

                    println!("\n(Chat TUI con ratatui en desarrollo)");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Commands::Tui => {
            let _my_peer_id = identity
                .load_identity()?
                .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;

            info!("Starting TUI...");
            println!("🎨 Abriendo interfaz TUI...");

            // Run the TUI (simplified - would need proper event handling)
            println!("(TUI en desarrollo - usa 'rustlink chat <peer_id>' para chat CLI)");
        }

        Commands::Send { file, to } => {
            match handle_send(&mut identity, &file, &to) {
                Ok(file_size) => {
                    println!("📦 Enviando {} ({} bytes)", file.display(), file_size);
                    println!("████████████████████░░ 80%");

                    println!(
                        "✓ Archivo enviado a {} (implementación en desarrollo)",
                        &to[..16.min(to.len())]
                    );
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Commands::Run { bootstrap } => {
            let peer_id = identity.load_identity()?.ok_or_else(|| {
                anyhow::anyhow!("No has iniciado sesión. Ejecuta 'rustlink init' primero.")
            })?;

            info!("Starting P2P node with peer ID: {}", peer_id);

            println!("🚀 Iniciando nodo P2P...");
            println!(" PeerID: {}", peer_id);

            let node = P2PNode::new(peer_id, storage).await?;

            // Add bootstrap nodes if provided
            if let Some(nodes) = bootstrap {
                for node_addr in nodes {
                    if let Ok(addr) = node_addr.parse() {
                        node.add_bootstrap_node(addr).await?;
                        println!("✓ Bootstrap node agregado");
                    }
                }
            }

            println!("✓ Nodo iniciado");
            println!(" Presiona Ctrl+C para salir\n");

            let addrs = node.get_listen_addresses().await;
            for addr in &addrs {
                println!(" Escuchando en: {}", addr);
            }

            node.start().await?;
        }

        Commands::Version => {
            let version = handle_version();
            println!("RustLink v{}", version);
            println!("P2P CLI Social App - Sin servidores, sin registro");
        }
    }

    Ok(())
}
