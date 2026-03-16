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
fn test_save_and_get_identity() {
    let (storage, _temp) = create_test_storage();

    storage.save_identity("12D3KooWTest", "testuser").unwrap();

    let result = storage.get_identity().unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ("12D3KooWTest".to_string(), "testuser".to_string()));
}

#[test]
fn test_get_identity_empty() {
    let (storage, _temp) = create_test_storage();

    let result = storage.get_identity().unwrap();
    assert!(result.is_none());
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
fn test_save_and_get_messages() {
    let (storage, _temp) = create_test_storage();

    // Save a message
    let msg_id = storage.save_message(
        "msg-123",
        "12D3KooWSender",
        "12D3KooWReceiver",
        b"Hello world!",
    ).unwrap();

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
