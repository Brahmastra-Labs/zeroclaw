[English](../README.md) | **中文**

<p align="center">
  <img src="zeroclaw.png" alt="ZeroClaw" width="200" />
</p>

<h1 align="center">ZeroClaw 🦀</h1>

<p align="center">
<strong>零开销。零妥协。 100% Rust。 100% 通用。</strong><br>
⚡️ <strong>在 10 美元硬件和 <5MB RAM 上运行：内存比 OpenClaw 少 99%，比 Mac mini 便宜 98%！</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License: Apache 2.0" /></a>
  <a href="NOTICE"><img src="https://img.shields.io/badge/contributors-27+-green.svg" alt="Contributors" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
</p>

快速、小巧且完全自主的 AI 助手基础设施 —— 随处部署，万物可换。

```
~3.4MB 二进制文件 · <10ms 启动时间 · 1,017 个测试 · 22+ 提供商（providers） · 8 个 Trait · 万物可插拔
```

### ✨ 特性

- 🏎️ **超轻量级：** <5MB 内存占用 — 比 OpenClaw 核心小 99%。
- 💰 **最低成本：** 足够高效，可以在 10 美元的硬件上运行 — 比 Mac mini 便宜 98%。
- ⚡ **闪电般快速：** 启动时间加快 400 倍，启动时间 <10 毫秒（即使在 0.6GHz 内核上，启动时间也低于 1 秒）。
- 🌍 **真正的可移植性：**跨ARM、x86 和RISC-V 的单个独立二进制文件。

### 为什么团队选择 ZeroClaw

- **默认精简：** 小巧的 Rust 二进制文件，快速启动，低内存占用。
- **设计安全：** 配对机制、严格沙箱、显式白名单、工作区作用域。
- **完全可交换：**核心系统均为 Trait（提供商（providers）、渠道（channels）、工具（tools）、记忆（memory）、隧道（tunnels））。
- **无锁定：** OpenAI 兼容的提供商（provider）支持 + 可插拔的自定义端点。

## 基准测试快照 (ZeroClaw vs OpenClaw)

本地机器快速基准测试（macOS arm64，2026 年 2 月），针对 0.8GHz 边缘硬件进行了归一化处理。

|  | OpenClaw | NanoBot | PicoClaw | ZeroClaw 🦀 |
|---|---|---|---|---|
| **语言** | TypeScript | Python | Go | **Rust** |
| **内存** | > 1GB | > 100MB | <10MB | **< 5MB** |
| **启动（0.8GHz 核心）** | > 500秒 | > 30秒 | <1秒 | **< 10 毫秒** |
| **二进制大小** | ~28MB (dist) | N/A (脚本) | ~8MB | **3.4 MB** |
| **成本** | Mac Mini $599 | Linux SBC ~$50 | Linux 开发板 $10 | **任意硬件 $10** |

> 注：注意：ZeroClaw 结果是在发布版本 (release builds) 上使用 `/usr/bin/time -l` 测量的。OpenClaw 需要 Node.js 运行时（~390MB 开销）。PicoClaw 和 ZeroClaw 是静态二进制文件。

<p align="center">
  <img src="zero-claw.jpeg" alt="ZeroClaw vs OpenClaw Comparison" width="800" />
</p>

在本地复现 ZeroClaw 数据：

```bash
cargo build --release
ls -lh target/release/zeroclaw

/usr/bin/time -l target/release/zeroclaw --help
/usr/bin/time -l target/release/zeroclaw status
```

## 先决条件

<details>
<summary><strong>Windows</strong></summary>

#### 必需

