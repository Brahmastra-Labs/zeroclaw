[English](README.md) | **中文**

<p align="center">
  <img src="zeroclaw.png" alt="ZeroClaw" width="200" />
</p>

<h1 align="center">ZeroClaw 🦀</h1>

<p align="center">
  <strong>零开销。零妥协。100% Rust。100% 不绑定。</strong><br>
  ⚡️ <strong>在 $10 硬件上运行，内存占用不到 5MB：比 OpenClaw 少 99% 内存，比 Mac mini 便宜 98%！</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
</p>

快速、轻量、完全自主的 AI 助手基础设施 ── 随处部署，随意替换。

```
~3.4MB binary · <10ms startup · 1,017 tests · 22+ providers · 8 traits · Pluggable everything
```

### ✨ 特性

- 🏎️ **超轻量：** 内存占用不到 5MB ── 比 OpenClaw 核心小 99%。
- 💰 **极低成本：** 高效到能在 $10 硬件上运行 ── 比 Mac mini 便宜 98%。
- ⚡ **闪电启动：** 启动速度快 400 倍，<10ms 冷启动（0.6GHz 核心上也不到 1 秒）。
- 🌍 **真正可移植：** 单个自包含二进制文件，支持 ARM、x86 和 RISC-V。

### 为什么团队选择 ZeroClaw

- **默认精简：** 小巧的 Rust 二进制文件，启动快，内存占用低。
- **安全优先：** 配对机制、严格沙箱、显式白名单、工作区隔离。
- **完全可替换：** 核心子系统都是 trait（providers、channels、tools、memory、tunnels）。
- **无厂商锁定：** 支持 OpenAI 兼容的 provider + 可插拔的自定义端点。

## 基准测试快照（ZeroClaw vs OpenClaw）

本地快速基准测试（macOS arm64，2026 年 2 月），已针对 0.8GHz 边缘硬件归一化。

| | OpenClaw | NanoBot | PicoClaw | ZeroClaw 🦀 |
|---|---|---|---|---|
| **语言** | TypeScript | Python | Go | **Rust** |
| **内存** | > 1GB | > 100MB | < 10MB | **< 5MB** |
| **启动时间 (0.8GHz core)** | > 500s | > 30s | < 1s | **< 10ms** |
| **二进制大小** | ~28MB (dist) | N/A (Scripts) | ~8MB | **3.4 MB** |
| **成本** | Mac Mini $599 | Linux SBC ~$50 | Linux Board $10 | **任意硬件 $10** |

> 注：ZeroClaw 数据使用 `/usr/bin/time -l` 在 release 构建上测量。OpenClaw 需要 Node.js 运行时（约 390MB 额外开销）。PicoClaw 和 ZeroClaw 均为静态二进制文件。

在本地复现 ZeroClaw 的测试数据：

```bash
cargo build --release
ls -lh target/release/zeroclaw

/usr/bin/time -l target/release/zeroclaw --help
/usr/bin/time -l target/release/zeroclaw status
```

## 前置要求

<details>
<summary><strong>Windows</strong></summary>

#### 必需

1. **Visual Studio Build Tools**（提供 MSVC 链接器和 Windows SDK）：
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
   安装时（或通过 Visual Studio Installer），选择 **"Desktop development with C++"** 工作负载。

2. **Rust 工具链：**
   ```powershell
   winget install Rustlang.Rustup
   ```
   安装完成后，打开新终端并运行 `rustup default stable` 以确保使用 stable 工具链。

3. **验证**安装是否正常：
   ```powershell
   rustc --version
   cargo --version
   ```

#### 可选

- **Docker Desktop** ── 仅在使用 Docker 沙箱运行时（`runtime.kind = "docker"`）时需要。通过 `winget install Docker.DockerDesktop` 安装。

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### 必需

1. **构建基础工具：**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf groupinstall "Development Tools" && sudo dnf install pkg-config`
   - **macOS:** 安装 Xcode Command Line Tools：`xcode-select --install`

2. **Rust 工具链：**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **验证**安装是否正常：
   ```bash
   rustc --version
   cargo --version
   ```

#### 可选

- **Docker** ── 仅在使用 Docker 沙箱运行时（`runtime.kind = "docker"`）时需要。

> **低内存开发板（如 Raspberry Pi 3，1GB RAM）：** 如果内核在编译时杀掉 rustc 进程，请使用 `CARGO_BUILD_JOBS=1 cargo build --release`。

</details>

## 快速开始

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked

# 快速设置（无交互提示）
zeroclaw onboard --api-key sk-... --provider openrouter

# 或使用交互式向导
zeroclaw onboard --interactive

# 或仅修复 channels/allowlists
zeroclaw onboard --channels-only

# 聊天
zeroclaw agent -m "Hello, ZeroClaw!"

# 交互模式
zeroclaw agent

# 启动 gateway（webhook 服务器）
zeroclaw gateway                # 默认: 127.0.0.1:8080
zeroclaw gateway --port 0       # 随机端口（安全加固）

# 启动完整自主运行时
zeroclaw daemon

# 查看状态
zeroclaw status

# 运行系统诊断
zeroclaw doctor

# 检查 channel 健康状态
zeroclaw channel doctor

# 获取集成配置详情
zeroclaw integrations info Telegram

# 管理后台服务
zeroclaw service install
zeroclaw service status

# 从 OpenClaw 迁移记忆（先安全预览）
zeroclaw migrate openclaw --dry-run
zeroclaw migrate openclaw
```

