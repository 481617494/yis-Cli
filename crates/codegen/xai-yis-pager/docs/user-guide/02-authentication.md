# Authentication（本地 BYOK）

Yis Cli 发行版默认 **本地安全模式**：不连接 grok.com / auth.x.ai，也不要求 xAI 账号登录。

推理请求只使用你在 `~/.yis/config.toml` 里配置的厂商 `base_url` + `api_key`。

---

## 推荐：模型管理向导

### CLI

```bash
# 交互：选厂商 → API Key → 模型
yis models setup

# 一键添加
yis models add --preset deepseek --api-key sk-你的密钥

# 查看预设
yis models presets
```

### TUI

- 首次进入会话且尚未配置模型时，自动打开模型管理弹窗
- `/model-add` 或 `/models`：管理自定义模型
- `/model` 或 `Ctrl+M`：切换当前模型

配置写入 `~/.yis/config.toml` 的 `[model.*]` 与 `[models].default`。

---

## 手写配置

```toml
[models]
default = "deepseek-chat"

[model.deepseek-chat]
model = "deepseek-chat"
name = "DeepSeek"
base_url = "https://api.deepseek.com/v1"
api_backend = "chat_completions"
api_key = "sk-..."
context_window = 128000
```

也可用环境变量承载密钥：

```toml
[model.deepseek-chat]
model = "deepseek-chat"
base_url = "https://api.deepseek.com/v1"
env_key = "DEEPSEEK_API_KEY"
```

全局 `XAI_API_KEY` 仍可作为兜底密钥（可选）。

---

## 凭证解析顺序

1. `[model.*]` 的 `api_key`
2. `env_key` 指向的环境变量
3. （云端模式下的会话 token — 本地发行版不使用）
4. `XAI_API_KEY` / `YIS_CODE_XAI_API_KEY`

---

## 已禁用的云端登录

以下能力在本地安全模式下 **不可用**：

- `yis login` / 浏览器 OAuth（auth.x.ai）
- `yis logout`（无云会话可清）
- 订阅门闸、远程 settings、会话上传、遥测

详见仓库根目录 [YIS_CLI.md](../../../../../YIS_CLI.md)。

---

## 另见

- [Custom Models](11-custom-models.md)
- [Configuration](05-configuration.md)
