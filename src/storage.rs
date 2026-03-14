use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    pub peer_id: String,
    pub username: String,
    pub added_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub from: String,
    pub to: String,
    pub content: String,
    pub sent_at: String,
    pub received: bool,
    pub delivered: bool,
}

#[derive(Clone)]
pub struct Storage {
    conn: Arc<Mutex<Connection>>,
}

impl Storage {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Create tables
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS identity (
                peer_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE TABLE IF NOT EXISTS friends (
                peer_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                added_at TEXT DEFAULT CURRENT_TIMESTAMP,
                status TEXT DEFAULT 'pending'
            );
            
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                msg_uuid TEXT UNIQUE,
                from_peer TEXT NOT NULL,
                to_peer TEXT NOT NULL,
                content BLOB NOT NULL,
                sent_at TEXT DEFAULT CURRENT_TIMESTAMP,
                received INTEGER DEFAULT 0,
                delivered INTEGER DEFAULT 0
            );
            
            CREATE INDEX IF NOT EXISTS idx_messages_from ON messages(from_peer);
            CREATE INDEX IF NOT EXISTS idx_messages_to ON messages(to_peer);
            "
        )?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
    
    pub fn save_identity(&self, peer_id: &str, username: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO identity (peer_id, username) VALUES (?1, ?2)",
            params![peer_id, username],
        )?;
        Ok(())
    }
    
    pub fn get_identity(&self) -> Result<Option<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT peer_id, username FROM identity LIMIT 1")?;
        let result = stmt.query_row([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        });
        
        match result {
            Ok(id) => Ok(Some(id)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    pub fn add_friend(&self, username: &str, peer_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO friends (peer_id, username, status) VALUES (?1, ?2, 'pending')",
            params![peer_id, username],
        )?;
        Ok(())
    }
    
    pub fn get_friends(&self) -> Result<Vec<Friend>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT peer_id, username, added_at, status FROM friends WHERE status = 'accepted'"
        )?;
        
        let friends = stmt.query_map([], |row| {
            Ok(Friend {
                peer_id: row.get(0)?,
                username: row.get(1)?,
                added_at: row.get(2)?,
                status: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(friends)
    }
    
    pub fn get_pending_friends(&self) -> Result<Vec<Friend>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT peer_id, username, added_at, status FROM friends WHERE status = 'pending'"
        )?;
        
        let friends = stmt.query_map([], |row| {
            Ok(Friend {
                peer_id: row.get(0)?,
                username: row.get(1)?,
                added_at: row.get(2)?,
                status: row.get(3)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(friends)
    }
    
    pub fn accept_friend(&self, peer_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE friends SET status = 'accepted' WHERE peer_id = ?1",
            params![peer_id],
        )?;
        Ok(())
    }
    
    pub fn remove_friend(&self, peer_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM friends WHERE peer_id = ?1", params![peer_id])?;
        Ok(())
    }
    
    pub fn save_message(&self, msg_uuid: &str, from: &str, to: &str, content: &[u8]) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO messages (msg_uuid, from_peer, to_peer, content) VALUES (?1, ?2, ?3, ?4)",
            params![msg_uuid, from, to, content],
        )?;
        Ok(conn.last_insert_rowid())
    }
    
    pub fn get_messages(&self, peer_id: &str) -> Result<Vec<Message>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, from_peer, to_peer, content, sent_at, received, delivered 
             FROM messages 
             WHERE from_peer = ?1 OR to_peer = ?1
             ORDER BY sent_at DESC
             LIMIT 100"
        )?;
        
        let messages = stmt.query_map([peer_id], |row| {
            let content_bytes: Vec<u8> = row.get(3)?;
            let content = String::from_utf8_lossy(&content_bytes).to_string();
            
            Ok(Message {
                id: row.get(0)?,
                from: row.get(1)?,
                to: row.get(2)?,
                content,
                sent_at: row.get(4)?,
                received: row.get::<_, i32>(5)? != 0,
                delivered: row.get::<_, i32>(6)? != 0,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(messages)
    }
    
    pub fn mark_received(&self, msg_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE messages SET received = 1 WHERE id = ?1",
            params![msg_id],
        )?;
        Ok(())
    }
    
    pub fn mark_delivered(&self, msg_id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE messages SET delivered = 1 WHERE id = ?1",
            params![msg_id],
        )?;
        Ok(())
    }
}
