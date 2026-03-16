use rustlink::filetransfer::{calculate_checksum, split_into_chunks, verify_integrity, TransferProgress, TransferStatus, FileTransferMessage, CHUNK_SIZE};

#[test]
fn test_checksum() {
    let data = b"Hello, World!";
    let checksum = calculate_checksum(data);
    assert_eq!(checksum.len(), 64);
}

#[test]
fn test_checksum_empty() {
    let data = b"";
    let checksum = calculate_checksum(data);
    assert_eq!(checksum.len(), 64);
    assert_eq!(checksum, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[test]
fn test_chunks() {
    let data = vec![1u8; 100 * 1024];
    let chunks = split_into_chunks(&data);
    assert_eq!(chunks.len(), 2);
}

#[test]
fn test_chunks_empty() {
    let data: Vec<u8> = vec![];
    let chunks = split_into_chunks(&data);
    assert_eq!(chunks.len(), 0);
}

#[test]
fn test_chunks_single() {
    let data = vec![1u8; 1024];
    let chunks = split_into_chunks(&data);
    assert_eq!(chunks.len(), 1);
}

#[test]
fn test_verify_integrity_valid() {
    let data = b"Test data";
    let checksum = calculate_checksum(data);
    assert!(verify_integrity(data, &checksum));
}

#[test]
fn test_verify_integrity_invalid() {
    let data = b"Test data";
    let wrong_checksum = "0".repeat(64);
    assert!(!verify_integrity(data, &wrong_checksum));
}

#[test]
fn test_transfer_progress_new() {
    let progress = TransferProgress::new(
        "file-123".to_string(),
        "test.txt".to_string(),
        1000,
        10,
    );
    assert_eq!(progress.file_id, "file-123");
    assert_eq!(progress.filename, "test.txt");
    assert_eq!(progress.total_size, 1000);
    assert_eq!(progress.total_chunks, 10);
    assert_eq!(progress.received_chunks, 0);
    assert_eq!(progress.status, TransferStatus::Pending);
}

#[test]
fn test_transfer_progress_add_chunk() {
    let mut progress = TransferProgress::new(
        "file-123".to_string(),
        "test.txt".to_string(),
        100,
        2,
    );

    progress.add_chunk(vec![1, 2, 3]);
    assert_eq!(progress.received_chunks, 1);
    assert_eq!(progress.status, TransferStatus::InProgress);

    progress.add_chunk(vec![4, 5, 6]);
    assert_eq!(progress.received_chunks, 2);
    assert_eq!(progress.status, TransferStatus::Completed);
}

#[test]
fn test_progress_percent() {
    let mut progress = TransferProgress::new("file-123".to_string(), "test.txt".to_string(), 100, 4);

    assert_eq!(progress.progress_percent(), 0.0);

    progress.add_chunk(vec![1]);
    assert_eq!(progress.progress_percent(), 25.0);

    progress.add_chunk(vec![1]);
    assert_eq!(progress.progress_percent(), 50.0);

    progress.add_chunk(vec![1]);
    assert_eq!(progress.progress_percent(), 75.0);

    progress.add_chunk(vec![1]);
    assert_eq!(progress.progress_percent(), 100.0);
}

#[test]
fn test_progress_percent_zero_chunks() {
    let progress = TransferProgress::new("file-123".to_string(), "test.txt".to_string(), 0, 0);
    assert_eq!(progress.progress_percent(), 0.0);
}

#[test]
fn test_mark_failed() {
    let mut progress = TransferProgress::new("file-123".to_string(), "test.txt".to_string(), 100, 2);
    progress.add_chunk(vec![1]);
    progress.mark_failed("Connection lost");
    assert_eq!(progress.status, TransferStatus::Failed("Connection lost".to_string()));
}

#[test]
fn test_file_transfer_message_new_request() {
    let data = b"test file content";
    let msg = FileTransferMessage::new_request("test.txt", data.len() as u64, data);

    match msg {
        FileTransferMessage::Request { file_id, filename, file_size, checksum } => {
            assert!(!file_id.is_empty());
            assert_eq!(filename, "test.txt");
            assert_eq!(file_size, data.len() as u64);
            assert_eq!(checksum.len(), 64);
        }
        _ => panic!("Expected Request"),
    }
}

#[test]
fn test_file_transfer_message_new_accept() {
    let msg = FileTransferMessage::new_accept("file-123");
    match msg {
        FileTransferMessage::Accept { file_id } => {
            assert_eq!(file_id, "file-123");
        }
        _ => panic!("Expected Accept"),
    }
}

#[test]
fn test_file_transfer_message_new_reject() {
    let msg = FileTransferMessage::new_reject("file-123", "Not enough space");
    match msg {
        FileTransferMessage::Reject { file_id, reason } => {
            assert_eq!(file_id, "file-123");
            assert_eq!(reason, "Not enough space");
        }
        _ => panic!("Expected Reject"),
    }
}

#[test]
fn test_file_transfer_message_new_chunk() {
    let msg = FileTransferMessage::new_chunk("file-123", 0, vec![1, 2, 3]);
    match msg {
        FileTransferMessage::Chunk { file_id, chunk_index, data } => {
            assert_eq!(file_id, "file-123");
            assert_eq!(chunk_index, 0);
            assert_eq!(data, vec![1, 2, 3]);
        }
        _ => panic!("Expected Chunk"),
    }
}

#[test]
fn test_file_transfer_message_new_complete() {
    let msg = FileTransferMessage::new_complete("file-123", "abc123");
    match msg {
        FileTransferMessage::Complete { file_id, checksum } => {
            assert_eq!(file_id, "file-123");
            assert_eq!(checksum, "abc123");
        }
        _ => panic!("Expected Complete"),
    }
}

#[test]
fn test_file_transfer_message_new_failed() {
    let msg = FileTransferMessage::new_failed("file-123", "Timeout");
    match msg {
        FileTransferMessage::Failed { file_id, reason } => {
            assert_eq!(file_id, "file-123");
            assert_eq!(reason, "Timeout");
        }
        _ => panic!("Expected Failed"),
    }
}

#[test]
fn test_file_transfer_message_serialize_roundtrip() {
    let original = FileTransferMessage::new_request("test.txt", 100, b"content");
    let data = original.serialize().unwrap();
    let decoded = FileTransferMessage::deserialize(&data).unwrap();

    match decoded {
        FileTransferMessage::Request { filename, .. } => {
            assert_eq!(filename, "test.txt");
        }
        _ => panic!("Expected Request"),
    }
}

#[test]
fn test_transfer_status_equality() {
    let s1 = TransferStatus::Pending;
    let s2 = TransferStatus::Pending;
    assert_eq!(s1, s2);

    let s3 = TransferStatus::InProgress;
    assert_ne!(s1, s3);
}
