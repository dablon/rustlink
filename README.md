# RustLink — P2P CLI Social App 🦀

Decentralized peer-to-peer social networking application for the command line. Built with Rust and libp2p.

## Features

- 🔐 **Identity Management** — Cryptographic keypair generation (Ed25519)
- 🌐 **P2P Networking** — No servers required, works behind NAT
- 🔍 **Peer Discovery** — mDNS (local) + Kademlia DHT (global)
- 💬 **Messaging** — Encrypted direct messages
- 📁 **File Transfer** — P2P file sharing
- 🗄️ **Local Storage** — SQLite for messages and friends

## Quick Start

```bash
# Register a new identity
rustlink register myusername

# Check status
rustlink status

# Add a friend
rustlink add-friend friendname

# Open chat
rustlink chat friendname

# Send a file
rustlink send-file /path/to/file.txt friendname

# Start P2P node (daemon)
rustlink run
```

## Architecture

```
┌─────────────────────────────────────┐
│     CLI Layer (clap)                │
│  register, login, friends, chat     │
├─────────────────────────────────────┤
│     Core Logic Layer                │
│  Identity, Messaging, Storage       │
├─────────────────────────────────────┤
│     Network Layer (libp2p)          │
│  Kademlia DHT, mDNS, Noise         │
├─────────────────────────────────────┤
│     Storage (SQLite)                │
└─────────────────────────────────────┘
```

## Tech Stack

- **Language:** Rust
- **Networking:** libp2p 0.56
- **CLI:** clap
- **Storage:** rusqlite

## Building

```bash
cargo build --release
```

## Requirements

- Rust 1.70+
- Linux/macOS (Windows support coming soon)

## License

MIT
