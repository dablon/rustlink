use rustlink::identity::IdentityManager;

#[allow(unused_imports)]
use rustlink::identity;
use tempfile::TempDir;

#[test]
fn test_new_creates_data_dir() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    let manager = IdentityManager::new(&data_dir).unwrap();

    assert!(data_dir.exists());
    assert!(!manager.has_identity());
}

#[test]
fn test_create_identity() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    let mut manager = IdentityManager::new(&data_dir).unwrap();

    let peer_id = manager.create_identity("testuser").unwrap();

    // Check peer ID format
    assert!(peer_id.starts_with("12D3KooW"));

    // Check files created
    assert!(data_dir.join("identity.key").exists());
    assert!(data_dir.join("username.txt").exists());
    assert!(manager.has_identity());
}

#[test]
fn test_load_identity() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    // First create an identity
    let mut manager = IdentityManager::new(&data_dir).unwrap();
    let original_peer_id = manager.create_identity("testuser").unwrap();

    // Now create a new manager and load
    let mut manager2 = IdentityManager::new(&data_dir).unwrap();
    let loaded_peer_id = manager2.load_identity().unwrap();

    assert!(loaded_peer_id.is_some());
    assert_eq!(original_peer_id, loaded_peer_id.unwrap());
}

#[test]
fn test_load_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    let mut manager = IdentityManager::new(&data_dir).unwrap();
    let result = manager.load_identity().unwrap();

    assert!(result.is_none());
}

#[test]
fn test_get_peer_id() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    let mut manager = IdentityManager::new(&data_dir).unwrap();

    // Before creation
    assert_eq!(manager.get_peer_id(), "Not initialized");

    // After creation
    manager.create_identity("testuser").unwrap();
    let peer_id = manager.get_peer_id();
    assert!(peer_id.starts_with("12D3KooW"));
}

#[test]
fn test_get_username() {
    let temp_dir = TempDir::new().unwrap();
    let data_dir = temp_dir.path().to_path_buf();

    let mut manager = IdentityManager::new(&data_dir).unwrap();

    // Before creation - no username
    assert!(manager.get_username().is_none());

    // After creation
    manager.create_identity("testuser").unwrap();
    assert_eq!(manager.get_username(), Some("testuser".to_string()));
}
