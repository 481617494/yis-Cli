# Qoder Switch（终端交互）

纯终端交互的多 CLI 模型切换工具。

```
你想要修改什么 CLI？
  ← → 切换   Enter 确认

↓ 模型列表

  * DeepSeek   deepseek/deepseek-chat   sk-****
  > + 新增模型

  ↑↓ 移动   Enter 选用/新增   e 编辑   d 删除   Esc 返回
```

## 使用

```bash
chmod +x ~/Desktop/qoder-switch/qoder-switch
ln -sfn ~/Desktop/qoder-switch/qoder-switch ~/.local/bin/qoder-switch

qoder-switch
```

## 键位

| 键 | 作用 |
|----|------|
| `←` `→` | 选择 CLI；新增时选择厂商 |
| `↑` `↓` | 列表移动 |
| `Enter` | 选用模型 / 进入新增 / 确认 |
| `e` | **编辑**当前模型（名称 / API Key / 模型） |
| `d` | 删除当前模型 |
| `Esc` / `q` | 返回 / 退出 |

### 选模型（新增 / 编辑时）

| 键 | 作用 |
|----|------|
| `↑` `↓` | 在拉取到的模型列表中移动 |
| `Enter` | 确认选中 |
| `m` | 手动输入模型 ID |
| `r` | 重新从接口拉取 |
| `Esc` | 取消 |

模型列表优先请求厂商 OpenAI 兼容接口 `GET …/models`；失败则用内置常用列表。

## 数据

- `~/.qoder-switch/store.json` — 自定义模型与当前选用
- 启用 Qoder 时写入 `~/.qoder/settings.json`（BYOK）
- 备份：`~/.qoder-switch/backups/`

启用或编辑当前使用中的模型后，请**重启对应 CLI**。
