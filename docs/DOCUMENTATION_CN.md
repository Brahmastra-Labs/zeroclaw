# ZeroClaw 文档

> ZeroClaw 完整指南 —— 快速、轻量、完全自主的 AI 助手基础设施。
>
> **版本：** 0.1 (2026 年 2 月) · **许可证：** MIT · **语言：** Rust

---

## 目录

1. [简介](#1-简介)
2. [安装](#2-安装)
3. [快速上手](#3-快速上手)
4. [配置参考](#4-配置参考)
5. [服务提供商](#5-服务提供商)
6. [频道](#6-频道)
7. [工具](#7-工具)
8. [记忆系统](#8-记忆系统)
9. [安全](#9-安全)
10. [部署指南](#10-部署指南)
11. [身份与个性](#11-身份与个性)
12. [高级功能](#12-高级功能)
13. [CLI 参考](#13-cli-参考)
14. [故障排除](#14-故障排除)
15. [常见问题](#15-常见问题)

---

## 1. 简介

### ZeroClaw 是什么？

ZeroClaw 是一个轻量级、完全自主的 AI 助手基础设施，100% 用 Rust 编写。它提供了一套完整的框架，用于构建和部署 AI 助手，支持多种频道（Telegram、Discord、Slack、WhatsApp、CLI 等）、多种 AI 服务提供商（OpenAI、Anthropic、Ollama、OpenRouter 等），并在沙箱环境中执行工具。

### 核心特性

- **~3.4 MB** 单一二进制文件 —— 无运行时依赖
- **< 5 MB** 内存占用 —— 在 10 美元的硬件上就能跑
- **< 10 ms** 启动时间 —— 低功耗设备也能秒启
- **22+ AI 服务提供商** —— 改个配置就能切换
- **8+ 频道** —— CLI、Telegram、Discord、Slack、WhatsApp、iMessage、Matrix、Webhook
- **基于 Trait 的架构** —— 每个子系统都是可插拔的 Trait

### ZeroClaw 适合谁？

- 想要自托管 AI 助手、不想被云服务锁定的开发者
- 资源受限的边缘/IoT 部署场景（Raspberry Pi、嵌入式开发板）
- 需要严格安全控制（配对、沙箱、白名单）的团队
- 从 OpenClaw 迁移过来、想要更精简更快方案的用户

### ZeroClaw vs OpenClaw

| 方面 | OpenClaw | ZeroClaw |
|------|----------|----------|
| 语言 | TypeScript (Node.js) | Rust |
| 内存 | > 1 GB | < 5 MB |
| 启动时间 | > 500s（0.8GHz 下） | < 10 ms |
| 二进制大小 | ~28 MB（+ Node.js 运行时） | ~3.4 MB（自包含） |
| 最低硬件成本 | ~$599（Mac mini） | ~$10（任意 ARM/x86 开发板） |

ZeroClaw 提供了迁移工具（`zeroclaw migrate openclaw`），可以导入你现有的 OpenClaw 工作区和记忆数据。

---

## 2. 安装

### 前置条件

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

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

### 从源码构建

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

或者手动构建镜像：

```bash
docker build -t zeroclaw .
docker run -d \
  --name zeroclaw \
  -v ~/.zeroclaw:/root/.zeroclaw \
  -e ZEROCLAW_API_KEY=sk-... \
  zeroclaw daemon
```

### 低内存开发板（Raspberry Pi 3，1 GB RAM）

如果编译时内核 OOM-kill 了 rustc：

```bash
# Limit to 1 parallel job
CARGO_BUILD_JOBS=1 cargo build --release

# Or cross-compile from a more powerful machine
# See: https://github.com/cross-rs/cross
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

---

## 3. 快速上手

### 快速设置（非交互式）

```bash
zeroclaw onboard --api-key sk-... --provider openrouter
```

这会在 `~/.zeroclaw/config.toml` 中创建配置文件，包含你的服务提供商和 API key。

### 交互式设置

```bash
zeroclaw onboard --interactive
```

向导会引导你完成：
1. 选择 AI 服务提供商
2. 输入 API key
3. 选择默认模型
4. 配置频道（Telegram、Discord 等）
5. 设置安全选项（白名单、自主级别）

### 仅修复频道配置

如果你已经有配置文件，只需要修复频道设置：

```bash
zeroclaw onboard --channels-only
```

### 第一次对话

```bash
# Single message
zeroclaw agent -m "Hello, ZeroClaw!"

# Interactive REPL
zeroclaw agent
```

### 启动 Gateway

Gateway 是接收频道消息的 Webhook 服务器：

```bash
# Default: binds to 127.0.0.1:8080
zeroclaw gateway

# Random port (security hardened)
zeroclaw gateway --port 0
```

### 启动 Daemon

Daemon 运行完整的自主运行时（Gateway + 心跳 + 频道）：

```bash
zeroclaw daemon
```

### 检查状态

```bash
zeroclaw status
```

### 运行诊断

```bash
# Full system check
zeroclaw doctor

# Channel-specific health check
zeroclaw channel doctor
```

---

## 4. 配置参考

ZeroClaw 使用单个 TOML 配置文件，位于 `~/.zeroclaw/config.toml`。由 `zeroclaw onboard` 创建。

### 顶层设置

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

Lucid 后端的环境变量：

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `ZEROCLAW_LUCID_CMD` | `lucid` | Lucid 二进制文件路径 |
| `ZEROCLAW_LUCID_BUDGET` | `200` | 召回的 Token 预算 |
| `ZEROCLAW_LUCID_LOCAL_HIT_THRESHOLD` | `3` | 本地命中次数达到此值后跳过外部查询 |
| `ZEROCLAW_LUCID_RECALL_TIMEOUT_MS` | `120` | 召回超时（毫秒） |
| `ZEROCLAW_LUCID_STORE_TIMEOUT_MS` | `800` | 存储同步超时（毫秒） |
| `ZEROCLAW_LUCID_FAILURE_COOLDOWN_MS` | `15000` | 失败后冷却时间（毫秒） |

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

### 频道配置

详见 [§6 频道](#6-频道) 了解每个频道的详细设置。

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

### 白名单规则（所有频道通用）

- **空列表 `[]`** → 拒绝所有入站消息
- **`["*"]`** → 允许所有（需显式启用，仅建议测试时使用）
- **`["user1", "user2"]`** → 精确匹配白名单

---

## 5. 服务提供商

ZeroClaw 开箱即用支持 22+ AI 服务提供商。切换提供商只需修改配置中的 `default_provider` 和 `api_key`。

### 提供商列表

| 提供商 | 配置名称 | API Key 环境变量 |
|--------|----------|-----------------|
| OpenRouter | `openrouter` | `ZEROCLAW_API_KEY` |
| OpenAI | `openai` | `ZEROCLAW_API_KEY` |
| Anthropic | `anthropic` | `ZEROCLAW_API_KEY` |
| Ollama（本地） | `ollama` | —（不需要 key） |
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
| Custom（OpenAI 兼容） | `custom` | `ZEROCLAW_API_KEY` |

### OpenRouter（推荐用于多模型场景）

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

### Ollama（本地模型）

Ollama 在本地运行模型，不需要 API key。

#### Ollama 在同一台机器上

```toml
default_provider = "ollama"
default_model = "llama3"

# Ollama defaults to http://127.0.0.1:11434
# Override if needed:
# [providers.ollama]
# base_url = "http://127.0.0.1:11434"
```

#### ⚠️ Ollama + ZeroClaw 在 Docker 中（常见问题）

这是排名第一的配置问题（见 [issue #426](https://github.com/zeroclaw-labs/zeroclaw/issues/426)）。当 ZeroClaw 运行在 Docker 容器内而 Ollama 运行在宿主机上时，容器内的 `127.0.0.1` 指向的是容器自身，而不是宿主机。

**方案 1：`host.docker.internal`（macOS / Windows）**

```toml
default_provider = "custom"

[providers.custom]
base_url = "http://host.docker.internal:11434/v1"
model = "llama3"
```

**方案 2：`--network host`（Linux）**

```bash
docker run -d \
  --network host \
  --name zeroclaw \
  -v ~/.zeroclaw:/root/.zeroclaw \
  zeroclaw daemon
```

使用 `--network host` 后，容器共享宿主机的网络栈，所以 `127.0.0.1:11434` 可以直接访问：

```toml
default_provider = "ollama"
default_model = "llama3"
```

**方案 3：宿主机局域网 IP（任意操作系统）**

找到你宿主机的局域网 IP（例如 `192.168.1.100`）：

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

**方案 4：Docker Compose 搭配 Ollama**

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

> **提示：** 如果你从另一个容器连接 Ollama，确保 Ollama 监听的是 `0.0.0.0`（而不仅仅是 localhost）。在 Ollama 环境中设置 `OLLAMA_HOST=0.0.0.0`。

### 自定义提供商（任何 OpenAI 兼容 API）

```toml
default_provider = "custom"

[providers.custom]
base_url = "https://your-api.example.com/v1"
api_key = "your-key"
model = "your-model"
```

适用于任何实现了 OpenAI Chat Completions 端点（`POST /v1/chat/completions`）的 API。

---

## 6. 频道

频道是用户与 ZeroClaw 交互的方式。可以同时运行多个频道。

### CLI（默认）

无需配置，直接运行：

```bash
zeroclaw agent
```

### Telegram

1. **创建 Bot**：在 Telegram 上找 [@BotFather](https://t.me/BotFather)：
   - 发送 `/newbot`
   - 选择名称和用户名
   - 复制 Bot Token

2. **配置：**

```toml
[channels_config.telegram]
token = "123456789:AABBccDDeeFFggHHiiJJ"
allowed_users = ["your_username"]   # without @, or numeric user ID
```

3. **启动 Gateway**（需要隧道，因为 Telegram 需要公网 HTTPS URL）：

```bash
zeroclaw gateway
```

4. **查找你的 User ID**（如果不确定）：
   - 给你的 Bot 发一条消息
   - 查看日志中的：`ignoring message from unauthorized user: <id>`
   - 把该 ID 添加到 `allowed_users`

### Discord

1. **创建 Bot**：在 [Discord Developer Portal](https://discord.com/developers/applications)：
   - New Application → Bot → 复制 Token
   - 在 Privileged Gateway Intents 下启用 **Message Content Intent**
   - 使用 `bot` + `applications.commands` 权限邀请 Bot 到你的服务器

2. **配置：**

```toml
[channels_config.discord]
token = "your-bot-token"
allowed_users = ["your-discord-user-id"]   # right-click user → Copy User ID
```

### Slack

1. **创建 Slack App**：在 [api.slack.com/apps](https://api.slack.com/apps)：
   - Create New App → From Scratch
   - 添加 Bot Token Scopes：`chat:write`、`app_mentions:read`、`im:history`、`im:read`、`im:write`
   - 安装到工作区
   - 启用 Socket Mode 并获取 App-Level Token（`xapp-...`）

2. **配置：**

```toml
[channels_config.slack]
token = "xoxb-your-bot-token"
app_token = "xapp-your-app-token"
allowed_users = ["U01ABCDEF"]   # Slack member ID
```

### WhatsApp

WhatsApp 使用 Meta 的 Business Cloud API（基于推送的 Webhook）：

1. 在 [developers.facebook.com](https://developers.facebook.com) 创建 Meta Business App
2. 添加 WhatsApp 产品
3. 获取 Access Token、Phone Number ID，并定义 Verify Token

```toml
[channels_config.whatsapp]
access_token = "EAABx..."
phone_number_id = "123456789012345"
verify_token = "my-secret-verify-token"
allowed_numbers = ["+1234567890"]   # E.164 format
```

4. 启动带隧道的 Gateway（WhatsApp 需要 HTTPS）
5. 在 Meta Developer Console → WhatsApp → Configuration → Webhook 中：
   - Callback URL：`https://your-tunnel-url/whatsapp`
   - Verify Token：与配置中相同
   - 订阅 `messages` 字段

### Matrix

```toml
[channels_config.matrix]
homeserver = "https://matrix.org"
user = "@yourbot:matrix.org"
password = "bot-password"
allowed_users = ["@you:matrix.org"]
```

### Webhook（自定义集成）

Gateway 暴露了一个 Webhook 端点用于自定义集成：

```bash
# Send a message via webhook
curl -X POST http://localhost:8080/webhook \
  -H "Authorization: Bearer <pairing-token>" \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from my app"}'
```

### 频道诊断

诊断频道问题：

```bash
zeroclaw channel doctor
```

这会检查所有已配置频道的 Token 有效性、Webhook 连通性和白名单配置。

---

## 7. 工具

工具是 Agent 在对话过程中可以调用的能力。ZeroClaw 自带一组内置工具，同时支持自定义扩展。

### 内置工具

| 工具 | 说明 |
|------|------|
| `shell` | 执行 Shell 命令（受自主级别设置约束） |
| `file_read` | 从工作区读取文件内容 |
| `file_write` | 在工作区中写入/创建文件 |
| `memory_store` | 将信息保存到长期记忆 |
| `memory_recall` | 搜索和检索记忆 |
| `memory_forget` | 删除特定记忆 |
| `browser_open` | 打开 URL（Brave 搜索 + 白名单） |
| `browser` | 完整的 Agent 浏览器 / Rust 原生浏览器自动化 |
| `composio` | 可选的 Composio 集成，用于第三方 API |

### 工具安全

工具遵循 `[autonomy]` 配置：

- **`readonly`** —— 仅 `file_read`、`memory_recall` 和 `browser_open` 可用
- **`supervised`** —— 所有工具可用，但破坏性操作需要确认
- **`full`** —— 所有工具可用，无需确认

Shell 命令会通过 `allowed_commands` 和 `forbidden_paths` 进行过滤：

```toml
[autonomy]
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep", "python3"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]
```

### 添加自定义工具

可以通过 Skills 系统（见 [§12 高级功能](#12-高级功能)）或在 Rust 中实现 `Tool` trait 来添加自定义工具。

---

## 8. 记忆系统

ZeroClaw 内置了一个全栈搜索引擎用于 Agent 记忆 —— 不需要任何外部依赖。

### 架构

| 层 | 实现 |
|----|------|
| Vector DB | Embedding 以 BLOB 形式存储在 SQLite 中，余弦相似度 |
| 关键词搜索 | FTS5 虚拟表 + BM25 评分 |
| 混合合并 | 自定义加权合并（`vector.rs`） |
| Embedding | `EmbeddingProvider` trait —— OpenAI、自定义 URL 或 noop |
| 分块 | 基于行的 Markdown 分块器，保留标题结构 |
| 缓存 | SQLite `embedding_cache` 表，LRU 淘汰策略 |
| 重建索引 | 原子性重建 FTS5 + 重新生成缺失的向量 |

### 后端

#### SQLite（默认，推荐）

```toml
[memory]
backend = "sqlite"
auto_save = true
embedding_provider = "openai"
vector_weight = 0.7
keyword_weight = 0.3
```

完整的混合搜索，同时支持关键词（BM25）和向量（余弦相似度）排序。准确度最高。

#### Lucid Bridge

```toml
[memory]
backend = "lucid"
```

与 Lucid CLI 工具同步，用于外部记忆管理。Lucid 不可用时回退到本地 SQLite。通过环境变量配置（见 [§4 配置参考](#4-配置参考)）。

#### Markdown

```toml
[memory]
backend = "markdown"
```

基于文件的简单记忆，使用 Markdown 文件。没有向量搜索 —— 仅支持关键词匹配。适合极简部署。

#### None（禁用）

```toml
[memory]
backend = "none"
```

显式的空操作后端。完全不持久化。适用于无状态/临时部署。

### Embedding 提供商

要使向量搜索生效，你需要一个 Embedding 提供商：

```toml
[memory]
embedding_provider = "openai"   # Uses OpenAI's embedding API (requires api_key)
# embedding_provider = "noop"   # Disables vector search, keyword-only
```

也可以使用自定义 Embedding 端点：

```toml
[memory]
embedding_provider = "custom"

[memory.embedding]
base_url = "http://localhost:8080/v1"
model = "your-embedding-model"
```

### 从 OpenClaw 迁移记忆

```bash
# Preview what will be migrated (safe, no changes)
zeroclaw migrate openclaw --dry-run

# Run the migration
zeroclaw migrate openclaw
```

这会将 OpenClaw 的 MEMORY.md、每日笔记和工作区文件导入到 ZeroClaw 的记忆后端。

---

## 9. 安全

ZeroClaw 在每一层都强制执行安全策略 —— 不仅仅是沙箱。

### 安全检查清单

| # | 项目 | 实现方式 |
|---|------|----------|
| 1 | Gateway 不暴露到公网 | 默认绑定 `127.0.0.1`。没有隧道或显式 `allow_public_bind = true` 时拒绝绑定 `0.0.0.0` |
| 2 | 需要配对 | 启动时生成 6 位一次性配对码。通过 `POST /pair` 交换获取 Bearer Token |
| 3 | 文件系统受限 | 默认 `workspace_only = true`。14 个系统目录 + 4 个敏感 dotfile 被屏蔽。检测符号链接逃逸 |
| 4 | 仅通过隧道访问 | 没有活跃隧道时 Gateway 拒绝公网绑定 |

### 配对流程

1. 启动 Gateway：`zeroclaw gateway`
2. 控制台会打印一个 6 位配对码
3. 发送 `POST /pair` 并附带配对码，获取 Bearer Token
4. 后续所有 `/webhook` 请求都需要 `Authorization: Bearer <token>`

### 文件系统限制

当 `workspace_only = true`（默认）时：

- 文件工具只能访问工作区目录
- 14 个系统目录被屏蔽（`/etc`、`/proc`、`/sys` 等）
- 4 个敏感 dotfile 被屏蔽（`~/.ssh`、`~/.gnupg`、`~/.aws` 等）
- 阻止空字节注入
- 通过路径规范化检测符号链接逃逸

### Docker 沙箱

为了最大程度的隔离，可以在 Docker 容器中运行工具：

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

### 速率限制

ZeroClaw 内置速率限制（默认每小时 20 次操作），防止 Agent 失控。

---

## 10. 部署指南

### 本地开发

```bash
# Build and run
cargo build --release
./target/release/zeroclaw onboard --interactive
./target/release/zeroclaw daemon
```

### Raspberry Pi / 边缘设备

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

另见：[docs/network-deployment.md](network-deployment.md) 了解 Raspberry Pi + Telegram 的详细部署方案。

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

### systemd 服务

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

### 云 VPS

1. 开一台小型 VPS（512 MB RAM 绰绰有余）
2. 安装 Rust 并构建 ZeroClaw
3. 配置隧道以便 Webhook 访问：

```toml
[tunnel]
kind = "cloudflare"

[tunnel.cloudflare]
token = "your-cloudflare-tunnel-token"
```

4. 安装为服务：`zeroclaw service install`
5. 将频道的 Webhook 指向隧道 URL

---

## 11. 身份与个性

ZeroClaw 支持通过工作区文件自定义 Agent 身份。

### 工作区文件

| 文件 | 用途 |
|------|------|
| `SOUL.md` | Agent 的个性、语气和行为准则 |
| `USER.md` | 关于 Agent 所服务的人类的信息 |
| `AGENTS.md` | 工作区规则、记忆协议、安全准则 |
| `TOOLS.md` | 本地环境备注（摄像头名称、SSH 主机等） |
| `HEARTBEAT.md` | 心跳系统的周期性任务 |
| `MEMORY.md` | 长期精选记忆 |
| `memory/YYYY-MM-DD.md` | 每日会话日志 |

### 身份格式

#### OpenClaw Markdown（默认）

将 Markdown 文件放在工作区目录中。Agent 在会话开始时读取它们。

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

在配置中指定格式：

```toml
[identity]
format = "openclaw"   # or "aieos"
```

---

## 12. 高级功能

### 心跳系统

心跳会定期唤醒 Agent，执行 `HEARTBEAT.md` 中定义的后台任务：

```markdown
# HEARTBEAT.md
- Check for unread emails
- Review calendar for upcoming events
- Check weather if going out today
```

Agent 在心跳轮询时轮流执行这些任务，状态记录在 `memory/heartbeat-state.json` 中。

### Skills

Skills 是打包好的能力，包含 TOML 清单和 SKILL.md 说明：

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

可以安装社区 Skills 或创建自己的。详见 [docs/adding-boards-and-tools.md](adding-boards-and-tools.md)。

### Cron / 定时任务

调度周期性或一次性任务：

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

### 费用追踪

ZeroClaw 会追踪 API 使用量和费用：

```bash
zeroclaw status   # Shows current session cost
```

配置预算限制：

```toml
[cost]
daily_limit_usd = 5.00
monthly_limit_usd = 50.00
warn_at_percent = 80
```

### RAG（检索增强生成）

ZeroClaw 的记忆系统同时也是一个 RAG 管线。当 Agent 收到查询时：

1. 混合搜索（FTS5 + 向量）检索相关的记忆片段
2. 结果通过 BM25 和余弦相似度分数的加权合并进行排序
3. 排名靠前的片段作为上下文注入到 Prompt 中

不需要外部向量数据库或框架。

### 集成注册表

ZeroClaw 自带 50+ 集成，涵盖 9 个类别。查看可用集成：

```bash
zeroclaw integrations list
zeroclaw integrations info Telegram
```

---

## 13. CLI 参考

### 核心命令

| 命令 | 说明 |
|------|------|
| `zeroclaw onboard` | 初始设置向导 |
| `zeroclaw onboard --interactive` | 交互式设置 |
| `zeroclaw onboard --api-key KEY --provider PROV` | 非交互式设置 |
| `zeroclaw onboard --channels-only` | 仅修复频道配置 |
| `zeroclaw agent` | 交互式聊天 REPL |
| `zeroclaw agent -m "message"` | 单条消息模式 |
| `zeroclaw gateway` | 启动 Webhook 服务器（默认：127.0.0.1:8080） |
| `zeroclaw gateway --port PORT` | 指定端口启动（0 = 随机） |
| `zeroclaw daemon` | 完整自主运行时 |
| `zeroclaw status` | 显示系统状态 |
| `zeroclaw doctor` | 运行系统诊断 |
| `zeroclaw channel doctor` | 检查频道健康状态 |

### 服务管理

| 命令 | 说明 |
|------|------|
| `zeroclaw service install` | 安装为 systemd 服务 |
| `zeroclaw service status` | 检查服务状态 |

### 迁移

| 命令 | 说明 |
|------|------|
| `zeroclaw migrate openclaw --dry-run` | 预览迁移（不做任何更改） |
| `zeroclaw migrate openclaw` | 从 OpenClaw 迁移 |

### 集成

| 命令 | 说明 |
|------|------|
| `zeroclaw integrations list` | 列出可用集成 |
| `zeroclaw integrations info NAME` | 显示集成详情 |

---

## 14. 故障排除

### 构建错误

#### OpenSSL 错误（Linux）

```bash
# Debian/Ubuntu
sudo apt install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel pkg-config
```

#### 编译时内存不足

在 Raspberry Pi 3（1 GB RAM）上很常见：

```bash
# Limit parallel compilation jobs
CARGO_BUILD_JOBS=1 cargo build --release

# Or add swap
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Docker 网络

#### 从 Docker 无法连接 Ollama

**症状：** Docker 中的 ZeroClaw 尝试访问宿主机上的 Ollama 时出现 `Connection refused`。

**原因：** 容器内的 `127.0.0.1` 指向的是容器自身，而不是宿主机。

**解决方案：** 见 [§5 服务提供商 → Ollama + ZeroClaw 在 Docker 中](#️-ollama--zeroclaw-在-docker-中常见问题) 的四种方案。

#### Ollama 不接受外部连接

**症状：** 从宿主机可以连接，但从 Docker 容器无法连接。

**原因：** Ollama 默认只监听 `127.0.0.1`。

**解决方案：** 启动 Ollama 前设置 `OLLAMA_HOST=0.0.0.0`：

```bash
OLLAMA_HOST=0.0.0.0 ollama serve
```

### 频道问题

#### "Ignoring message from unauthorized user"

你的发送者身份不在白名单中。

1. 查看日志中的确切发送者身份
2. 将其添加到对应的 `allowed_users` / `allowed_numbers` 列表
3. 重启：`zeroclaw onboard --channels-only`

#### Telegram Webhook 收不到消息

1. 确保 Gateway 正在运行且有公网隧道
2. 检查隧道是否活跃：`zeroclaw status`
3. 验证 Telegram 中的 Webhook URL 设置正确（BotFather → `/setwebhook`）
4. 运行 `zeroclaw channel doctor`

### 记忆 / Embedding 问题

#### "Embedding provider not configured"

你需要一个 Embedding 提供商才能使用向量搜索：

```toml
[memory]
embedding_provider = "openai"   # Requires api_key
```

或者禁用向量搜索：

```toml
[memory]
embedding_provider = "noop"
vector_weight = 0.0
keyword_weight = 1.0
```

#### 记忆搜索没有返回结果

1. 检查后端是否不是 `"none"`
2. 验证记忆是否已保存：`zeroclaw agent -m "What do you remember?"`
3. 尝试重建索引：Agent 的 `memory_recall` 工具会自动触发搜索

---

## 15. 常见问题

**问：使用 ZeroClaw 需要 API key 吗？**
答：只有使用云端 AI 提供商时才需要。使用 Ollama（本地模型）不需要 API key。

**问：可以同时使用多个提供商吗？**
答：你设置一个 `default_provider`，但可以在每次对话中切换模型。OpenRouter 通过一个 API key 就能访问 100+ 模型。

**问：怎么更新 ZeroClaw？**
答：拉取最新源码并重新构建：
```bash
cd zeroclaw
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

**问：可以在 Raspberry Pi Zero 上运行 ZeroClaw 吗？**
答：二进制文件可以正常运行，但编译需要在更强的机器上交叉编译。Pi Zero 的 512 MB RAM 不够编译 Rust。

**问：怎么备份数据？**
答：备份 `~/.zeroclaw/`（配置 + SQLite 记忆数据库）和你的工作区目录。

**问：ZeroClaw 兼容 OpenClaw 的 Skills 吗？**
答：ZeroClaw 使用 TOML Skill 清单，格式与 OpenClaw 不同，但迁移工具（`zeroclaw migrate openclaw`）会处理转换。

**问：可以运行多个 ZeroClaw 实例吗？**
答：可以，使用不同的配置目录：`ZEROCLAW_CONFIG_DIR=~/.zeroclaw-2 zeroclaw daemon`

**问：怎么重置所有设置？**
答：删除配置目录并重新初始化：
```bash
rm -rf ~/.zeroclaw
zeroclaw onboard --interactive
```

**问：`gateway` 和 `daemon` 有什么区别？**
答：`gateway` 只运行 Webhook 服务器。`daemon` 运行完整的自主运行时：Gateway + 心跳 + 频道轮询 + 定时任务。

**问：ZeroClaw 安全吗？**
答：ZeroClaw 默认只绑定 localhost，需要配对 Token，限制文件系统访问，支持 Docker 沙箱，并强制执行频道白名单。运行 `nmap -p 1-65535 <your-host>` 验证没有端口暴露。详见 [§9 安全](#9-安全)。

---

*本文档持续更新中。欢迎在 [github.com/zeroclaw-labs/zeroclaw](https://github.com/zeroclaw-labs/zeroclaw) 贡献。*
