use crate::identity::IdentityManager;
use crate::storage::Storage;
use anyhow::Result;
use tracing::info;

/// Handle the Init command - create a new identity
pub fn handle_init(
    storage: &Storage,
    identity: &mut IdentityManager,
    username: &str,
) -> Result<String> {
    info!("Initializing new identity: {}", username);

    if identity.has_identity() {
        return Err(anyhow::anyhow!("Ya existe una identidad"));
    }

    let peer_id = identity.create_identity(username)?;
    storage.save_identity(&peer_id, username)?;
    Ok(peer_id)
}

/// Handle the Login command - load existing identity
pub fn handle_login(identity: &mut IdentityManager) -> Result<Option<String>> {
    let peer_id = identity.load_identity()?;
    Ok(peer_id)
}

/// Handle the Status command - show current status
pub fn handle_status(identity: &mut IdentityManager) -> Result<Option<(String, String)>> {
    if let Some(peer_id) = identity.load_identity()? {
        let username = identity
            .get_username()
            .unwrap_or_else(|| "unknown".to_string());
        Ok(Some((peer_id, username)))
    } else {
        Ok(None)
    }
}

/// Handle the Friends command - list friends
pub fn handle_friends(storage: &Storage) -> Result<Vec<crate::storage::Friend>> {
    let friends = storage.get_friends()?;
    Ok(friends)
}

/// Handle the Add command - add a friend (returns the peer_id being added)
pub fn handle_add(identity: &mut IdentityManager, peer_id: &str) -> Result<String> {
    let _my_peer_id = identity
        .load_identity()?
        .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;
    Ok(peer_id.to_string())
}

/// Handle the Chat command - open chat with a friend
pub fn handle_chat(
    storage: &Storage,
    identity: &mut IdentityManager,
    peer_id: &str,
) -> Result<Vec<crate::storage::Message>> {
    let _my_peer_id = identity
        .load_identity()?
        .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;

    let messages = storage.get_messages(peer_id)?;
    Ok(messages)
}

/// Handle the Send command - send a file (returns file info)
pub fn handle_send(
    identity: &mut IdentityManager,
    file: &std::path::Path,
    _to: &str,
) -> Result<u64> {
    let _my_peer_id = identity
        .load_identity()?
        .ok_or_else(|| anyhow::anyhow!("No has iniciado sesión"))?;

    if !file.exists() {
        return Err(anyhow::anyhow!("Archivo no encontrado: {}", file.display()));
    }

    let file_size = std::fs::metadata(file)?.len();
    Ok(file_size)
}

/// Handle the Version command - show version
pub fn handle_version() -> String {
    crate::get_version()
}