## 架构

每个子系统都是一个 **trait** ── 通过修改配置即可替换实现，无需改动代码。

| 子系统 | Trait | 内置实现 | 扩展方式 |
|--------|-------|----------|----------|
| **AI 模型** | `Provider` | 22+ providers（OpenRouter、Anthropic、OpenAI、Ollama、Venice、Groq、Mistral、xAI、DeepSeek、Together、Fireworks、Perplexity、Cohere、Bedrock 等） | `custom:https://your-api.com` ── 任何 OpenAI 兼容 API |
| **消息通道** | `Channel` | CLI、Telegram、Discord、Slack、iMessage、Matrix、WhatsApp、Webhook | 任何消息 API |
| **记忆** | `Memory` | SQLite 混合搜索（FTS5 + 向量余弦相似度）、Lucid bridge、Markdown | 任何持久化后端 |
| **工具** | `Tool` | shell、file_read、file_write、memory_store、memory_recall、memory_forget、browser_open、browser、composio | 任何能力 |
| **可观测性** | `Observer` | Noop、Log、Multi | Prometheus、OTel |
| **运行时** | `RuntimeAdapter` | Native、Docker（沙箱） | WASM（计划中） |
| **安全** | `SecurityPolicy` | Gateway 配对、沙箱、白名单、速率限制、文件系统隔离、加密密钥 | ── |
| **身份** | `IdentityConfig` | OpenClaw (markdown)、AIEOS v1.1 (JSON) | 任何身份格式 |
| **隧道** | `Tunnel` | None、Cloudflare、Tailscale、ngrok、Custom | 任何隧道程序 |

### 运行时支持（当前）

- ✅ 已支持：`runtime.kind = "native"` 或 `runtime.kind = "docker"`
- 🚧 计划中，尚未实现：WASM / 边缘运行时

### 记忆系统（全栈搜索引擎）

全部自研，零外部依赖：

| 层级 | 实现方式 |
|------|----------|
| **向量数据库** | Embeddings 以 BLOB 形式存储在 SQLite 中，余弦相似度搜索 |
| **关键词搜索** | FTS5 虚拟表 + BM25 评分 |
| **混合合并** | 自定义加权合并函数（`vector.rs`） |
| **Embeddings** | `EmbeddingProvider` trait ── OpenAI、自定义 URL 或 noop |
| **分块** | 基于行的 Markdown 分块器，保留标题结构 |
| **缓存** | SQLite `embedding_cache` 表 + LRU 淘汰策略 |
| **安全重建索引** | 原子性重建 FTS5 + 重新嵌入缺失向量 |

```toml
[memory]
backend = "sqlite"
auto_save = true
embedding_provider = "openai"
vector_weight = 0.7
keyword_weight = 0.3
```

## 安全

ZeroClaw 在每一层都强制执行安全策略。

### 安全检查清单

| # | 项目 | 状态 | 实现方式 |
|---|------|------|----------|
| 1 | **Gateway 不公开暴露** | ✅ | 默认绑定 `127.0.0.1`。没有隧道或显式 `allow_public_bind = true` 时拒绝绑定 `0.0.0.0`。 |
| 2 | **需要配对** | ✅ | 启动时生成 6 位一次性配对码。通过 `POST /pair` 交换 bearer token。 |
| 3 | **文件系统隔离** | ✅ | 默认 `workspace_only = true`。屏蔽 14 个系统目录 + 4 个敏感 dotfiles。检测符号链接逃逸。 |
| 4 | **仅通过隧道访问** | ✅ | 没有活跃隧道时，Gateway 拒绝公开绑定。 |

### Channel 白名单

- 空白名单 = 拒绝所有入站消息
- `"*"` = 允许所有（需显式启用）
- 其他情况 = 精确匹配白名单

### WhatsApp Business Cloud API 配置

WhatsApp 使用 Meta 的 Cloud API + webhooks（推送模式）：

