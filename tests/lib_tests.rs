// Integration tests for RustLink CLI
// Run with: cargo test --lib

#[cfg(test)]
mod cli_tests {
    use clap::Parser;
    use rustlink::cli::{Commands, Opts};

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
            Commands::Run { bootstrap } => {
                assert!(bootstrap.is_none());
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_run_command_with_bootstrap() {
        let args = vec![
            "rustlink",
            "run",
            "--bootstrap",
            "/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWTest123",
        ];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Run { bootstrap } => {
                assert!(bootstrap.is_some());
                let nodes = bootstrap.unwrap();
                assert_eq!(nodes.len(), 1);
                assert_eq!(nodes[0], "/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWTest123");
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_run_command_with_multiple_bootstrap() {
        let args = vec![
            "rustlink",
            "run",
            "-b",
            "/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWNode1",
            "-b",
            "/ip4/192.168.1.1/tcp/4002/p2p/12D3KooWNode2",
        ];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Run { bootstrap } => {
                assert!(bootstrap.is_some());
                let nodes = bootstrap.unwrap();
                assert_eq!(nodes.len(), 2);
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_tui_command() {
        let args = vec!["rustlink", "tui"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Tui => {}
            _ => panic!("Expected Tui command"),
        }
    }

    #[test]
    fn test_cli_add_command_parsing() {
        let args = vec!["rustlink", "add", "12D3KooW9ABCDEF123456789"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Add { peer_id } => {
                assert_eq!(peer_id.len(), 24); // Test input length
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_chat_command_parsing() {
        let args = vec!["rustlink", "chat", "12D3KooW9ABCDEF123456789"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Chat { peer_id } => {
                assert_eq!(peer_id.len(), 24);
            }
            _ => panic!("Expected Chat command"),
        }
    }

    #[test]
    fn test_cli_send_command_file_not_exists() {
        // This just tests parsing - file existence is checked at runtime
        let args = vec!["rustlink", "send", "nonexistent.txt", "12D3KooWTest123"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Send { file, to } => {
                assert!(!file.exists());
                assert_eq!(to, "12D3KooWTest123");
            }
            _ => panic!("Expected Send command"),
        }
    }

    #[test]
    fn test_cli_init_with_special_chars_username() {
        // Test usernames with various characters
        let args = vec!["rustlink", "init", "user_with_underscore"];
        let opts = Opts::parse_from(&args);
        match opts.command {
            Commands::Init { username } => {
                assert_eq!(username, "user_with_underscore");
            }
            _ => panic!("Expected Init command"),
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
mod main_tests {
    use rustlink::{get_data_dir, get_version, setup_logging};

    #[test]
    fn test_get_data_dir() {
        // Should return a valid path
        let dir = get_data_dir();
        assert!(dir.to_str().is_some());
    }

    #[test]
    fn test_get_version() {
        let version = get_version();
        assert!(!version.is_empty());
        // Version should be semver-like (e.g., "0.1.0")
        assert!(version.contains('.'));
    }

    #[test]
    fn test_setup_logging() {
        // Should not panic
        setup_logging();
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
            b"Hello, World!",
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

        storage
            .save_message("msg-1", "12D3KooWA", "12D3KooWB", b"Hello")
            .unwrap();
        storage
            .save_message("msg-2", "12D3KooWB", "12D3KooWA", b"Hi there")
            .unwrap();

        let messages = storage.get_messages("12D3KooWA").unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_message_content() {
        let temp_dir = create_temp_dir();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();

        let content = "Test message content 你好 🔐";
        storage
            .save_message("msg-content", "12D3KooWA", "12D3KooWB", content.as_bytes())
            .unwrap();

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
            storage
                .save_identity("12D3KooWPersist", "persistuser")
                .unwrap();
            storage.add_friend("Friend", "12D3KooWFriend").unwrap();
            storage
                .save_message("msg-persist", "12D3KooWA", "12D3KooWB", b"Test")
                .unwrap();
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

#[cfg(test)]
mod main_handler_tests {
    use rustlink::handlers::{
        handle_add, handle_chat, handle_friends, handle_init, handle_login, handle_send,
        handle_status, handle_version,
    };
    use rustlink::identity::IdentityManager;
    use rustlink::storage::Storage;
    use std::path::Path;
    use tempfile::TempDir;

    fn create_test_env() -> (Storage, IdentityManager, TempDir) {
        let temp_dir = tempfile::tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        let identity = IdentityManager::new(temp_dir.path()).unwrap();
        (storage, identity, temp_dir)
    }

    #[test]
    fn test_handle_version() {
        let version = handle_version();
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }

    #[test]
    fn test_handle_init_creates_identity() {
        let (storage, mut identity, _temp) = create_test_env();

        let result = handle_init(&storage, &mut identity, "testuser");

        assert!(result.is_ok());
        let peer_id = result.unwrap();
        assert!(!peer_id.is_empty());
        assert!(peer_id.starts_with("12D3KooW"));
    }

    #[test]
    fn test_handle_init_already_exists() {
        let (storage, mut identity, _temp) = create_test_env();

        // First init should succeed
        handle_init(&storage, &mut identity, "user1").unwrap();

        // Second init should fail
        let result = handle_init(&storage, &mut identity, "user2");
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_login_no_identity() {
        let (_storage, mut identity, _temp) = create_test_env();

        let result = handle_login(&mut identity).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_handle_login_with_identity() {
        let (storage, mut identity, _temp) = create_test_env();

        // Create identity first
        handle_init(&storage, &mut identity, "testuser").unwrap();

        // Now login should work
        let result = handle_login(&mut identity).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_handle_status_no_identity() {
        let (_storage, mut identity, _temp) = create_test_env();

        let result = handle_status(&mut identity).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_handle_status_with_identity() {
        let (storage, mut identity, _temp) = create_test_env();

        // Create identity first
        handle_init(&storage, &mut identity, "myuser").unwrap();

        // Now status should return identity info
        let result = handle_status(&mut identity).unwrap();
        assert!(result.is_some());

        let (peer_id, username) = result.unwrap();
        assert_eq!(username, "myuser");
        assert!(peer_id.starts_with("12D3KooW"));
    }

    #[test]
    fn test_handle_friends_empty() {
        let (storage, _identity, _temp) = create_test_env();

        let result = handle_friends(&storage).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_handle_add_no_session() {
        let (_storage, mut identity, _temp) = create_test_env();

        let result = handle_add(&mut identity, "12D3KooWTest");
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_add_with_session() {
        let (storage, mut identity, _temp) = create_test_env();

        // Create identity first
        handle_init(&storage, &mut identity, "testuser").unwrap();

        // Now add should work
        let result = handle_add(&mut identity, "12D3KooWFriend");
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_chat_no_session() {
        let (storage, mut identity, _temp) = create_test_env();

        let result = handle_chat(&storage, &mut identity, "12D3KooWTest");
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_chat_with_session() {
        let (storage, mut identity, _temp) = create_test_env();

        // Create identity first
        handle_init(&storage, &mut identity, "testuser").unwrap();

        // Save a message first
        storage
            .save_message("msg-1", "12D3KooWA", "12D3KooWB", b"Hello")
            .unwrap();

        // Now chat should work
        let result = handle_chat(&storage, &mut identity, "12D3KooWA").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_handle_send_no_session() {
        let (_storage, mut identity, _temp) = create_test_env();

        let result = handle_send(&mut identity, Path::new("test.txt"), "12D3KooWTest");
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_send_file_not_found() {
        let (storage, mut identity, _temp) = create_test_env();

        // Create identity first
        handle_init(&storage, &mut identity, "testuser").unwrap();

        // Try to send non-existent file
        let result = handle_send(&mut identity, Path::new("nonexistent.txt"), "12D3KooWTest");
        assert!(result.is_err());
    }
}
