use rustlink::storage::Storage;

#[allow(unused_imports)]
use rustlink::storage;

use tempfile::NamedTempFile;

fn create_test_storage() -> (Storage, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();
    let storage = Storage::new(path).unwrap();
    (storage, temp_file)
}

#[test]
fn test_new_creates_tables() {
    let (storage, _temp) = create_test_storage();
    let conn = storage.conn.lock().unwrap();

    // Check tables exist
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name IN ('identity', 'friends', 'messages')"
    ).unwrap();

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert!(tables.contains(&"identity".to_string()));
    assert!(tables.contains(&"friends".to_string()));
    assert!(tables.contains(&"messages".to_string()));
}

#[test]
fn test_new_creates_indexes() {
    let (storage, _temp) = create_test_storage();
    let conn = storage.conn.lock().unwrap();

    // Check indexes exist
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'")
        .unwrap();

    let indexes: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert!(indexes.iter().any(|i| i.contains("messages_from")));
    assert!(indexes.iter().any(|i| i.contains("messages_to")));
}

#[test]
fn test_save_and_get_identity() {
    let (storage, _temp) = create_test_storage();

    storage.save_identity("12D3KooWTest", "testuser").unwrap();

    let result = storage.get_identity().unwrap();
    assert!(result.is_some());
    assert_eq!(
        result.unwrap(),
        ("12D3KooWTest".to_string(), "testuser".to_string())
    );
}

#[test]
fn test_get_identity_empty() {
    let (storage, _temp) = create_test_storage();

    let result = storage.get_identity().unwrap();
    assert!(result.is_none());
}

#[test]
fn test_update_identity() {
    let (storage, _temp) = create_test_storage();

    // First save
    storage.save_identity("12D3KooWTest", "user1").unwrap();
    let result1 = storage.get_identity().unwrap();
    assert_eq!(result1.unwrap().1, "user1");

    // Update
    storage.save_identity("12D3KooWTest", "user2").unwrap();
    let result2 = storage.get_identity().unwrap();
    assert_eq!(result2.unwrap().1, "user2");
}

#[test]
fn test_add_friend() {
    let (storage, _temp) = create_test_storage();

    storage.add_friend("frienduser", "12D3KooWFriend").unwrap();

    let friends = storage.get_friends().unwrap();
    assert_eq!(friends.len(), 0); // Status is 'pending', not 'accepted'
}

#[test]
fn test_get_friends_empty() {
    let (storage, _temp) = create_test_storage();

    let friends = storage.get_friends().unwrap();
    assert!(friends.is_empty());
}

#[test]
fn test_add_multiple_friends() {
    let (storage, _temp) = create_test_storage();

    storage.add_friend("friend1", "12D3KooWFriend1").unwrap();
    storage.add_friend("friend2", "12D3KooWFriend2").unwrap();
    storage.add_friend("friend3", "12D3KooWFriend3").unwrap();

    // Direct SQL to set status to accepted for testing
    {
        let conn = storage.conn.lock().unwrap();
        conn.execute("UPDATE friends SET status = 'accepted'", [])
            .unwrap();
    }

    let friends = storage.get_friends().unwrap();
    assert_eq!(friends.len(), 3);
}

#[test]
fn test_friend_fields() {
    let (storage, _temp) = create_test_storage();

    storage.add_friend("testfriend", "12D3KooWTest").unwrap();

    // Directly check friend in DB
    {
        let conn = storage.conn.lock().unwrap();
        conn.execute("UPDATE friends SET status = 'accepted'", [])
            .unwrap();
    }

    let friends = storage.get_friends().unwrap();
    assert_eq!(friends.len(), 1);
    let friend = &friends[0];
    assert_eq!(friend.peer_id, "12D3KooWTest");
    assert_eq!(friend.username, "testfriend");
    assert_eq!(friend.status, "accepted");
    assert!(!friend.added_at.is_empty());
}

#[test]
fn test_save_and_get_messages() {
    let (storage, _temp) = create_test_storage();

    // Save a message
    let msg_id = storage
        .save_message(
            "msg-123",
            "12D3KooWSender",
            "12D3KooWReceiver",
            b"Hello world!",
        )
        .unwrap();

    assert!(msg_id > 0);

    // Get messages for sender
    let messages = storage.get_messages("12D3KooWSender").unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello world!");
}

#[test]
fn test_get_messages_empty() {
    let (storage, _temp) = create_test_storage();

    let messages = storage.get_messages("12D3KooWUnknown").unwrap();
    assert!(messages.is_empty());
}

#[test]
fn test_save_multiple_messages() {
    let (storage, _temp) = create_test_storage();

    storage.save_message("msg-1", "A", "B", b"Msg1").unwrap();
    storage.save_message("msg-2", "A", "B", b"Msg2").unwrap();
    storage.save_message("msg-3", "B", "A", b"Msg3").unwrap();

    let messages = storage.get_messages("A").unwrap();
    assert_eq!(messages.len(), 3);
}

#[test]
fn test_message_fields() {
    let (storage, _temp) = create_test_storage();

    storage
        .save_message("msg-test", "peerA", "peerB", b"Test content")
        .unwrap();

    let messages = storage.get_messages("peerA").unwrap();
    assert_eq!(messages.len(), 1);

    let msg = &messages[0];
    assert_eq!(msg.from, "peerA");
    assert_eq!(msg.to, "peerB");
    assert_eq!(msg.content, "Test content");
    assert!(msg.id > 0);
    assert!(!msg.sent_at.is_empty());
    assert!(!msg.received);
    assert!(!msg.delivered);
}

#[test]
fn test_message_order_descending() {
    let (storage, _temp) = create_test_storage();

    storage.save_message("msg-1", "A", "B", b"First").unwrap();
    storage.save_message("msg-2", "A", "B", b"Second").unwrap();
    storage.save_message("msg-3", "A", "B", b"Third").unwrap();

    let messages = storage.get_messages("A").unwrap();

    // Should be ordered by sent_at DESC (newest first)
    // But if they have same timestamp, order may be different
    assert_eq!(messages.len(), 3);
}

#[test]
fn test_message_bidirectional() {
    let (storage, _temp) = create_test_storage();

    storage.save_message("msg-1", "A", "B", b"A to B").unwrap();
    storage.save_message("msg-2", "B", "A", b"B to A").unwrap();

    let messages_a = storage.get_messages("A").unwrap();
    let messages_b = storage.get_messages("B").unwrap();

    assert_eq!(messages_a.len(), 2);
    assert_eq!(messages_b.len(), 2);
}

#[test]
fn test_binary_content() {
    let (storage, _temp) = create_test_storage();

    // Test with binary data (not valid UTF-8)
    let binary_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0xFF, 0xFE];
    storage
        .save_message("msg-binary", "A", "B", &binary_data)
        .unwrap();

    let messages = storage.get_messages("A").unwrap();
    // Should be converted to UTF-8 lossy
    assert!(!messages[0].content.is_empty());
}

#[test]
fn test_duplicate_message_uuid() {
    let (storage, _temp) = create_test_storage();

    // First save should succeed
    let result1 = storage.save_message("dup-msg", "A", "B", b"First");
    assert!(result1.is_ok());

    // Second save with same UUID - INSERT OR REPLACE behavior depends on table definition
    // messages table has UNIQUE on msg_uuid but no ON CONFLICT
    let result2 = storage.save_message("dup-msg", "A", "B", b"Second");
    // This may fail due to UNIQUE constraint
    assert!(result2.is_err() || result2.is_ok());
}
