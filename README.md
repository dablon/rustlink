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

# Iniciar nodo P2P
$ rustlink run
🚀 Iniciando nodo P2P...
```

## ✨ Características

- 🔐 **Identidad Descentralizada** — Keypair Ed25519, sin servidor de registro
- 🌐 **P2P Real** — libp2p con Kademlia DHT + mDNS
- 🔒 **E2E Encryption** — Noise Protocol integrado
- 📡 **NAT Traversal** — Hole punching + relay fallback
- 💬 **Chat** — Mensajería en tiempo real
- 📁 **Transferencia de Archivos** — Con verificación SHA256
- 🗄️ **Storage Local** — SQLite para mensajes y amigos

## 🏗️ Arquitectura

```
┌─────────────────────────────────────────┐
│           CLI Layer (clap)              │
│  init, login, add, chat, send, run    │
├─────────────────────────────────────────┤
│           Core Logic Layer              │
│  Identity, Messaging, Storage           │
├─────────────────────────────────────────┤
│           Network Layer (libp2p)        │
│  Kademlia DHT, mDNS, Noise, QUIC      │
├─────────────────────────────────────────┤
│           Storage (SQLite)              │
└─────────────────────────────────────────┘
```

## 📦 Stack

- **Rust** con tokio async
- **libp2p 0.56** — P2P networking
- **clap** — CLI
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
| `rustlink send <file> <peer_id>` | Enviar archivo |
| `rustlink run` | Iniciar nodo P2P |

## 📁 Estructura del Proyecto

```
rustlink/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI commands
│   ├── identity.rs      # Identity management
│   ├── storage.rs       # SQLite storage
│   └── network.rs       # P2P networking
├── Cargo.toml
├── README.md
└── SPEC.md
```

## 🔑 Conceptos Clave

- **PeerID**: Identificador único basado en tu clave pública (ej: `12D3KooW...`)
- **DHT**: Kademlia para descubrimiento global de peers
- **mDNS**: Descubrimiento automático en red local
- **Noise Protocol**: Cifrado E2E automático

## ⚠️ Estado

**En desarrollo** — La red P2P básica funciona, faltan:
- Protocolos de chat/file-transfer completos
- TUI con ratatui
- Bootstrap nodes públicos

## 📝 Licencia

MIT
