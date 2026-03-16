# RustLink — P2P CLI Social App 🦀

Decentralized peer-to-peer social networking application for the command line. Built with Rust and libp2p.

## 🚀 Ejemplo de Uso

```bash
# Primera vez - inicializar identidad
$ rustlink init miusuario
✓ Identidad creada!
 Tu PeerID: 12D3KooWEyp4x7Zq...
 Compártelo con tus amigos para conectarse

# Ver estado
$ rustlink status
┌─────────────────────────────────┐
│ Estado de RustLink             │
├─────────────────────────────────┤
│ Usuario: miusuario              │
│ PeerID: 12D3KooWEyp4x7Zq...   │
│ Estado: 🟢 En línea            │
└─────────────────────────────────┘

# Agregar amigo
$ rustlink add 12D3KooWAbc123...
🔍 Buscando peer 12D3KooWAbc123...
✓ Solicitud enviada

# Chat
$ rustlink chat 12D3KooWAbc123
💬 Abriendo chat con 12D3KooWAbc123...
(Mensajes recientes se muestran aquí)

# Enviar archivo
$ rustlink send archivo.zip 12D3KooWAbc123
📦 Enociando archivo.zip (2.3 MB)
████████████████████░░░░ 84%

# Iniciar nodo P2P (con nodos bootstrap)
$ rustlink run --bootstrap /ip4/147.75.80.110/tcp/4001/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gvUBJyJt4Wz
🚀 Iniciando nodo P2P...
```

## ✨ Características

- 🔐 **Identidad Descentralizada** — Keypair Ed25519, sin servidor de registro
- 🌐 **P2P Real** — libp2p con Kademlia DHT + mDNS
- 🔒 **E2E Encryption** — Noise Protocol integrado
- 📡 **NAT Traversal** — Relay fallback
- 💬 **Chat** — Protocolo definido (`/rustlink/chat/1.0.0`)
- 📁 **Transferencia de Archivos** — Chunks de 64KB con verificación SHA256
- 🗄️ **Storage Local** — SQLite para mensajes y amigos
- 🎨 **TUI** — Interfaz interactiva con ratatui

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────┐
│           CLI Layer (clap + ratatui)    │
│  init, login, add, chat, send, run, tui│
├─────────────────────────────────────────┤
│           Core Logic Layer              │
│  Identity, Messaging, FileTransfer      │
├─────────────────────────────────────────┤
│           Network Layer (libp2p)        │
│  Kademlia DHT, mDNS, Noise, TCP        │
├─────────────────────────────────────────┤
│           Storage (SQLite)              │
│  Friends, Messages, Identity            │
└─────────────────────────────────────────┘
```

## 📦 Stack

- **Rust** con tokio async
- **libp2p 0.56** — P2P networking
- **clap** — CLI
- **ratatui** — TUI
- **rusqlite** — SQLite local

## 🚀 Instalación

### Quick Install (desde Release)

```bash
# Linux/macOS
curl -L https://github.com/rustlink/rustlink/releases/latest/download/rustlink -o rustlink && chmod +x rustlink && sudo mv rustlink /usr/local/bin/

# Windows (PowerShell)
iwr -Uri "https://github.com/rustlink/rustlink/releases/latest/download/rustlink.exe" -OutFile rustlink.exe
```

### Build desde código

```bash
# Clonar y compilar
git clone https://github.com/rustlink/rustlink.git
cd rustlink

# Opción 1: Script de instalación (recomendado)
# Linux/macOS/WSL
./install.sh

# Windows (PowerShell)
.\install.ps1

# Opción 2: Manual
cargo build --release
# El binario estará en target/release/rustlink
```

## 🔧 Build

```bash
cargo build --release
```

## 📋 Comandos

| Comando | Descripción |
|---------|-------------|
| `rustlink init <username>` | Crear nueva identidad |
| `rustlink login` | Cargar identidad existente |
| `rustlink status` | Ver estado actual |
| `rustlink friends` | Listar amigos |
| `rustlink add <peer_id>` | Agregar amigo |
| `rustlink chat <peer_id>` | Abrir chat |
| `rustlink tui` | Abrir interfaz TUI interactiva |
| `rustlink send <file> <peer_id>` | Enviar archivo |
| `rustlink run [--bootstrap <addr>]` | Iniciar nodo P2P |
| `rustlink version` | Ver versión |

## 📁 Estructura del Proyecto

```
rustlink/
├── src/
│   ├── main.rs            # Entry point
│   ├── cli.rs             # CLI commands
│   ├── identity.rs        # Identity management
│   ├── storage.rs         # SQLite storage
│   ├── network.rs         # P2P networking (Kademlia)
│   ├── messaging.rs       # Chat protocol
│   ├── filetransfer.rs   # File transfer protocol
│   └── tui.rs            # TUI with ratatui
├── Cargo.toml
├── README.md
└── SPEC.md
```

## 🔑 Conceptos Clave

- **PeerID**: Identificador único basado en tu clave pública (ej: `12D3KooW...`)
- **DHT**: Kademlia para descubrimiento global de peers
- **mDNS**: Descubrimiento automático en red local
- **Noise Protocol**: Cifrado E2E automático
- **Bootstrap Nodes**: Nodos iniciales para unirse a la red (configurables con `--bootstrap`)

## ⚠️ Estado

**En desarrollo** — La red P2P básica funciona:

- ✅ Identity management (Ed25519 keypair)
- ✅ Kademlia DHT discovery
- ✅ mDNS local discovery
- ✅ SQLite storage
- ✅ Chat protocol (definido)
- ✅ File transfer protocol (definido, chunks 64KB + SHA256)
- ✅ TUI skeleton (ratatui)
- 🔧 Bootstrap nodes (configurables via CLI)

## 📝 Licencia

MIT
