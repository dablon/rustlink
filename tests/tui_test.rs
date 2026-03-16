use rustlink::tui::{Screen, TuiApp};

#[allow(unused_imports)]
use rustlink::tui;

#[allow(unused_imports)]
use rustlink::storage::Storage;

use tempfile::NamedTempFile;

fn create_test_storage() -> (Storage, NamedTempFile) {
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();
    let storage = Storage::new(path).unwrap();
    (storage, temp_file)
}

#[test]
fn test_tui_app_new() {
    let (storage, _temp) = create_test_storage();

    let app = TuiApp::new(storage);

    assert_eq!(app.current_screen, Screen::FriendsList);
    assert!(app.selected_friend.is_none());
    assert!(app.messages.is_empty());
    assert!(app.input_buffer.is_empty());
    assert_eq!(app.scroll_offset, 0);
}

#[test]
fn test_tui_app_new_sets_default_values() {
    let (storage, _temp) = create_test_storage();

    let app = TuiApp::new(storage);

    // Verify all default values
    assert_eq!(app.current_screen, Screen::FriendsList);
    assert_eq!(app.scroll_offset, 0);
    assert_eq!(app.input_buffer.len(), 0);
}

#[test]
fn test_tui_app_select_friend() {
    let (storage, _temp) = create_test_storage();
    let mut app = TuiApp::new(storage);

    app.selected_friend = Some("Alice".to_string());

    assert!(app.selected_friend.is_some());
    assert_eq!(app.selected_friend.unwrap(), "Alice");
}

#[test]
fn test_tui_app_input_buffer() {
    let (storage, _temp) = create_test_storage();
    let mut app = TuiApp::new(storage);

    app.input_buffer = "Hello, world!".to_string();

    assert_eq!(app.input_buffer, "Hello, world!");
}

#[test]
fn test_tui_app_scroll_offset() {
    let (storage, _temp) = create_test_storage();
    let mut app = TuiApp::new(storage);

    app.scroll_offset = 10;

    assert_eq!(app.scroll_offset, 10);
}

#[test]
fn test_tui_app_set_screen_chat() {
    let (storage, _temp) = create_test_storage();
    let mut app = TuiApp::new(storage);

    app.current_screen = Screen::Chat;

    assert_eq!(app.current_screen, Screen::Chat);
}

#[test]
fn test_tui_app_set_screen_friends_list() {
    let (storage, _temp) = create_test_storage();
    let mut app = TuiApp::new(storage);

    // Start in Chat, switch back to FriendsList
    app.current_screen = Screen::Chat;
    app.current_screen = Screen::FriendsList;

    assert_eq!(app.current_screen, Screen::FriendsList);
}

#[test]
fn test_tui_app_load_messages() {
    let (storage, _temp) = create_test_storage();

    // Save some messages first
    storage.save_message("msg-1", "Alice", "Bob", b"Hello").unwrap();
    storage.save_message("msg-2", "Bob", "Alice", b"Hi there").unwrap();

    let mut app = TuiApp::new(storage);

    // Load messages from Alice
    app.load_messages("Alice").unwrap();

    assert_eq!(app.messages.len(), 2);
}

#[test]
fn test_tui_app_load_messages_empty() {
    let (storage, _temp) = create_test_storage();

    let mut app = TuiApp::new(storage);

    // Load messages for unknown peer
    app.load_messages("Unknown").unwrap();

    assert!(app.messages.is_empty());
}

#[test]
fn test_screen_enum() {
    let friends = Screen::FriendsList;
    let chat = Screen::Chat;

    assert_ne!(friends, chat);

    // Test clone
    let friends_clone = friends.clone();
    assert_eq!(friends, friends_clone);

    // Test debug
    let debug_friends = format!("{:?}", friends);
    assert!(debug_friends.contains("FriendsList"));

    let debug_chat = format!("{:?}", chat);
    assert!(debug_chat.contains("Chat"));
}

#[test]
fn test_screen_partial_eq() {
    assert_eq!(Screen::FriendsList, Screen::FriendsList);
    assert_eq!(Screen::Chat, Screen::Chat);
    assert_ne!(Screen::FriendsList, Screen::Chat);
}
