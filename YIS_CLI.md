# Yis Cli

本仓库基于上游 CLI 二次发行，产品名为 **Yis Cli**。

> **本地开发 / 编译 / 发版 / 清理 target**：见 [docs/本地开发与运维.md](docs/本地开发与运维.md)

## 本地安全模式（发行默认）

**Release 构建强制本地模式**：不向 `grok.com` / `auth.x.ai` / `cli-chat-proxy` / `x.ai` 发起授权、遥测、会话同步、trace 上传或自动更新。

| 能力 | 行为 |
|------|------|
| xAI 浏览器登录 / OAuth | **禁用**（无 `yis login` 云端登录） |
| 远程设置 / 模型目录拉取 | **禁用** |
| 遥测 / Mixpanel / Sentry 上报 | **禁用** |
| 自动更新 / 最低版本检查 | **禁用** |
| 会话云端同步 / relay | **禁用** |
| 模型推理 | **仅**请求你在配置里写的厂商 `base_url` |

Debug 构建仍可用 `YIS_LOCAL_MODE=0` 临时恢复上游云路径（开发用）。`YIS_OFFLINE=1` 任意构建均强制本地。

| 环境变量 | 含义 |
|----------|------|
| （Release 默认） | 本地安全模式，不可关闭 |
| `YIS_LOCAL_MODE=0` | 仅 Debug 可关本地模式 |
| `YIS_OFFLINE=1` | 强制本地 |
| `YIS_LANG=zh` / `en` | 界面语言（默认 zh） |

## 配置模型（qoder-switch 风格）

本地模式**必须**自行配置国内/自建厂商的 API Key，否则无法对话。

### CLI

```bash
# 交互式：选厂商 → 填 Key → 选模型 → 写入 ~/.yis/config.toml
yis models setup

# 查看内置厂商预设
yis models presets

# 非交互添加
yis models add --preset deepseek --api-key sk-你的密钥
yis models add --preset deepseek --api-key sk-... --model deepseek-reasoner

# 列出已配置模型
yis models
```

### TUI

- 首次新建会话且尚未配置模型时，会**自动打开**模型管理弹窗
- 会话内：`/model-add` 或 `/models` 打开管理界面
- 切换模型：`/model` 或 `Ctrl+M`

### 手写 config（等价）

```toml
# ~/.yis/config.toml
[models]
default = "deepseek-chat"

[model.deepseek-chat]
model = "deepseek-chat"
name = "DeepSeek"
base_url = "https://api.deepseek.com/v1"
api_backend = "chat_completions"
api_key = "sk-你的密钥"
context_window = 128000
```

内置预设包括：DeepSeek、阿里云百炼、智谱、Kimi、MiniMax、小米 MIMO、OpenAI 兼容、Anthropic 兼容、自定义。

## 语言切换

```
/language          # 中英切换
/language zh
/language en
/lang
/语言
```

## 构建与运行

```bash
cd /path/to/grok-build
export PATH="$HOME/.cargo/bin:$PATH"
cargo run -p xai-yis-pager-bin
# 或
./target/debug/yis
```

## 安装 / 发布（macOS + Windows）

仓库：https://github.com/481617494/yis-Cli

### 用户安装（发布 Release 之后）

**macOS**

```bash
curl -fsSL https://github.com/481617494/yis-Cli/releases/latest/download/install.sh | bash
# 确保 ~/.local/bin 在 PATH
yis models setup
yis
```

**Windows（PowerShell）**

```powershell
irm https://github.com/481617494/yis-Cli/releases/latest/download/install.ps1 | iex
yis models setup
yis
```

### 维护者：打 tag 自动出包

```bash
# 推送代码到 yis-Cli 后
git tag v0.1.0
git push origin v0.1.0
# GitHub Actions 会构建并上传：
#   yis-darwin-arm64 / yis-darwin-x64 / yis-windows-x64.exe
#   install.sh / install.ps1 / checksums.txt
```

本机仅当前架构打包：

```bash
./scripts/build-release-local.sh
# 输出 dist/yis-darwin-arm64 等
```

## 说明

- 主二进制名为 `yis`。配置目录 `~/.yis`，环境变量 `YIS_*`。
- 用户可见品牌为 **Yis Cli** / `yis`。
- 隐私：出站请求白名单仅为你配置的模型厂商地址；不上传对话/遥测到 xAI 云。
