# ZeroClaw Documentation

> Comprehensive guide for ZeroClaw — the fast, small, fully autonomous AI assistant infrastructure.
>
> **Version:** 0.1 (Feb 2026) · **License:** MIT · **Language:** Rust

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Installation](#2-installation)
3. [Getting Started](#3-getting-started)
4. [Configuration Reference](#4-configuration-reference)
5. [Providers](#5-providers)
6. [Channels](#6-channels)
7. [Tools](#7-tools)
8. [Memory System](#8-memory-system)
9. [Security](#9-security)
10. [Deployment Guides](#10-deployment-guides)
11. [Identity & Personality](#11-identity--personality)
12. [Advanced Features](#12-advanced-features)
13. [CLI Reference](#13-cli-reference)
14. [Troubleshooting](#14-troubleshooting)
15. [FAQ](#15-faq)

---

## 1. Introduction

### What is ZeroClaw?

ZeroClaw is a lightweight, fully autonomous AI assistant infrastructure written in 100% Rust. It provides a complete framework for building and deploying AI assistants that can interact through multiple channels (Telegram, Discord, Slack, WhatsApp, CLI, etc.), use various AI providers (OpenAI, Anthropic, Ollama, OpenRouter, etc.), and execute tools in a sandboxed environment.

### Key Characteristics

- **~3.4 MB** single binary — no runtime dependencies
- **< 5 MB** RAM footprint — runs on $10 hardware
- **< 10 ms** startup time — instant boot even on low-power devices
- **22+ AI providers** — swap with a config change
- **8+ channels** — CLI, Telegram, Discord, Slack, WhatsApp, iMessage, Matrix, Webhook
- **Traits-based architecture** — every subsystem is a pluggable trait

### Who is ZeroClaw for?

- Developers who want a self-hosted AI assistant without cloud lock-in
- Edge/IoT deployments where resources are constrained (Raspberry Pi, embedded boards)
- Teams that need strict security controls (pairing, sandboxing, allowlists)
- Anyone migrating from OpenClaw who wants a leaner, faster alternative

### ZeroClaw vs OpenClaw

| Aspect | OpenClaw | ZeroClaw |
|--------|----------|----------|
| Language | TypeScript (Node.js) | Rust |
| RAM | > 1 GB | < 5 MB |
| Startup | > 500s on 0.8GHz | < 10 ms |
| Binary | ~28 MB (+ Node.js runtime) | ~3.4 MB (self-contained) |
| Min Hardware Cost | ~$599 (Mac mini) | ~$10 (any ARM/x86 board) |

ZeroClaw provides a migration tool (`zeroclaw migrate openclaw`) to import your existing OpenClaw workspace and memory.

---

## 2. Installation

### Prerequisites

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install build-essential pkg-config
```

#### Linux (Fedora/RHEL)

```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config
```

#### macOS

```bash
xcode-select --install
```

#### Windows

```powershell
# Install Visual Studio Build Tools (select "Desktop development with C++" workload)
winget install Microsoft.VisualStudio.2022.BuildTools

# Install Rust
winget install Rustlang.Rustup
```

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

### Build from Source

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked

# Verify
zeroclaw --version
```

### Docker

```bash
# Using Docker Compose (recommended)
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cp .env.example .env
# Edit .env with your API key and settings
docker compose up -d
```

Or build the image manually:

```bash
docker build -t zeroclaw .
docker run -d \
  --name zeroclaw \
  -v ~/.zeroclaw:/root/.zeroclaw \
  -e ZEROCLAW_API_KEY=sk-... \
  zeroclaw daemon
```

### Low-Memory Boards (Raspberry Pi 3, 1 GB RAM)

If the kernel OOM-kills rustc during compilation:

```bash
# Limit to 1 parallel job
CARGO_BUILD_JOBS=1 cargo build --release

# Or cross-compile from a more powerful machine
# See: https://github.com/cross-rs/cross
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

---

## 3. Getting Started

### Quick Setup (Non-Interactive)

```bash
zeroclaw onboard --api-key sk-... --provider openrouter
```

This creates `~/.zeroclaw/config.toml` with your provider and API key.

### Interactive Setup

```bash
zeroclaw onboard --interactive
```

The wizard walks you through:
1. Choosing an AI provider
2. Entering your API key
3. Selecting a default model
4. Configuring channels (Telegram, Discord, etc.)
5. Setting up security (allowlists, autonomy level)

### Repair Channels Only

If you already have a config but need to fix channel settings:

```bash
zeroclaw onboard --channels-only
```

### Your First Chat

```bash
# Single message
zeroclaw agent -m "Hello, ZeroClaw!"

# Interactive REPL
zeroclaw agent
```

### Start the Gateway

The gateway is the webhook server that receives messages from channels:

```bash
# Default: binds to 127.0.0.1:8080
zeroclaw gateway

# Random port (security hardened)
zeroclaw gateway --port 0
```

### Start the Daemon

The daemon runs the full autonomous runtime (gateway + heartbeat + channels):

```bash
zeroclaw daemon
```

### Check Status

```bash
zeroclaw status
```

### Run Diagnostics

```bash
# Full system check
zeroclaw doctor

# Channel-specific health check
zeroclaw channel doctor
```

---

## 4. Configuration Reference

ZeroClaw uses a single TOML config file at `~/.zeroclaw/config.toml`. Created by `zeroclaw onboard`.

### Top-Level Settings

```toml
# Required: your AI provider API key
api_key = "sk-..."

# Default provider (see §5 for full list)
default_provider = "openrouter"

# Default model for the chosen provider
default_model = "anthropic/claude-sonnet-4-20250514"

# Sampling temperature (0.0 = deterministic, 1.0 = creative)
default_temperature = 0.7
```

### [memory]

```toml
[memory]
# Backend: "sqlite" (default), "lucid", "markdown", "none"
backend = "sqlite"

# Auto-save conversation context to memory
auto_save = true

# Embedding provider for vector search: "openai", "noop"
embedding_provider = "openai"

# Hybrid search weights (must sum to 1.0)
vector_weight = 0.7
keyword_weight = 0.3
```

Environment variables for Lucid backend:

| Variable | Default | Description |
|----------|---------|-------------|
| `ZEROCLAW_LUCID_CMD` | `lucid` | Path to lucid binary |
| `ZEROCLAW_LUCID_BUDGET` | `200` | Token budget for recall |
| `ZEROCLAW_LUCID_LOCAL_HIT_THRESHOLD` | `3` | Local hits before skipping external |
| `ZEROCLAW_LUCID_RECALL_TIMEOUT_MS` | `120` | Recall timeout (ms) |
| `ZEROCLAW_LUCID_STORE_TIMEOUT_MS` | `800` | Store sync timeout (ms) |
| `ZEROCLAW_LUCID_FAILURE_COOLDOWN_MS` | `15000` | Cooldown after failure (ms) |

### [gateway]

```toml
[gateway]
# Require pairing code on first connect
require_pairing = true

# Refuse 0.0.0.0 binding without a tunnel
allow_public_bind = false
```

### [autonomy]

```toml
[autonomy]
# "readonly" — can only read, no writes or commands
# "supervised" — asks before external actions (default)
# "full" — fully autonomous
level = "supervised"

# Restrict file operations to workspace directory
workspace_only = true

# Commands the agent is allowed to execute
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep"]

# Paths the agent cannot access
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]
```

### [runtime]

```toml
[runtime]
# "native" — run tools directly on host (default)
# "docker" — run tools in sandboxed container
kind = "native"

[runtime.docker]
image = "alpine:3.20"
network = "none"              # "none", "bridge", "host"
memory_limit_mb = 512
cpu_limit = 1.0
read_only_rootfs = true
```

### [tunnel]

```toml
[tunnel]
# "none" — no tunnel (default, localhost only)
# "cloudflare" — Cloudflare Tunnel
# "tailscale" — Tailscale Funnel
# "ngrok" — ngrok tunnel
# "custom" — custom tunnel binary
kind = "none"

# For cloudflare:
# [tunnel.cloudflare]
# token = "your-tunnel-token"

# For ngrok:
# [tunnel.ngrok]
# authtoken = "your-ngrok-token"
# domain = "your-domain.ngrok-free.app"

# For custom:
# [tunnel.custom]
# command = "/usr/local/bin/my-tunnel"
# args = ["--port", "{port}"]
```

### Channel Configuration

See [§6 Channels](#6-channels) for detailed setup of each channel.

```toml
[channels_config.telegram]
token = "123456:ABC-DEF..."
allowed_users = ["your_username"]    # without @

[channels_config.discord]
token = "your-bot-token"
allowed_users = ["your-discord-user-id"]

[channels_config.slack]
token = "xoxb-your-bot-token"
app_token = "xapp-your-app-token"
allowed_users = ["U01ABCDEF"]

[channels_config.whatsapp]
access_token = "EAABx..."
phone_number_id = "123456789012345"
verify_token = "my-secret-verify-token"
allowed_numbers = ["+1234567890"]

[channels_config.matrix]
homeserver = "https://matrix.org"
user = "@bot:matrix.org"
password = "your-password"
allowed_users = ["@you:matrix.org"]
```

### Allowlist Rules (All Channels)

- **Empty list `[]`** → deny all inbound messages
- **`["*"]`** → allow all (explicit opt-in, use for testing only)
- **`["user1", "user2"]`** → exact-match allowlist

---

## 5. Providers

ZeroClaw supports 22+ AI providers out of the box. Switch providers by changing `default_provider` and `api_key` in your config.

### Provider List

| Provider | Config Name | API Key Env Var |
|----------|-------------|-----------------|
| OpenRouter | `openrouter` | `ZEROCLAW_API_KEY` |
| OpenAI | `openai` | `ZEROCLAW_API_KEY` |
| Anthropic | `anthropic` | `ZEROCLAW_API_KEY` |
| Ollama (local) | `ollama` | — (no key needed) |
| Groq | `groq` | `ZEROCLAW_API_KEY` |
| DeepSeek | `deepseek` | `ZEROCLAW_API_KEY` |
| Mistral | `mistral` | `ZEROCLAW_API_KEY` |
| xAI (Grok) | `xai` | `ZEROCLAW_API_KEY` |
| Together | `together` | `ZEROCLAW_API_KEY` |
| Fireworks | `fireworks` | `ZEROCLAW_API_KEY` |
| Perplexity | `perplexity` | `ZEROCLAW_API_KEY` |
| Cohere | `cohere` | `ZEROCLAW_API_KEY` |
| Venice | `venice` | `ZEROCLAW_API_KEY` |
| AWS Bedrock | `bedrock` | AWS credentials |
| Custom (OpenAI-compatible) | `custom` | `ZEROCLAW_API_KEY` |

### OpenRouter (Recommended for Multi-Model)

```toml
api_key = "sk-or-v1-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-20250514"
```

### OpenAI

```toml
api_key = "sk-..."
default_provider = "openai"
default_model = "gpt-4o"
```

### Anthropic

```toml
api_key = "sk-ant-..."
default_provider = "anthropic"
default_model = "claude-sonnet-4-20250514"
```

### Groq

```toml
api_key = "gsk_..."
default_provider = "groq"
default_model = "llama-3.3-70b-versatile"
```

### DeepSeek

```toml
api_key = "sk-..."
default_provider = "deepseek"
default_model = "deepseek-chat"
```

### Ollama (Local Models)

Ollama runs models locally with no API key required.

#### Ollama on the Same Machine

```toml
default_provider = "ollama"
default_model = "llama3"

# Ollama defaults to http://127.0.0.1:11434
# Override if needed:
# [providers.ollama]
# base_url = "http://127.0.0.1:11434"
```

#### ⚠️ Ollama + ZeroClaw in Docker (Common Issue)

This is the #1 setup question (see [issue #426](https://github.com/zeroclaw-labs/zeroclaw/issues/426)). When ZeroClaw runs inside Docker and Ollama runs on the host, `127.0.0.1` inside the container points to the container itself, not the host.

**Solution 1: `host.docker.internal` (macOS / Windows)**

```toml
default_provider = "custom"

[providers.custom]
base_url = "http://host.docker.internal:11434/v1"
model = "llama3"
```

**Solution 2: `--network host` (Linux)**

```bash
docker run -d \
  --network host \
  --name zeroclaw \
  -v ~/.zeroclaw:/root/.zeroclaw \
  zeroclaw daemon
```

With `--network host`, the container shares the host's network stack, so `127.0.0.1:11434` works directly:

```toml
default_provider = "ollama"
default_model = "llama3"
```

**Solution 3: Host LAN IP (Any OS)**

Find your host's LAN IP (e.g., `192.168.1.100`):

```bash
# Linux
ip addr show | grep "inet " | grep -v 127.0.0.1

# macOS
ifconfig | grep "inet " | grep -v 127.0.0.1
```

```toml
default_provider = "custom"

[providers.custom]
base_url = "http://192.168.1.100:11434/v1"
model = "llama3"
```

**Solution 4: Docker Compose with Ollama**

```yaml
# docker-compose.yml
services:
  ollama:
    image: ollama/ollama
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama

  zeroclaw:
    build: .
    depends_on:
      - ollama
    volumes:
      - ~/.zeroclaw:/root/.zeroclaw
    environment:
      - ZEROCLAW_API_KEY=not-needed

volumes:
  ollama_data:
```

```toml
# config.toml — use Docker service name
default_provider = "custom"

[providers.custom]
base_url = "http://ollama:11434/v1"
model = "llama3"
```

> **Tip:** Make sure Ollama is listening on `0.0.0.0` (not just localhost) if you're connecting from another container. Set `OLLAMA_HOST=0.0.0.0` in the Ollama environment.

### Custom Provider (Any OpenAI-Compatible API)

```toml
default_provider = "custom"

[providers.custom]
base_url = "https://your-api.example.com/v1"
api_key = "your-key"
model = "your-model"
```

This works with any API that implements the OpenAI chat completions endpoint (`POST /v1/chat/completions`).

---

## 6. Channels

Channels are how users interact with ZeroClaw. Multiple channels can run simultaneously.

### CLI (Default)

No configuration needed. Just run:

```bash
zeroclaw agent
```

### Telegram

1. **Create a bot** via [@BotFather](https://t.me/BotFather) on Telegram:
   - Send `/newbot`
   - Choose a name and username
   - Copy the bot token

2. **Configure:**

```toml
[channels_config.telegram]
token = "123456789:AABBccDDeeFFggHHiiJJ"
allowed_users = ["your_username"]   # without @, or numeric user ID
```

3. **Start the gateway** with a tunnel (Telegram needs a public HTTPS URL):

```bash
zeroclaw gateway
```

4. **Find your user ID** if unsure:
   - Send a message to your bot
   - Check logs for: `ignoring message from unauthorized user: <id>`
   - Add that ID to `allowed_users`

### Discord

1. **Create a bot** at [Discord Developer Portal](https://discord.com/developers/applications):
   - New Application → Bot → Copy token
   - Enable **Message Content Intent** under Privileged Gateway Intents
   - Invite bot to your server with `bot` + `applications.commands` scopes

2. **Configure:**

```toml
[channels_config.discord]
token = "your-bot-token"
allowed_users = ["your-discord-user-id"]   # right-click user → Copy User ID
```

### Slack

1. **Create a Slack App** at [api.slack.com/apps](https://api.slack.com/apps):
   - Create New App → From Scratch
   - Add Bot Token Scopes: `chat:write`, `app_mentions:read`, `im:history`, `im:read`, `im:write`
   - Install to workspace
   - Enable Socket Mode and get an App-Level Token (`xapp-...`)

2. **Configure:**

```toml
[channels_config.slack]
token = "xoxb-your-bot-token"
app_token = "xapp-your-app-token"
allowed_users = ["U01ABCDEF"]   # Slack member ID
```

### WhatsApp

WhatsApp uses Meta's Business Cloud API (push-based webhooks):

1. Create a Meta Business App at [developers.facebook.com](https://developers.facebook.com)
2. Add the WhatsApp product
3. Get your Access Token, Phone Number ID, and define a Verify Token

```toml
[channels_config.whatsapp]
access_token = "EAABx..."
phone_number_id = "123456789012345"
verify_token = "my-secret-verify-token"
allowed_numbers = ["+1234567890"]   # E.164 format
```

4. Start gateway with a tunnel (WhatsApp requires HTTPS)
5. In Meta Developer Console → WhatsApp → Configuration → Webhook:
   - Callback URL: `https://your-tunnel-url/whatsapp`
   - Verify Token: same as config
   - Subscribe to `messages` field

### Matrix

```toml
[channels_config.matrix]
homeserver = "https://matrix.org"
user = "@yourbot:matrix.org"
password = "bot-password"
allowed_users = ["@you:matrix.org"]
```

### Webhook (Custom Integration)

The gateway exposes a webhook endpoint for custom integrations:

```bash
# Send a message via webhook
curl -X POST http://localhost:8080/webhook \
  -H "Authorization: Bearer <pairing-token>" \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from my app"}'
```

### Channel Doctor

Diagnose channel issues:

```bash
zeroclaw channel doctor
```

This checks token validity, webhook connectivity, and allowlist configuration for all configured channels.

---

## 7. Tools

Tools are capabilities the agent can invoke during conversations. ZeroClaw ships with a set of built-in tools and supports custom extensions.

### Built-in Tools

| Tool | Description |
|------|-------------|
| `shell` | Execute shell commands (scoped by autonomy settings) |
| `file_read` | Read file contents from the workspace |
| `file_write` | Write/create files in the workspace |
| `memory_store` | Save information to long-term memory |
| `memory_recall` | Search and retrieve from memory |
| `memory_forget` | Remove specific memories |
| `browser_open` | Open URLs in a browser (Brave search + allowlist) |
| `browser` | Full agent-browser / rust-native browser automation |
| `composio` | Optional integration with Composio for 3rd-party APIs |

### Tool Security

Tools respect the `[autonomy]` configuration:

- **`readonly`** — only `file_read`, `memory_recall`, and `browser_open` are available
- **`supervised`** — all tools available, but destructive actions require confirmation
- **`full`** — all tools available without confirmation

Shell commands are filtered through `allowed_commands` and `forbidden_paths`:

```toml
[autonomy]
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep", "python3"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]
```

### Adding Custom Tools

Custom tools can be added via the Skills system (see [§12 Advanced Features](#12-advanced-features)) or by implementing the `Tool` trait in Rust.

---

## 8. Memory System

ZeroClaw includes a full-stack search engine for agent memory — no external dependencies required.

### Architecture

| Layer | Implementation |
|-------|---------------|
| Vector DB | Embeddings stored as BLOB in SQLite, cosine similarity |
| Keyword Search | FTS5 virtual tables with BM25 scoring |
| Hybrid Merge | Custom weighted merge (`vector.rs`) |
| Embeddings | `EmbeddingProvider` trait — OpenAI, custom URL, or noop |
| Chunking | Line-based markdown chunker with heading preservation |
| Caching | SQLite `embedding_cache` table with LRU eviction |
| Reindex | Rebuild FTS5 + re-embed missing vectors atomically |

### Backends

#### SQLite (Default, Recommended)

```toml
[memory]
backend = "sqlite"
auto_save = true
embedding_provider = "openai"
vector_weight = 0.7
keyword_weight = 0.3
```

Full hybrid search with both keyword (BM25) and vector (cosine similarity) ranking. Best accuracy.

#### Lucid Bridge

```toml
[memory]
backend = "lucid"
```

Syncs with the Lucid CLI tool for external memory management. Falls back to local SQLite when Lucid is unavailable. Configure via environment variables (see [§4 Configuration Reference](#4-configuration-reference)).

#### Markdown

```toml
[memory]
backend = "markdown"
```

Simple file-based memory using markdown files. No vector search — keyword matching only. Good for minimal setups.

#### None (Disabled)

```toml
[memory]
backend = "none"
```

Explicit no-op backend. No persistence at all. Useful for stateless/ephemeral deployments.

### Embedding Providers

For vector search to work, you need an embedding provider:

```toml
[memory]
embedding_provider = "openai"   # Uses OpenAI's embedding API (requires api_key)
# embedding_provider = "noop"   # Disables vector search, keyword-only
```

You can also use a custom embedding endpoint:

```toml
[memory]
embedding_provider = "custom"

[memory.embedding]
base_url = "http://localhost:8080/v1"
model = "your-embedding-model"
```

### Memory Migration from OpenClaw

```bash
# Preview what will be migrated (safe, no changes)
zeroclaw migrate openclaw --dry-run

# Run the migration
zeroclaw migrate openclaw
```

This imports OpenClaw's MEMORY.md, daily notes, and workspace files into ZeroClaw's memory backend.

---

## 9. Security

ZeroClaw enforces security at every layer — not just the sandbox.

### Security Checklist

| # | Item | How |
|---|------|-----|
| 1 | Gateway not publicly exposed | Binds `127.0.0.1` by default. Refuses `0.0.0.0` without tunnel or explicit `allow_public_bind = true` |
| 2 | Pairing required | 6-digit one-time code on startup. Exchange via `POST /pair` for bearer token |
| 3 | Filesystem scoped | `workspace_only = true` by default. 14 system dirs + 4 sensitive dotfiles blocked. Symlink escape detection |
| 4 | Access via tunnel only | Gateway refuses public bind without active tunnel |

### Pairing Flow

1. Start the gateway: `zeroclaw gateway`
2. A 6-digit pairing code is printed to the console
3. Send `POST /pair` with the code to get a bearer token
4. All subsequent `/webhook` requests require `Authorization: Bearer <token>`

### Filesystem Scoping

When `workspace_only = true` (default):

- File tools can only access the workspace directory
- 14 system directories are blocked (`/etc`, `/proc`, `/sys`, etc.)
- 4 sensitive dotfiles are blocked (`~/.ssh`, `~/.gnupg`, `~/.aws`, etc.)
- Null byte injection is blocked
- Symlink escape is detected via path canonicalization

### Docker Sandboxing

For maximum isolation, run tools inside a Docker container:

```toml
[runtime]
kind = "docker"

[runtime.docker]
image = "alpine:3.20"
network = "none"           # No network access for tools
memory_limit_mb = 512
cpu_limit = 1.0
read_only_rootfs = true    # Immutable root filesystem
```

### Rate Limiting

ZeroClaw includes built-in rate limiting (20 actions/hour by default) to prevent runaway agents.

---

## 10. Deployment Guides

### Local Development

```bash
# Build and run
cargo build --release
./target/release/zeroclaw onboard --interactive
./target/release/zeroclaw daemon
```

### Raspberry Pi / Edge Devices

```bash
# On the Pi (if it has enough RAM to compile)
CARGO_BUILD_JOBS=1 cargo build --release

# Or cross-compile from a powerful machine
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
scp target/aarch64-unknown-linux-gnu/release/zeroclaw pi@raspberrypi:~/

# On the Pi
ssh pi@raspberrypi
./zeroclaw onboard --interactive
./zeroclaw daemon
```

See also: [docs/network-deployment.md](network-deployment.md) for detailed Raspberry Pi + Telegram setup.

### Docker Compose

```yaml
# docker-compose.yml
version: "3.8"
services:
  zeroclaw:
    build: .
    restart: unless-stopped
    volumes:
      - zeroclaw_config:/root/.zeroclaw
      - zeroclaw_workspace:/workspace
    ports:
      - "8080:8080"
    environment:
      - ZEROCLAW_API_KEY=sk-...

volumes:
  zeroclaw_config:
  zeroclaw_workspace:
```

```bash
docker compose up -d
docker compose logs -f zeroclaw
```

### systemd Service

```bash
# Install as a system service
zeroclaw service install

# Manage
zeroclaw service status
systemctl start zeroclaw
systemctl stop zeroclaw
systemctl restart zeroclaw

# View logs
journalctl -u zeroclaw -f
```

### Cloud VPS

1. Provision a small VPS (512 MB RAM is plenty)
2. Install Rust and build ZeroClaw
3. Configure a tunnel for webhook access:

```toml
[tunnel]
kind = "cloudflare"

[tunnel.cloudflare]
token = "your-cloudflare-tunnel-token"
```

4. Install as a service: `zeroclaw service install`
5. Configure your channel webhooks to point to the tunnel URL

---

## 11. Identity & Personality

ZeroClaw supports customizable agent identity through workspace files.

### Workspace Files

| File | Purpose |
|------|---------|
| `SOUL.md` | Agent personality, tone, and behavioral guidelines |
| `USER.md` | Information about the human the agent assists |
| `AGENTS.md` | Workspace rules, memory protocol, safety guidelines |
| `TOOLS.md` | Local environment notes (camera names, SSH hosts, etc.) |
| `HEARTBEAT.md` | Periodic tasks for the heartbeat system |
| `MEMORY.md` | Long-term curated memory |
| `memory/YYYY-MM-DD.md` | Daily session logs |

### Identity Formats

#### OpenClaw Markdown (Default)

Place markdown files in your workspace directory. The agent reads them at session start.

```markdown
# SOUL.md
You are a helpful coding assistant. Be concise and direct.
Prefer Rust and TypeScript. Avoid unnecessary abstractions.
```

#### AIEOS v1.1 JSON

```json
{
  "version": "1.1",
  "name": "MyAssistant",
  "role": "developer-assistant",
  "personality": {
    "tone": "professional",
    "verbosity": "concise"
  }
}
```

Configure the format in config:

```toml
[identity]
format = "openclaw"   # or "aieos"
```

---

## 12. Advanced Features

### Heartbeat System

The heartbeat periodically wakes the agent to perform background tasks defined in `HEARTBEAT.md`:

```markdown
# HEARTBEAT.md
- Check for unread emails
- Review calendar for upcoming events
- Check weather if going out today
```

The agent rotates through these tasks during heartbeat polls, tracking state in `memory/heartbeat-state.json`.

### Skills

Skills are packaged capabilities with TOML manifests and SKILL.md instructions:

```
skills/
  my-skill/
    skill.toml      # Manifest: name, description, dependencies
    SKILL.md         # Instructions for the agent
    scripts/         # Helper scripts
```

```toml
# skill.toml
name = "my-skill"
description = "Does something useful"
version = "0.1.0"

[dependencies]
commands = ["curl", "jq"]
```

Install community skills or create your own. See [docs/adding-boards-and-tools.md](adding-boards-and-tools.md).

### Cron / Scheduled Tasks

Schedule recurring or one-shot tasks:

```toml
[[cron]]
name = "daily-summary"
schedule = "0 9 * * *"          # 9 AM daily
command = "Summarize yesterday's activity"

[[cron]]
name = "weekly-review"
schedule = "0 10 * * 1"         # Monday 10 AM
command = "Review this week's goals and progress"
```

### Cost Tracking

ZeroClaw tracks API usage and costs:

```bash
zeroclaw status   # Shows current session cost
```

Configure budget limits:

```toml
[cost]
daily_limit_usd = 5.00
monthly_limit_usd = 50.00
warn_at_percent = 80
```

### RAG (Retrieval-Augmented Generation)

ZeroClaw's memory system doubles as a RAG pipeline. When the agent receives a query:

1. Hybrid search (FTS5 + vector) retrieves relevant memory chunks
2. Results are ranked by weighted merge of BM25 and cosine similarity scores
3. Top chunks are injected into the prompt as context

No external vector databases or frameworks needed.

### Integrations Registry

ZeroClaw ships with 50+ integrations across 9 categories. View available integrations:

```bash
zeroclaw integrations list
zeroclaw integrations info Telegram
```

---

## 13. CLI Reference

### Core Commands

| Command | Description |
|---------|-------------|
| `zeroclaw onboard` | Initial setup wizard |
| `zeroclaw onboard --interactive` | Interactive setup |
| `zeroclaw onboard --api-key KEY --provider PROV` | Non-interactive setup |
| `zeroclaw onboard --channels-only` | Repair channel config only |
| `zeroclaw agent` | Interactive chat REPL |
| `zeroclaw agent -m "message"` | Single message mode |
| `zeroclaw gateway` | Start webhook server (default: 127.0.0.1:8080) |
| `zeroclaw gateway --port PORT` | Start on specific port (0 = random) |
| `zeroclaw daemon` | Full autonomous runtime |
| `zeroclaw status` | Show system status |
| `zeroclaw doctor` | Run system diagnostics |
| `zeroclaw channel doctor` | Check channel health |

### Service Management

| Command | Description |
|---------|-------------|
| `zeroclaw service install` | Install as systemd service |
| `zeroclaw service status` | Check service status |

### Migration

| Command | Description |
|---------|-------------|
| `zeroclaw migrate openclaw --dry-run` | Preview migration (no changes) |
| `zeroclaw migrate openclaw` | Migrate from OpenClaw |

### Integrations

| Command | Description |
|---------|-------------|
| `zeroclaw integrations list` | List available integrations |
| `zeroclaw integrations info NAME` | Show integration details |

---

## 14. Troubleshooting

### Build Errors

#### OpenSSL Errors (Linux)

```bash
# Debian/Ubuntu
sudo apt install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel pkg-config
```

#### Out of Memory During Compilation

Common on Raspberry Pi 3 (1 GB RAM):

```bash
# Limit parallel compilation jobs
CARGO_BUILD_JOBS=1 cargo build --release

# Or add swap
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Docker Networking

#### Can't Connect to Ollama from Docker

**Symptom:** `Connection refused` when ZeroClaw in Docker tries to reach Ollama on the host.

**Cause:** `127.0.0.1` inside the container refers to the container, not the host.

**Fix:** See [§5 Providers → Ollama + ZeroClaw in Docker](#️-ollama--zeroclaw-in-docker-common-issue) for four solutions.

#### Ollama Not Accepting External Connections

**Symptom:** Connection works from host but not from Docker container.

**Cause:** Ollama defaults to listening on `127.0.0.1` only.

**Fix:** Set `OLLAMA_HOST=0.0.0.0` before starting Ollama:

```bash
OLLAMA_HOST=0.0.0.0 ollama serve
```

### Channel Issues

#### "Ignoring message from unauthorized user"

Your sender identity isn't in the allowlist.

1. Check the log for the exact sender identity
2. Add it to the appropriate `allowed_users` / `allowed_numbers` list
3. Restart: `zeroclaw onboard --channels-only`

#### Telegram Webhook Not Receiving Messages

1. Ensure gateway is running with a public tunnel
2. Check tunnel is active: `zeroclaw status`
3. Verify webhook URL is set correctly in Telegram (BotFather → `/setwebhook`)
4. Run `zeroclaw channel doctor`

### Memory / Embedding Issues

#### "Embedding provider not configured"

You need an embedding provider for vector search:

```toml
[memory]
embedding_provider = "openai"   # Requires api_key
```

Or disable vector search:

```toml
[memory]
embedding_provider = "noop"
vector_weight = 0.0
keyword_weight = 1.0
```

#### Memory Search Returns No Results

1. Check backend is not `"none"`
2. Verify memories were saved: `zeroclaw agent -m "What do you remember?"`
3. Try reindexing: the agent's `memory_recall` tool triggers search automatically

---

## 15. FAQ

**Q: Do I need an API key to use ZeroClaw?**
A: Only if you're using a cloud AI provider. With Ollama (local models), no API key is needed.

**Q: Can I use multiple providers simultaneously?**
A: You set one `default_provider`, but you can switch models per-conversation. OpenRouter gives you access to 100+ models through a single API key.

**Q: How do I update ZeroClaw?**
A: Pull the latest source and rebuild:
```bash
cd zeroclaw
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

**Q: Can I run ZeroClaw on a Raspberry Pi Zero?**
A: The binary runs fine, but compilation needs cross-compiling from a more powerful machine. The Pi Zero's 512 MB RAM isn't enough to compile Rust.

**Q: How do I back up my data?**
A: Back up `~/.zeroclaw/` (config + SQLite memory database) and your workspace directory.

**Q: Is ZeroClaw compatible with OpenClaw skills?**
A: ZeroClaw uses TOML skill manifests instead of OpenClaw's format, but the migration tool (`zeroclaw migrate openclaw`) handles conversion.

**Q: Can I run multiple ZeroClaw instances?**
A: Yes, use different config directories: `ZEROCLAW_CONFIG_DIR=~/.zeroclaw-2 zeroclaw daemon`

**Q: How do I reset everything?**
A: Remove the config directory and re-onboard:
```bash
rm -rf ~/.zeroclaw
zeroclaw onboard --interactive
```

**Q: What's the difference between `gateway` and `daemon`?**
A: `gateway` runs only the webhook server. `daemon` runs the full autonomous runtime: gateway + heartbeat + channel polling + scheduled tasks.

**Q: How secure is ZeroClaw?**
A: ZeroClaw binds to localhost only, requires pairing tokens, scopes filesystem access, supports Docker sandboxing, and enforces channel allowlists. Run `nmap -p 1-65535 <your-host>` to verify nothing is exposed. See [§9 Security](#9-security) for details.

---

*This documentation is a living document. Contributions welcome at [github.com/zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw).*
