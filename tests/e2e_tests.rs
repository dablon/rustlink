// E2E tests for RustLink
// Tests the full CLI workflow

use std::process::Command;
use std::fs;
use tempfile::TempDir;

fn rustlink_binary() -> String {
    // Use the release binary
    let path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default()
        .join("rustlink");
    
    // Try debug if release doesn't exist
    if !path.exists() {
        return "cargo run --".to_string();
    }
    
    path.to_string_lossy().to_string()
}

#[test]
fn test_e2e_init_and_status() {
    let temp_dir = tempfile::tempdir().unwrap();
    let data_dir = temp_dir.path();
    
    // Run init
    let output = Command::new("cargo")
        .args(&["run", "--", "init", "testuser"])
        .current_dir("/workspace/rustlink")
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink init");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Identidad creada") || stdout.contains("created"));
    
    // Run status
    let output = Command::new("cargo")
        .args(&["run", "--", "status"])
        .current_dir("/workspace/rustlink")
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
        .current_dir("/workspace/rustlink")
        .output()
        .expect("Failed to run rustlink --version");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rustlink") || stdout.contains("RustLink"));
}

#[test]
fn test_e2e_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .current_dir("/workspace/rustlink")
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
        .current_dir("/workspace/rustlink")
        .env("HOME", data_dir)
        .output()
        .expect("Failed to init");
    
    // Then check friends
    let output = Command::new("cargo")
        .args(&["run", "--", "friends"])
        .current_dir("/workspace/rustlink")
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
        .current_dir("/workspace/rustlink")
        .env("HOME", data_dir)
        .output()
        .expect("Failed to init");
    
    // Login should work
    let output = Command::new("cargo")
        .args(&["run", "--", "login"])
        .current_dir("/workspace/rustlink")
        .env("HOME", data_dir)
        .output()
        .expect("Failed to run rustlink login");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Sesión iniciada") || stdout.contains("logged in"));
}
