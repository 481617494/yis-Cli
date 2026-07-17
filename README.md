# Yis Cli (`yis`)

终端 AI 编程助手 · **本地安全模式** · 自备模型 API（BYOK）

基于上游开源 CLI 二次发行。**不登录 grok.com / xAI**，不向 xAI 云上传对话与遥测；推理只请求你配置的厂商（DeepSeek、百炼、智谱等）或本机模型。

仓库：https://github.com/481617494/yis-Cli

---

## 功能概览

| 能力 | 说明 |
|------|------|
| **终端 TUI** | 全屏对话、读改代码、执行命令、管理会话 |
| **本地安全** | Release 强制本地模式：无 xAI 登录、无遥测、无会话云同步 |
| **多厂商模型** | DeepSeek / 阿里云百炼 / 智谱 / Kimi / MiniMax / 小米 MIMO / OpenAI·Anthropic 兼容 / 自定义 |
| **模型配置** | CLI `yis models setup` + TUI `/model-add`（类 qoder-switch） |
| **斜杠命令** | `/model` `/new` `/compact` `/language` 等 |
| **无界面模式** | `yis -p "问题"` 适合脚本 |
| **中英界面** | 默认中文，`/language` 切换 |
| **安装包** | macOS + Windows，GitHub Actions 发版 |

### 不做的事（相对 xAI 云）

- 浏览器 OAuth / `yis login` 连 auth.x.ai  
- 远程 settings、官方模型目录拉取  
- Mixpanel / Sentry / 会话 trace 上报  
- 自动检查 x.ai 更新  

### 仍会出网的数据

| 数据 | 去向 |
|------|------|
| 对话 / 代码上下文 | **你配置的模型厂商** `base_url` |
| `web_fetch` / 联网工具 / MCP | 对应目标（用到时） |

完全离线：模型指向本机（如 Ollama `http://127.0.0.1:11434/v1`）。

---

## 安装

### macOS

```bash
curl -fsSL https://github.com/481617494/yis-Cli/releases/latest/download/install.sh | bash

# 如提示找不到命令：
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc
```

### Windows（PowerShell）

```powershell
irm https://github.com/481617494/yis-Cli/releases/latest/download/install.ps1 | iex
```

预编译包：https://github.com/481617494/yis-Cli/releases  

（需 Releases 页有 Assets；仅有 Tags 时请等 Actions 的 `release` 任务成功。）

---

## 快速开始

```bash
# 1. 配置模型（必做）
yis models setup
# 或一条命令：
# yis models add --preset deepseek --api-key sk-你的密钥

# 2. 进入项目并启动
cd /path/to/your/project
yis

# 3. 输入需求，回车发送
```

### 常用 CLI

| 命令 | 作用 |
|------|------|
| `yis` | 启动 TUI |
| `yis -m <模型id>` | 指定模型启动 |
| `yis -p "问题"` | 无界面单次提问 |
| `yis models` | 列出已配置模型 |
| `yis models setup` | 交互配置厂商 + API Key |
| `yis models presets` | 内置厂商列表 |
| `yis models add --preset deepseek --api-key sk-...` | 非交互添加 |
| `yis --version` / `yis --help` | 版本 / 帮助 |

### TUI 内常用

| 操作 | 说明 |
|------|------|
| 输入后 Enter | 发送 |
| `/model` 或 `Ctrl+M` | 切换模型 |
| `/model-add` | 管理厂商 / Key |
| `/new` | 新会话 |
| `/compact` | 压缩上下文 |
| `/language` | 中英切换 |
| `/help` | 帮助 |
| `/quit` | 退出 |

配置文件：`~/.yis/config.toml`（勿把含 Key 的配置提交到公开仓库）

---

## 文档

| 文档 | 内容 |
|------|------|
| **[docs/使用手册.md](docs/使用手册.md)** | 安装、配置、对话、快捷键、FAQ |
| **[docs/本地开发与运维.md](docs/本地开发与运维.md)** | 编译、改代码、发版、清理 `target/` |
| **[YIS_CLI.md](YIS_CLI.md)** | 本地安全模式与隐私边界 |
| [user-guide/](crates/codegen/xai-yis-pager/docs/user-guide/) | 更细的上游能力文档（部分仍带上游表述） |

---

## 从源码构建

依赖：

- **Rust**（见 `rust-toolchain.toml`，`rustup` 会自动安装）  
- **protoc**（`PATH` 上的 `protoc`，或设置 `PROTOC`；仓库 `bin/protoc` 需 [dotslash](https://dotslash-cli.com)）

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cd /path/to/yis-Cli

cargo run -p xai-yis-pager-bin                 # 开发运行
cargo build -p xai-yis-pager-bin --release     # 产物: target/release/yis
cargo check -p xai-yis-pager-bin               # 快速检查

# 磁盘占用大时可清理编译缓存（不影响 ~/.yis 配置）
cargo clean
```

---

## 发版（维护者）

```bash
git push yis main
git tag v0.1.x
git push yis v0.1.x
```

GitHub Actions（`.github/workflows/release.yml`）构建：

- `yis-darwin-arm64` / `yis-darwin-x64`  
- `yis-windows-x64.exe`  
- `install.sh` / `install.ps1`  

---

## 仓库结构（简）

| 路径 | 内容 |
|------|------|
| `crates/codegen/xai-yis-pager-bin` | 主二进制 `yis` |
| `crates/codegen/xai-yis-pager` | TUI |
| `crates/codegen/xai-yis-shell` | Agent 运行时、模型配置、本地模式闸 |
| `crates/codegen/xai-yis-env` | `is_local_mode()` 真源 |
| `crates/codegen/xai-yis-tools` | 工具实现 |
| `scripts/` | 安装脚本、本机打 release |
| `docs/` | 中文使用与运维文档 |
| `qoder-switch/` | 模型切换交互参考 |

---

## 许可

见 [LICENSE](LICENSE)。本仓库为二次发行，上游组件遵循其各自许可。