1. 在 developers.facebook.com 创建 Meta Business App
2. 添加 WhatsApp 产品
3. 获取 Access Token、Phone Number ID，定义 Verify Token

```toml
[channels_config.whatsapp]
access_token = "EAABx..."
phone_number_id = "123456789012345"
verify_token = "my-secret-verify-token"
allowed_numbers = ["+1234567890"]
```

4. 启动带隧道的 gateway（WhatsApp 要求 HTTPS）
5. 在 Meta 后台配置 webhook 回调 URL

## 配置

配置文件：`~/.zeroclaw/config.toml`（由 `onboard` 创建）

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-20250514"
default_temperature = 0.7

[memory]
backend = "sqlite"
auto_save = true
embedding_provider = "openai"
vector_weight = 0.7
keyword_weight = 0.3

[gateway]
require_pairing = true
allow_public_bind = false

[autonomy]
level = "supervised"
workspace_only = true
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]

[runtime]
kind = "native"

[runtime.docker]
image = "alpine:3.20"
network = "none"
memory_limit_mb = 512
cpu_limit = 1.0
read_only_rootfs = true
mount_workspace = true

[heartbeat]
enabled = false
interval_minutes = 30

[tunnel]
provider = "none"

[secrets]
encrypt = true

[browser]
enabled = false
allowed_domains = ["docs.rs"]
backend = "agent_browser"
native_headless = true
native_webdriver_url = "http://127.0.0.1:9515"

[browser.computer_use]
endpoint = "http://127.0.0.1:8787/v1/actions"
timeout_ms = 15000
allow_remote_endpoint = false

[composio]
enabled = false
entity_id = "default"

[identity]
format = "openclaw"
```

## 身份系统（AIEOS 支持）

支持两种格式：

### OpenClaw（默认）
Markdown 文件：IDENTITY.md、SOUL.md、USER.md、AGENTS.md

### AIEOS（AI Entity Object Specification）
AIEOS v1.1 JSON 载荷，用于可移植的 AI 身份。

```toml
[identity]
format = "aieos"
aieos_path = "identity.json"
```

## Gateway API

| 端点 | 方法 | 认证 | 描述 |
|------|------|------|------|
| `/health` | GET | 无 | 健康检查 |
| `/pair` | POST | `X-Pairing-Code` | 用配对码交换 bearer token |
| `/webhook` | POST | `Bearer <token>` | 发送消息 |
| `/whatsapp` | GET | Query params | Meta webhook 验证 |
| `/whatsapp` | POST | 无（Meta 签名） | WhatsApp 入站 webhook |

## 命令

| 命令 | 描述 |
|------|------|
| `onboard` | 快速设置 |
| `onboard --interactive` | 完整交互式向导 |
| `onboard --channels-only` | 仅重新配置 channels |
| `agent -m "..."` | 单条消息模式 |
| `agent` | 交互式聊天 |
| `gateway` | 启动 webhook 服务器 |
| `gateway --port 0` | 随机端口模式 |
| `daemon` | 完整自主运行时 |
| `service install/start/stop/status/uninstall` | 管理服务 |
| `doctor` | 系统诊断 |
| `status` | 系统状态 |
| `channel doctor` | Channel 健康检查 |
| `integrations info <name>` | 集成详情 |

## 开发

```bash
cargo build
cargo build --release
CARGO_BUILD_JOBS=1 cargo build --release    # 低内存回退方案
cargo test               # 1,017 tests
cargo clippy
cargo fmt

cargo test --test memory_comparison -- --nocapture
```

### Pre-push hook

```bash
git config core.hooksPath .githooks
```

### 构建故障排除（Linux OpenSSL 错误）

```bash
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

## 协作与文档

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [docs/pr-workflow.md](docs/pr-workflow.md)
- [docs/reviewer-playbook.md](docs/reviewer-playbook.md)
- [docs/ci-map.md](docs/ci-map.md)
- [SECURITY.md](SECURITY.md)

## 支持

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 特别感谢

- **Harvard University**
- **MIT**
- **Sundai Club**
- **The World & Beyond** 🌍✨

## 许可证

MIT ── 详见 [LICENSE](LICENSE)

## 贡献

参见 [CONTRIBUTING.md](CONTRIBUTING.md)。实现一个 trait，提交 PR。

## Star 历史

<p align="center">
  <a href="https://www.star-history.com/#zeroclaw-labs/zeroclaw&Date">
    <img src="https://api.star-history.com/svg?repos=zeroclaw-labs/zeroclaw&type=Date" alt="Star History Chart" />
  </a>
</p>

---
**ZeroClaw** ── 零开销。零妥协。随处部署。随意替换。🦀