1. **Visual Studio Build Tools**（提供MSVC链接器和Windows SDK）：
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```
在安装过程中（或通过 Visual Studio 安装程序），选择 **“使用 C++ 进行桌面开发”** 工作负载。

2. **Rust工具链：**
   ```powershell
   winget install Rustlang.Rustup
   ```
安装后，打开一个新终端并运行 `rustup default stable` 以确保稳定工具链处于活动状态。

3. **验证**两者都正常工作：
   ```powershell
   rustc --version
   cargo --version
   ```

#### 可选

- **Docker 桌面** — 仅在使用 [Docker 沙盒运行时](#runtime-support-current) (`runtime.kind = "docker"`) 时才需要。通过`winget install Docker.DockerDesktop`安装。

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### 必需

1. **构建要点：**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf groupinstall "Development Tools" && sudo dnf install pkg-config`
   - **macOS:** 安装 Xcode 命令行 Tools: `xcode-select --install`

2. **Rust工具链：**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
详情请参阅[rustup.rs](https://rustup.rs)。

3. **验证**两者都正常工作：
   ```bash
   rustc --version
   cargo --version
   ```

#### 可选

- **Docker** — 仅当使用[Docker 沙盒运行时](#runtime-support-current) (`runtime.kind = "docker"`) 时才需要。通过包管理器或[docker.com](https://docs.docker.com/engine/install/)安装。

> **注意：** 默认`cargo build --release` 默认使用 `codegen-units=1`，可兼容低内存设备（例如 1GB RAM 的 Raspberry Pi 3）。如需在高性能机器上更快构建，可使用 `cargo build --profile release-fast`。

</details>


## 快速入门

```bash
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw
cargo build --release --locked
cargo install --path . --force --locked

# 快速设置（无提示）
zeroclaw onboard --api-key sk-... --provider openrouter

# 或者交互式向导
zeroclaw onboard --interactive

# 或者仅快速修复频道/白名单
zeroclaw onboard --channels-only

# 聊天
zeroclaw agent -m "Hello, ZeroClaw!"

# 交互模式
zeroclaw agent

# 启动网关（webhook 服务器）
zeroclaw gateway                # 默认：127.0.0.1:8080
zeroclaw gateway --port 0       # 随机端口（安全强化）

# 启动完全自主运行时
zeroclaw daemon

# 检查状态
zeroclaw status

# 运行系统诊断
zeroclaw doctor

# 检查频道健康状况
zeroclaw channel doctor

# 获取集成设置详细信息
zeroclaw integrations info Telegram

# 管理后台服务
zeroclaw service install
zeroclaw service status

# 从OpenClaw迁移内存（首先安全预览）
zeroclaw migrate openclaw --dry-run
zeroclaw migrate openclaw
```

> **开发后备（无全局安装）：** 在命令前添加`cargo run --release --`（示例：`cargo run --release -- status`）。

## 建筑学

每个子系统都是一个**特征**——通过配置更改来交换实现，零代码更改。

<p align="center">
  <img src="docs/architecture.svg" alt="ZeroClaw Architecture" width="900" />
</p>

| 子系统 | 特征 | 发货时附带 | 延长 |
|-----------|-------|------------|--------|
| **人工智能模型** | `Provider` | 22+ 提供商（providers）（OpenRouter、Anthropic、OpenAI、Ollama、Venice、Groq、Mistral、xAI、DeepSeek、Together、Fireworks、Perplexity、Cohere、Bedrock 等） | `custom:https://your-api.com` — 任何 OpenAI 兼容 API |
| **Channels** | `Channel` | CLI、Telegram、Discord、Slack、iMessage、矩阵、WhatsApp、Webhook | 任何消息API |
| **Memory** | `Memory` | SQLite 混合搜索（FTS5 + 矢量余弦相似度）、Lucid 桥接（CLI 同步 + SQLite 后备）、Markdown | 任何持久化后端 |
| **Tools** | `Tool` | shell、file_read、file_write、memory_store、memory_recall、memory_forget、browser_open（Brave + 白名单）、浏览器（代理浏览器/rust-native）、composio（可选） | 任何能力 |
| **可观察性** | `Observer` | Noop、日志、多 | Prometheus, OTel |
| **运行时** | `RuntimeAdapter` | 本机，Docker（沙盒） | WASM（已计划；不受支持的类型会快速失败） |
| **安全** | `SecurityPolicy` | Gateway 配对、沙箱、白名单、速率限制、文件系统范围、加密秘密 | — |
| **身份** | `IdentityConfig` | OpenClaw（降价）、AIEOS v1.1 (JSON) | 任何身份格式 |
| **Tunnel** | `Tunnel` | 无、Cloudflare、Tailscale、ngrok、自定义 | 任何隧道二进制文件 |
| **心跳** | 引擎 | HEARTBEAT.md定期任务 | — |
| **技能** | 装载机 | TOML 清单 + SKILL.md 说明 | 社区技能包 |
| **整合** | 登记处 | 跨 9 个类别的 50 多个集成 | 插件系统 |

### 运行时支持（当前）

- ✅ 今天支持：`runtime.kind = "native"` 或 `runtime.kind = "docker"`
- 🚧 已计划，尚未实施：WASM / 边缘运行时

当配置了不受支持的 `runtime.kind` 时，ZeroClaw 现在会退出并显示明显的错误，而不是默默地回退到本机。

### Memory系统（全栈搜索引擎）

所有自定义，零外部依赖 - 没有Pinecone，没有Elasticsearch，没有LangChain：

| 层 | 执行 |
|-------|---------------|
| **矢量数据库** | 嵌入存储为BLOB in SQLite，余弦相似度搜索 |
| **关键字搜索** | FTS5 虚拟牌桌，BM25 计分 |
| **混合合并** | 自定义加权合并函数（`vector.rs`） |
| **嵌入** | `EmbeddingProvider` 特征 — OpenAI、自定义 URL 或 noop |
| **分块** | 具有标题保存功能的基于行的 Markdown 分块器 |
| **缓存** | SQLite `embedding_cache` 表，LRU 驱逐 |
| **安全重建索引** | 重建 FTS5 + 以原子方式重新嵌入缺失的向量 |

代理通过工具（tools）自动召回、保存和管理记忆。

```toml
[memory]
backend = "sqlite"          # “sqlite”、“清晰”、“markdown”、“无”
auto_save = true
embedding_provider = "openai"
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" 使用显式无操作内存后端（无持久性）

# 后端可选 =“lucid”
# ZEROCLAW_LUCID_CMD=/usr/local/bin/lucid # 默认值：lucid
# ZEROCLAW_LUCID_BUDGET=200 # 默认值：200
# ZEROCLAW_LUCID_LOCAL_HIT_THRESHOLD=3 # 本地命中计数以跳过外部调用
# ZEROCLAW_LUCID_RECALL_TIMEOUT_MS=120 # 清晰上下文回忆的低延迟预算
# ZEROCLAW_LUCID_STORE_TIMEOUT_MS=800 # lucid 存储的异步同步超时
# ZEROCLAW_LUCID_FAILURE_COOLDOWN_MS=15000 # 清醒失败后的冷却时间，以避免重复的缓慢尝试
```

## 安全

ZeroClaw 在**每一层**强制执行安全性——而不仅仅是沙箱。它通过了社区安全检查表中的所有项目。

### 安全检查表

| # | 物品 | 地位 | 如何 |
|---|------|--------|-----|
| 1 | **Gateway 未公开暴露** | ✅ | 默认绑定`127.0.0.1`。在没有隧道或显式`allow_public_bind = true`的情况下拒绝`0.0.0.0`。 |
| 2 | **需要配对** | ✅ | 启动时的 6 位一次性代码。通过`POST /pair` 交换不记名令牌。所有`/webhook` 请求都需要`Authorization: Bearer <token>`。 |
| 3 | **文件系统范围（无/）** | ✅ | 默认`workspace_only = true`。 14 个系统目录 + 4 个敏感点文件被阻止。空字节注入被阻止。通过文件读/写工具中的规范化+解析路径工作区检查进行符号链接转义检测。 |
| 4 | **只能通过隧道进入** | ✅ | Gateway 在没有活动隧道的情况下拒绝公共绑定。支持Tailscale、Cloudflare、ngrok或任何自定义隧道。 |

> **运行您自己的 nmap：** `nmap -p 1-65535 <your-host>` — ZeroClaw 仅绑定到本地主机，因此除非您显式配置隧道，否则不会暴露任何内容。

### Channel 白名单 (Telegram / Discord / Slack)

入站发件人策略现在是一致的：

- 空允许列表 = **拒绝所有入站消息**
- `"*"` = **允许所有**（明确选择加入）
- 否则 = 完全匹配白名单

默认情况下，这可以降低意外暴露的风险。

推荐的低摩擦设置（安全+快速）：

- **Telegram：** 将您自己的`@username`（不含`@`）和/或您的数字Telegram 用户ID 列入白名单。
- **Discord:** 将您自己的Discord 用户 ID 列入白名单。
- **Slack：** 将您自己的Slack 会员 ID 列入白名单（通常以 `U` 开头）。
- `"*"`仅用于临时开放测试。

如果您不确定要使用哪个身份：

1. 启动频道并向您的机器人发送一条消息。
2. 阅读警告日志以查看确切的发件人身份。
3. 将该值添加到白名单，并重新运行仅限通道（channels-only）的设置。

如果您在日志中遇到授权警告（例如：`ignoring message from unauthorized user`），
仅重新运行通道（channel）设置：

```bash
zeroclaw onboard --channels-only
```

### WhatsApp 业务云 API 设置

WhatsApp 使用 Meta 的 Cloud API 和 webhooks（基于推送，而不是轮询）：

1. **创建Meta商业应用程序：**
   - Go至[开发者.facebook.com](https://developers.facebook.com)
   - 创建新应用→选择“商业”类型
   - 添加“WhatsApp”产品

2. **获取您的凭据：**
   - **访问令牌：** 从 WhatsApp → API 设置 → 生成令牌（或为永久令牌创建系统用户）
   - **电话号码 ID：** 从 WhatsApp → API 设置 → 电话号码 ID
   - **验证令牌：** 您定义此（任何随机字符串） - Meta 将在 webhook 验证期间将其发回

3. **配置ZeroClaw:**
   ```toml
   [channels_config.whatsapp]
   access_token = "EAABx..."
   phone_number_id = "123456789012345"
   verify_token = "my-secret-verify-token"
   allowed_numbers = ["+1234567890"]  # E.164 格式，或 ["*"] 表示所有
   ```

4. **使用隧道启动网关：**
   ```bash
   zeroclaw gateway --port 8080
   ```
WhatsApp 需要 HTTPS，因此请使用隧道（ngrok、Cloudflare、Tailscale 漏斗）。

5. **配置Meta webhook：**
   - 在Meta开发者控制台→WhatsApp→配置→Webhook
   - **回调网址：** `https://your-tunnel-url/whatsapp`
   - **验证令牌：**与配置中的`verify_token`相同
   - 订阅`messages`字段

6. **测试：** 向您的 WhatsApp 企业号码发送消息 — ZeroClaw 将通过 LLM 进行回复。

## 配置

配置：`~/.zeroclaw/config.toml`（由`onboard`创建）

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-20250514"
default_temperature = 0.7

[memory]
backend = "sqlite"              # “sqlite”、“清晰”、“markdown”、“无”
auto_save = true
embedding_provider = "openai"   # “openai”、“noop”
vector_weight = 0.7
keyword_weight = 0.3

# backend = "none" 通过无操作后端禁用持久内存

[gateway]
require_pairing = true          # 首次连接时需要配对代码
allow_public_bind = false       # 拒绝没有隧道的 0.0.0.0

[autonomy]
level = "supervised"            # “只读”、“受监督”、“完整”（默认: 受监督）
workspace_only = true           # 默认值： true — 范围为工作区
allowed_commands = ["git", "npm", "cargo", "ls", "cat", "grep"]
forbidden_paths = ["/etc", "/root", "/proc", "/sys", "~/.ssh", "~/.gnupg", "~/.aws"]

[runtime]
kind = "native"                # “本地”或“码头工人”

[runtime.docker]
image = "alpine:3.20"          # 用于 shell 执行的容器镜像
network = "none"               # docker 网络模式（“无”、“桥接”等）
memory_limit_mb = 512          # 可选内存限制（MB）
cpu_limit = 1.0                # 可选CPU限制
read_only_rootfs = true        # 将根文件系统挂载为只读
mount_workspace = true         # 将工作空间挂载到 /workspace
allowed_workspace_roots = []   # 用于工作区安装验证的可选允许列表

[heartbeat]
enabled = false
interval_minutes = 30

[tunnel]
provider = "none"               # “无”、“cloudflare”、“tailscale”、“ngrok”、“自定义”

[secrets]
encrypt = true                  # API 使用本地密钥文件加密的密钥

[browser]
enabled = false                        # 选择加入 browser_open + 浏览器工具
allowed_domains = ["docs.rs"]         # 启用浏览器时需要
backend = "agent_browser"             # “agent_browser”（默认）、“rust_native”、“computer_use”、“auto”
native_headless = true                 # 当后端使用 Rust-native 时适用
native_webdriver_url = "http://127.0.0.1:9515" # WebDriver 端点（chromedriver/selenium）
# native_chrome_path = "/usr/bin/chromium" # 驱动程序的可选显式浏览器二进制文件

[browser.computer_use]
endpoint = "http://127.0.0.1:8787/v1/actions" # 计算机使用 sidecar HTTP 端点
timeout_ms = 15000                    # 每个操作超时
allow_remote_endpoint = false         # 安全默认值：仅私有/本地主机端点
window_allowlist = []                 # 可选的窗口标题/进程白名单提示
# api_key = "..." # sidecar 的可选承载令牌
# max_coordinate_x = 3840 # 可选坐标护栏
# max_coordinate_y = 2160 # 可选坐标护栏

# Rust-本机后端构建标志：
# Cargo build --release --features 浏览器原生
# 确保 WebDriver 服务器正在运行，例如chromedriver --端口=9515

# 计算机使用 sidecar 合约 (MVP)
# POST browser.computer_use.endpoint
# 要求： {
# “动作”：“鼠标单击”，
# “params”：{“x”：640，“y”：360，“按钮”：“左”}，
# “策略”：{“allowed_domains”：[...]，“window_allowlist”：[...]，“max_coordinate_x”：3840，“max_coordinate_y”：2160}，
# “元数据”：{“会话名称”：“...”，“源”：“zeroclaw.browser”，“版本”：“...”}
# }
# 响应：{"success": true, "data": {...}} 或 {"success": false, "error": "..."}

[composio]
enabled = false                 # 选择加入：通过 composio.dev 提供 1000 多个 OAuth 应用程序
# api_key = "cmp_..." # 可选：当 [secrets].encrypt = true 时加密存储
entity_id = "default"         # Composio 工具调用的默认 user_id

[identity]
format = "openclaw"             # “openclaw”（默认，markdown 文件）或“aieos”(JSON)
# aieos_path = "identity.json" # AIEOS JSON 文件的路径（相对于工作空间或绝对路径）
# aieos_inline = '{"identity":{"names":{"first":"Nova"}}}' # 内联 AIEOS JSON
```

## Python 伴侣套餐 (`zeroclaw-tools`)

对于具有不一致的本机工具调用的 LLM 提供程序（例如 GLM-5/Zhipu），ZeroClaw 附带了一个 Python 配套包以及 **基于 LangGraph 的工具调用**，以保证一致性：

```bash
pip install zeroclaw-tools
```

```python
from zeroclaw_tools import create_agent, shell, file_read
from langchain_core.messages import HumanMessage

# 可与任何 OpenAI 兼容的提供商合作
agent = create_agent(
    tools=[shell, file_read],
    model="glm-5",
    api_key="your-key",
    base_url="https://api.z.ai/api/coding/paas/v4"
)

result = await agent.ainvoke({
    "messages": [HumanMessage(content="List files in /tmp")]
})
print(result["messages"][-1].content)
```

**为什么使用它：**
- **所有提供商的一致工具调用**（即使是那些原生支持较差的提供商）
- **自动工具循环** — 不断调用工具直到任务完成
- **轻松扩展** - 使用 `@tool` 装饰器添加自定义工具
- **包括Discord 机器人集成**（Telegram 已计划）

有关完整文档，请参阅[`python/README.md`](python/README.md)。

## 身份系统（AIEOS 支持）

ZeroClaw 通过两种格式支持**身份无关**人工智能角色：

### OpenClaw（默认）

工作区中的传统 Markdown 文件：
- `IDENTITY.md` — 代理人是谁
- `SOUL.md` — 核心人格和价值观
- `USER.md` — 代理正在帮助谁
- `AGENTS.md` — 行为准则

### AIEOS（AI实体对象规范）

[AIEOS](https://aieos.org)是便携式人工智能身份的标准化框架。 ZeroClaw 支持AIEOS v1.1 JSON 有效负载，允许您：

- **从AIEOS生态系统导入身份**
- **导出身份**到其他AIEOS兼容系统
- **在不同的人工智能模型中保持行为完整性**

#### 启用 AIEOS

```toml
[identity]
format = "aieos"
aieos_path = "identity.json"  # 相对于工作空间或绝对路径
```

或内联JSON：

```toml
[identity]
format = "aieos"
aieos_inline = '''
{
  "identity": {
    "names": { "first": "Nova", "nickname": "N" }
  },
  "psychology": {
    "neural_matrix": { "creativity": 0.9, "logic": 0.8 },
    "traits": { "mbti": "ENTP" },
    "moral_compass": { "alignment": "Chaotic Good" }
  },
  "linguistics": {
    "text_style": { "formality_level": 0.2, "slang_usage": true }
  },
  "motivations": {
    "core_drive": "Push boundaries and explore possibilities"
  }
}
'''
```

#### AIEOS 模式部分

| 部分 | 描述 |
|---------|-------------|
| `identity` | 姓名、简介、出身、居住地 |
| `psychology` | 神经矩阵（认知权重）、MBTI、海洋、道德指南针 |
| `linguistics` | 文本风格、形式、流行语、禁用词 |
| `motivations` | 核心动力、短期/长期目标、恐惧 |
| `capabilities` | 代理可以使用的技能和工具（tools） |
| `physicality` | 用于图像生成的视觉描述符 |
| `history` | 起源故事、教育、职业 |
| `interests` | 爱好、最爱、生活方式 |

请访问 [aieos.org](https://aieos.org) 查看完整架构和实时示例。

## Gateway API

| 端点 | 方法 | 授权 | 描述 |
|----------|--------|------|-------------|
| `/health` | 得到 | 没有任何 | 健康检查（始终公开，不泄露秘密） |
| `/pair` | 邮政 | `X-Pairing-Code`标题 | 将一次性代码兑换为不记名令牌 |
| `/webhook` | 邮政 | `Authorization: Bearer <token>` | 发送消息：`{"message": "your prompt"}` |
| `/whatsapp` | 得到 | 查询参数 | Meta webhook 验证（hub.mode、hub.verify_token、hub.challenge） |
| `/whatsapp` | 邮政 | 无（Meta 签名） | WhatsApp 传入消息 webhook |

## 命令

| 命令 | 描述 |
|---------|-------------|
| `onboard` | 快速设置（默认） |
| `onboard --interactive` | 完整的交互式 7 步向导 |
| `onboard --channels-only` | 仅重新配置通道（channels）/允许列表（allowlists）（快速修复流程） |
| `agent -m "..."` | 单消息模式 |
| `agent` | 交互式聊天模式 |
| `gateway` | 启动 webhook 服务器（默认: `127.0.0.1:8080`） |
| `gateway --port 0` | 随机端口模式 |
| `daemon` | 启动长时间运行的自治运行时 |
| `service install/start/stop/status/uninstall` | 管理用户级后台服务 |
| `doctor` | 诊断 daemon/scheduler/通道（channel）新鲜度 |
| `status` | 显示完整的系统状态 |
| `channel doctor` | 对配置的通道（channels）运行健康检查 |
| `integrations info <name>` | 显示一项集成的设置/状态详细信息 |

## 开发

```bash
cargo build              # 开发构建
cargo build --release    # 发布版本（codegen-units=1，适用于包括 Raspberry Pi 在内的所有设备）
cargo build --profile release-fast    # 更快的构建（codegen-units=8，需要 16GB+ RAM）
cargo test               # 1,017 次测试
cargo clippy             # Lint（0 warnings）
cargo fmt                # 格式

# 运行 SQLite 与 Markdown 基准测试
cargo test --test memory_comparison -- --nocapture
```

### 推送前钩子（Pre-push hook）

git hook 在每次推送之前运行 `cargo fmt --check`、`cargo clippy -- -D warnings` 和 `cargo test`。启用一次：

```bash
git config core.hooksPath .githooks
```

### 构建故障排除（Linux OpenSSL 错误）

如果您看到 `openssl-sys` 构建错误，请同步依赖项并使用存储库锁定文件进行重建：

```bash
git pull
cargo build --release --locked
cargo install --path . --force --locked
```

ZeroClaw 配置为使用 `rustls` 进行 HTTP/TLS 依赖项； `--locked` 使传递图在新环境中保持确定性。

在开发过程中需要快速推送时跳过钩子：

```bash
git push --no-verify
```

## 协作和文档

对于高通量协作和一致的评论：

- 贡献指南: [CONTRIBUTING.md](CONTRIBUTING.md)
- PR 工作流策略: [docs/pr-workflow.md](docs/pr-workflow.md)
- 审查员手册（分类 + 深度审查）: [docs/reviewer-playbook.md](docs/reviewer-playbook.md)
- CI 所有权和分类图: [docs/ci-map.md](docs/ci-map.md)
- 安全披露政策: [SECURITY.md](SECURITY.md)

## 支持

ZeroClaw 是一个充满热情维护的开源项目。如果你觉得它有用，并愿意支持其持续开发、测试硬件以及维护者的咖啡，你可以在这里支持我：

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 特别感谢

衷心感谢激发和推动这项开源工作的社区和机构：

- **哈佛大学**——培养求知欲并突破可能的界限。
- **麻省理工学院**——倡导开放知识、开源以及技术应该为每个人所用的信念。
- **Sundai Club** — 为社区、活力以及打造重要事物的不懈动力。
- **世界与彼岸** 🌍✨ — 致每一位贡献者、梦想家和建设者，让开源成为一股向善的力量。这是给您的。

我们正在公开建设，因为最好的想法来自四面八方。如果您正在阅读本文，那么您就是其中的一部分。欢迎。 🦀❤️

## 许可证

Apache 2.0 — 详见 [LICENSE](LICENSE) 与 [NOTICE](NOTICE)（贡献者署名）

## 贡献

见 [CONTRIBUTING.md](CONTRIBUTING.md)。实现一个 Trait，提交 PR：
- CI 工作流指南: [docs/ci-map.md](docs/ci-map.md)
- 新`Provider` → `src/providers/`
- 新`Channel` → `src/channels/`
- 新`Observer` → `src/observability/`
- 新`Tool` → `src/tools/`
- 新`Memory` → `src/memory/`
- 新`Tunnel` → `src/tunnel/`
- 新`Skill` → `~/.zeroclaw/workspace/skills/<name>/`


---

**ZeroClaw** — 零开销。零妥协。随处部署。万物可换。 🦀

## Star History

<p align="center">
  <a href="https://www.star-history.com/#zeroclaw-labs/zeroclaw&Date">
    <img src="https://api.star-history.com/svg?repos=zeroclaw-labs/zeroclaw&type=Date" alt="Star History Chart" />
  </a>
</p>
