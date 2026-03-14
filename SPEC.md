# RustLink — P2P CLI Social App Specification

## 1. Project Overview

**Project Name:** RustLink
**Type:** CLI Application (Rust)
**Core Functionality:** Decentralized P2P social networking app for CLI - allows users to chat, share files, and manage friends without servers or port forwarding.
**Target Users:** Privacy-conscious developers, CLI enthusiasts, decentralized communication advocates

## 2. Architecture

```
┌─────────────────────────────────────────┐
│           CLI Layer (clap)              │
│  login, register, friends, chat, send  │
├─────────────────────────────────────────┤
│           Core Logic Layer              │
│  User Identity, Friend Manager,         │
│  Message Handler, File Transfer         │
├─────────────────────────────────────────┤
│           Network Layer (libp2p)        │
│  mDNS, Kademlia DHT, QUIC, Noise,       │
│  Relay (hole punching fallback)         │
├─────────────────────────────────────────┤
│           Storage Layer (rusqlite)      │
│  Local SQLite for messages & friends    │
└─────────────────────────────────────────┘
```

## 3. Tech Stack

- **Language:** Rust (stable)
- **CLI:** clap + ratatui (for interactive TUI)
- **Networking:** libp2p (with customizations)
- **Storage:** rusqlite
- **Crypto:** libp2p's built-in keys + x25519-dalek
- **Async:** tokio

## 4. Functional Specification

### 4.1 Identity Management
- Generate cryptographic keypair on first run (Ed25519)
- Store peer ID and private key locally
- Username registration (stored in local DB, announced via DHT)

### 4.2 Discovery
- **mDNS:** Auto-discover peers on local network
- **Kademlia DHT:** Global peer discovery by username/peer ID
- **Bootstrap nodes:** Initial peers to join the network

### 4.3 Friends System
- Add friend by username or peer ID
- Accept/reject friend requests
- View online/offline status
- Block/remove friends

### 4.4 Messaging
- Direct encrypted messages to friends
- Message persistence (stored locally)
- Read receipts (optional)
- Offline message storage (relay through DHT)

### 4.5 File Transfer
- Send files to friends
- Chunked transfer via libp2p
- Progress indicator
- Resume support (future)

### 4.6 CLI Commands
```
rustlink register <username>     # Create identity
rustlink login                    # Load existing identity
rustlink add-friend <username>    # Add friend
rustlink friends                  # List friends
rustlink chat <username>          # Open chat
rustlink send <file> <username>   # Send file
rustlink status                   # Show online status
rustlink quit                      # Exit gracefully
```

## 5. Data Flow

1. **Startup:** Load/create keypair → Initialize libp2p → Connect to bootstrap → Discover peers
2. **Add Friend:** Query DHT for username → Get peer ID → Initiate connection → Exchange info
3. **Chat:** Open substream → Encrypt with Noise → Send/receive messages → Store locally
4. **File Transfer:** Negotiate via chat → Open stream → Chunk and transfer

## 6. Storage Schema (SQLite)

```sql
-- Users (self)
CREATE TABLE identity (
    peer_id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    private_key BLOB,
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
    peer_id TEXT,
    content BLOB,
    sent_at TIMESTAMP,
    received BOOLEAN,
    delivered BOOLEAN
);
```

## 7. Network Configuration

- **Transport:** QUIC (UDP) with TCP fallback
- **Encryption:** Noise protocol (libp2p-noise)
- **Discovery:** mDNS (local) + Kademlia (global)
- **NAT:** Auto-relay when direct connection fails
- **Ports:** 0 (OS-assigned) — no manual config needed

## 8. Security

- E2E encryption via Noise protocol
- Keys stored encrypted (future: with passphrase)
- No plaintext messages ever sent
- Peer verification via peer IDs

## 9. Acceptance Criteria

- [ ] User can generate identity and set username
- [ ] User can discover peers on local network via mDNS
- [ ] User can find peers globally via DHT
- [ ] User can add friends and see their online status
- [ ] User can send/receive encrypted messages
- [ ] User can transfer files to friends
- [ ] All data persists in local SQLite
- [ ] CLI is interactive and user-friendly
- [ ] No port forwarding required (works behind NAT)
- [ ] Graceful shutdown saves state
