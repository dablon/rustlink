# RustLink — P2P CLI Social App 🦀

Decentralized peer-to-peer social networking application for the command line. Built with Rust and libp2p.

## ⚡ Quick Start

```bash
# Clone and build
git clone https://github.com/dablon/rustlink.git
cd rustlink
cargo generate-lockfile
cargo build --release

# Run
./target/release/rustlink --help
```

## 🚀 Usage Examples (Verified)

### 1. Initialize Identity
```bash
$ rustlink init testuser

✓ Identidad creada!
 Tu PeerID: 12D3KooWFUfm6wSYUX9JnxEWN681Vto7dwmaC91Ty1y3d8EeBHQp
 Compártelo con tus amigos para conectarse
```

### 2. Login (Load Identity)
```bash
$ rustlink login

✓ Sesión iniciada
 PeerID: 12D3KooWFUfm6wSYUX9JnxEWN681Vto7dwmaC91Ty1y3d8EeBHQp
```

### 3. Check Status
```bash
$ rustlink status

┌─────────────────────────────────┐
│ Estado de RustLink             │
├─────────────────────────────────┤
│ Usuario: testuser                     │
│ PeerID: 12D3KooWFUfm6wSY... │
│ Estado: 🟢 En línea            │
└─────────────────────────────────┘
```

### 4. List Friends
```bash
$ rustlink friends

No tienes amigos aún.
Usa 'rustlink add <peer_id>' para agregar uno.
```

### 5. Add Friend
```bash
$ rustlink add 12D3KooWTest123456789

🔍 Buscando peer 12D3KooWTest1234...
✓ Solicitud enviada (DHT en desarrollo)
```

### 6. Open Chat
```bash
$ rustlink chat 12D3KooWTest123456789

💬 Abriendo chat con 12D3KooWTest1234...

(Chat TUI con ratatui en desarrollo)
```

### 7. Send File
```bash
$ rustlink send /path/to/file.zip 12D3KooWTest123456789

📦 Enviando /path/to/file.zip (13 bytes)
████████████████████░░░░ 80%
✓ Archivo enviado a 12D3KooWTest1234 (implementación en desarrollo)
```

### 8. Start P2P Node
```bash
$ rustlink run

🚀 Iniciando nodo P2P...
 PeerID: 12D3KooWFUfm6wSYUX9JnxEWN681Vto7dwmaC91Ty1y3d8EeBHQp
✓ Nodo iniciado
 Presiona Ctrl+C para salir
```

### 9. Version
```bash
$ rustlink version

RustLink v0.1.0
P2P CLI Social App - Sin servidores, sin registro
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
# Generate lockfile (required first time)
cargo generate-lockfile

# Build release binary
cargo build --release

# Binary location: target/release/rustlink
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
| `rustlink version` | Ver versión |

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

## 🐳 Docker

```bash
# Build image
docker build -t rustlink .

# Run with docker-compose
docker-compose -f docker-compose.test.yml up
```

## 📝 Licencia

MIT
