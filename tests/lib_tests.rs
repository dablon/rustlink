// Integration tests for RustLink CLI
// Run with: cargo test --lib

#[cfg(test)]
mod cli_tests {
    use rustlink::cli::{Commands, Opts};
    use clap::Parser;

    #[test]
    fn test_cli_init_command() {
        let args = vec!["rustlink", "init", "testuser"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Init { username } => {
                assert_eq!(username, "testuser");
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn test_cli_login_command() {
        let args = vec!["rustlink", "login"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Login => {}
            _ => panic!("Expected Login command"),
        }
    }

    #[test]
    fn test_cli_status_command() {
        let args = vec!["rustlink", "status"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Status => {}
            _ => panic!("Expected Status command"),
        }
    }

    #[test]
    fn test_cli_friends_command() {
        let args = vec!["rustlink", "friends"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Friends => {}
            _ => panic!("Expected Friends command"),
        }
    }

    #[test]
    fn test_cli_add_command() {
        let args = vec!["rustlink", "add", "12D3KooWAbc123"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Add { peer_id } => {
                assert_eq!(peer_id, "12D3KooWAbc123");
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_chat_command() {
        let args = vec!["rustlink", "chat", "12D3KooWAbc123"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Chat { peer_id } => {
                assert_eq!(peer_id, "12D3KooWAbc123");
            }
            _ => panic!("Expected Chat command"),
        }
    }

    #[test]
    fn test_cli_send_command() {
        let args = vec!["rustlink", "send", "test.txt", "12D3KooWAbc123"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Send { file, to } => {
                assert_eq!(file.to_str().unwrap(), "test.txt");
                assert_eq!(to, "12D3KooWAbc123");
            }
            _ => panic!("Expected Send command"),
        }
    }

    #[test]
    fn test_cli_run_command() {
        let args = vec!["rustlink", "run"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Run => {}
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_version_command() {
        let args = vec!["rustlink", "version"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Version => {}
            _ => panic!("Expected Version command"),
        }
    }
}

#[cfg(test)]
mod identity_tests {
    use rustlink::identity::IdentityManager;
    use std::fs;
    use tempfile::TempDir;

    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_identity_manager_new() {
        let temp_dir = create_temp_dir();
        let result = IdentityManager::new(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_has_identity_when_no_identity() {
        let temp_dir = create_temp_dir();
        let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
        assert!(!identity.has_identity());
    }

    #[test]
    fn test_create_identity() {
        let temp_dir = create_temp_dir();
        let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
        
        let peer_id = identity.create_identity("testuser").unwrap();
        
        assert!(!peer_id.is_empty());
        assert!(peer_id.starts_with("12D3KooW"));
        assert!(identity.has_identity());
        
        let username = identity.get_username();
        assert!(username.is_some());
        assert_eq!(username.unwrap(), "testuser");
    }

    #[test]
    fn test_load_existing_identity() {
        let temp_dir = create_temp_dir();
        
        {
            let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
            identity.create_identity("testuser").unwrap();
        }
        
        let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
        let loaded = identity.load_identity().unwrap();
        
        assert!(loaded.is_some());
        let peer_id = loaded.unwrap();
        assert!(peer_id.starts_with("12D3KooW"));
    }

    #[test]
    fn test_load_nonexistent_identity() {
        let temp_dir = create_temp_dir();
        let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
        
        let result = identity.load_identity().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_identity_key_is_saved() {
        let temp_dir = create_temp_dir();
        let mut identity = IdentityManager::new(temp_dir.path()).unwrap();
        
        identity.create_identity("testuser").unwrap();
        
        let key_path = temp_dir.path().join("identity.key");
        assert!(key_path.exists());
        
        let username_path = temp_dir.path().join("username.txt");
        assert!(username_path.exists());
    }

    #[test]
    fn test_different_users_different_peer_ids() {
        let temp_dir1 = create_temp_dir();
        let temp_dir2 = create_temp_dir();
        
        let mut identity1 = IdentityManager::new(temp_dir1.path()).unwrap();
        let mut identity2 = IdentityManager::new(temp_dir2.path()).unwrap();
        
        let peer_id1 = identity1.create_identity("user1").unwrap();
        let peer_id2 = identity2.create_identity("user2").unwrap();
        
        assert_ne!(peer_id1, peer_id2);
    }
}

#[cfg(test)]
mod storage_tests {
    use rustlink::storage::Storage;
    use tempfile::TempDir;

    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn test_storage_new() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let result = Storage::new(&db_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_and_get_identity() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        let result = storage.save_identity("12D3KooWTest123", "testuser");
        assert!(result.is_ok());
        
        let identity = storage.get_identity().unwrap();
        assert!(identity.is_some());
        
        let (peer_id, username) = identity.unwrap();
        assert_eq!(peer_id, "12D3KooWTest123");
        assert_eq!(username, "testuser");
    }

    #[test]
    fn test_get_identity_empty() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        let identity = storage.get_identity().unwrap();
        assert!(identity.is_none());
    }

    #[test]
    fn test_add_friend() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        let result = storage.add_friend("friend1", "12D3KooWFriend123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_message() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        let result = storage.save_message(
            "msg-123",
            "12D3KooWSender",
            "12D3KooWReceiver",
            b"Hello, World!"
        );
        assert!(result.is_ok());
        
        let msg_id = result.unwrap();
        assert!(msg_id > 0);
    }

    #[test]
    fn test_get_messages() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        storage.save_message("msg-1", "12D3KooWA", "12D3KooWB", b"Hello").unwrap();
        storage.save_message("msg-2", "12D3KooWB", "12D3KooWA", b"Hi there").unwrap();
        
        let messages = storage.get_messages("12D3KooWA").unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_message_content() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        
        let content = "Test message content 你好 🔐";
        storage.save_message(
            "msg-content",
            "12D3KooWA",
            "12D3KooWB",
            content.as_bytes()
        ).unwrap();
        
        let messages = storage.get_messages("12D3KooWA").unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, content);
    }

    #[test]
    fn test_storage_persists() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        
        {
            let storage = Storage::new(&db_path).unwrap();
            storage.save_identity("12D3KooWPersist", "persistuser").unwrap();
            storage.add_friend("Friend", "12D3KooWFriend").unwrap();
            storage.save_message("msg-persist", "12D3KooWA", "12D3KooWB", b"Test").unwrap();
        }
        
        {
            let storage = Storage::new(&db_path).unwrap();
            let identity = storage.get_identity().unwrap();
            assert!(identity.is_some());
            
            let messages = storage.get_messages("12D3KooWA").unwrap();
            assert_eq!(messages.len(), 1);
        }
    }
}
