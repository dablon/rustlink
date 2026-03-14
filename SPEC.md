# RustLink — P2P CLI Social App Specification

## 1. Project Overview

**Project Name:** RustLink
**Type:** CLI Application (Rust)
**Core Functionality:** Decentralized P2P social networking app for CLI - allows users to chat, share files, and manage friends without servers or port forwarding.
**Target Users:** Privacy-conscious developers, CLI enthusiasts, decentralized communication advocates
**Repository:** https://github.com/dablon/rustlink

## 2. Architecture

```
┌─────────────────────────────────────────┐
│           CLI Layer (clap + ratatui)   │
│  login, register, friends, chat, send  │
├─────────────────────────────────────────┤
│           Core Logic Layer              │
│  User Identity, Friend Manager,         │
│  Messaging, File Transfer              │
├─────────────────────────────────────────┤
│           Network Layer (libp2p)        │
│  mDNS, Kademlia DHT, QUIC, Noise,    │
│  Relay (hole punching fallback)        │
├─────────────────────────────────────────┤
│           Storage Layer (rusqlite)      │
│  Local SQLite for messages & friends   │
└─────────────────────────────────────────┘
```

## 3. Tech Stack

- **Language:** Rust (stable)
- **CLI:** clap + ratatui (for interactive TUI)
- **Networking:** libp2p 0.56 (with full features)
- **Storage:** rusqlite
- **Crypto:** libp2p's built-in keys + Ed25519
- **Async:** tokio

## 4. Functional Specification

### 4.1 Identity Management
- Generate cryptographic keypair on first run (Ed25519)
- Store peer ID and private key locally (protobuf encoded)
- Username registration (stored in local DB, announced via DHT)
- Peer ID format: `12D3KooW...` (similar to Bitcoin addresses)

### 4.2 Discovery
- **mDNS:** Auto-discover peers on local network
- **Kademlia DHT:** Global peer discovery by username/peer ID
- **Bootstrap nodes:** Initial peers to join the network (configurable)

### 4.3 Friends System
- Add friend by peer ID or username
- Accept/reject friend requests
- View online/offline status
- Block/remove friends

### 4.4 Messaging Protocol
- **Protocol:** `/rustlink/chat/1.0.0` (custom libp2p protocol)
- Direct encrypted messages to friends
- Message persistence (stored locally in SQLite)
- Delivery receipts (Sent ✓, Delivered ✓✓, Read ✓✓)
- Offline message storage (relay through DHT)

### 4.5 File Transfer Protocol
- **Protocol:** `/rustlink/filetransfer/1.0.0`
- Send files to friends
- Chunked transfer (64KB chunks)
- SHA256 hash verification for integrity
- Progress indicator via indicatif
- Future: resume support

### 4.6 CLI Commands
```
rustlink register <username>     # Create identity
rustlink login                   # Load existing identity
rustlink status                  # Show online status
rustlink friends                 # List friends
rustlink add-friend <peer_id>   # Add friend by peer ID
rustlink chat <peer_id>         # Open chat with friend
rustlink send-file <file> <peer_id>  # Send file to friend
rustlink run                     # Start P2P daemon
rustlink version                 # Show version
```

## 5. Connection Flow

```
Usuario A                              Usuario B
  │                                      │
  │ 1. Genera PeerID (clave pública)    │
  │ 2. Se anuncia en la DHT             │
  │                                      │
  │ 3. Busca PeerID de Usuario B ──────► │
  │                                      │
  │ 4. Intenta conexión directa          │
  │    └─ Si falla NAT: Hole Punching    │
  │    └─ Si falla: Relay Node           │
  │                                      │
  │◄──── 5. Canal seguro establecido ────►│
  │    (Noise Protocol / TLS 1.3)       │
  │                                      │
  │◄──── 6. Chat / Archivos ────────────►│
```

## 6. Data Flow

1. **Startup:** Load/create keypair → Initialize libp2p → Connect to bootstrap → Discover peers
2. **Add Friend:** Query DHT for peer ID → Initiate connection → Exchange info → Store locally
3. **Chat:** Open substream → Encrypt with Noise → Send/receive messages → Store locally
4. **File Transfer:** Negotiate via chat → Open stream → Chunk and transfer with hash verification

## 7. Storage Schema (SQLite)

```sql
-- Users (self)
CREATE TABLE identity (
    peer_id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    created_at TIMESTAMP
);

-- Friends
CREATE TABLE friends (
    peer_id TEXT PRIMARY KEY,
    username TEXT,
    added_at TIMESTAMP,
    status TEXT -- 'pending', 'accepted', 'blocked'
);

-- Messages
CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    msg_uuid TEXT UNIQUE,
    from_peer TEXT NOT NULL,
    to_peer TEXT NOT NULL,
    content BLOB NOT NULL,
    sent_at TIMESTAMP,
    received INTEGER DEFAULT 0,
    delivered INTEGER DEFAULT 0
);
```

## 8. Network Configuration

- **Transport:** QUIC (UDP) with TCP fallback
- **Encryption:** Noise protocol (libp2p-noise)
- **Discovery:** mDNS (local) + Kademlia (global)
- **NAT:** Auto-relay when direct connection fails
- **Ports:** 0 (OS-assigned) — no manual config needed

## 9. Security

- E2E encryption via Noise protocol
- Keys stored locally (encrypted in future with passphrase)
- No plaintext messages ever sent
- Peer verification via peer IDs
- SHA256 integrity verification for file transfers

## 10. Acceptance Criteria

- [x] User can generate identity and set username
- [x] User can discover peers on local network via mDNS
- [x] User can find peers globally via DHT
- [x] User can add friends and see their online status
- [x] User can send/receive encrypted messages
- [x] User can transfer files to friends
- [x] All data persists in local SQLite
- [x] CLI is interactive and user-friendly
- [x] No port forwarding required (works behind NAT)
- [x] Graceful shutdown saves state
- [x] GitHub Actions CI pipeline configured

## 11. Protocol Versions

- Chat: `/rustlink/chat/1.0.0`
- File Transfer: `/rustlink/filetransfer/1.0.0`

## 12. File Structure

```
/workspace/rustlink/
├── .github/workflows/ci.yml     # GitHub Actions
├── src/
│   ├── main.rs                  # Entry point
│   ├── cli.rs                   # CLI commands
│   ├── identity.rs              # Identity management
│   ├── storage.rs               # SQLite storage
│   ├── network.rs               # P2P networking
│   ├── messaging.rs             # Messaging service
│   ├── protocol.rs              # Chat protocol
│   └── filetransfer.rs          # File transfer
├── Cargo.toml
├── README.md
└── SPEC.md
```
