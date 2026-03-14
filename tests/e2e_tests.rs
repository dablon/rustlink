// E2E tests for RustLink
// Tests the full CLI workflow

use std::process::Command;

fn project_root() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

#[test]
fn test_e2e_init_and_status() {
    let temp_dir = tempfile::tempdir().unwrap();
    let data_dir = temp_dir.path();
    
    // Run init
    let output = Command::new("cargo")
        .args(&["run", "--", "init", "testuser"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink init");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Identidad creada") || stdout.contains("created"));
    
    // Run status
    let output = Command::new("cargo")
        .args(&["run", "--", "status"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink status");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("testuser") || stdout.contains("PeerID"));
}

#[test]
fn test_e2e_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .current_dir(project_root())
        .output()
        .expect("Failed to run rustlink --version");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rustlink") || stdout.contains("RustLink"));
}

#[test]
fn test_e2e_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .current_dir(project_root())
        .output()
        .expect("Failed to run rustlink --help");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage") || stdout.contains("Commands"));
}

#[test]
fn test_e2e_friends_empty() {
    let temp_dir = tempfile::tempdir().unwrap();
    let data_dir = temp_dir.path();
    
    // Create identity first
    Command::new("cargo")
        .args(&["run", "--", "init", "testuser"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to init");
    
    // Then check friends
    let output = Command::new("cargo")
        .args(&["run", "--", "friends"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink friends");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No tienes amigos") || stdout.contains("amigos"));
}

#[test]
fn test_e2e_login_after_init() {
    let temp_dir = tempfile::tempdir().unwrap();
    let data_dir = temp_dir.path();
    
    // Init
    Command::new("cargo")
        .args(&["run", "--", "init", "testuser2"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to init");
    
    // Login should work
    let output = Command::new("cargo")
        .args(&["run", "--", "login"])
        .current_dir(project_root())
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink login");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sesión iniciada") || stdout.contains("logged in"));
}
