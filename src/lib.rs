pub mod cli;
pub mod filetransfer;
pub mod handlers;
pub mod identity;
pub mod messaging;
pub mod network;
pub mod storage;
pub mod tui;

use directories::ProjectDirs;
use std::path::PathBuf;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Get the data directory for rustlink
/// This function is pub for testing
pub fn get_data_dir() -> PathBuf {
    get_data_dir_internal()
}

fn get_data_dir_internal() -> PathBuf {
    // Check for custom data dir via environment variable
    if let Ok(custom_dir) = std::env::var("RUSTLINK_DATA_DIR") {
        return PathBuf::from(custom_dir);
    }

    // Check for HOME environment
    if let Ok(home) = std::env::var("HOME") {
        let data_dir = PathBuf::from(home).join(".local/share/rustlink");
        if data_dir.exists() || std::fs::create_dir_all(&data_dir).is_ok() {
            return data_dir;
        }
    }

    ProjectDirs::from("com", "rustlink", "RustLink")
        .map(|d| d.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Get current version string
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Setup logging (no-op for tests)
pub fn setup_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stdout))
        .try_init();
}
