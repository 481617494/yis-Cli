<div align="center">

<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://media.x.ai/v1/website/spacexai-symbol-white-transparent-0c31957f.png">
    <source media="(prefers-color-scheme: light)" srcset="https://media.x.ai/v1/website/spacexai-symbol-black-transparent-6435cf42.png">
    <img alt="SpaceXAI logo" src="https://media.x.ai/v1/website/spacexai-symbol-black-transparent-6435cf42.png" width="96">
  </picture>
  <br>
  Yis Cli (<code>yis</code>)
</h1>

**Yis Cli** is SpaceXAI's terminal-based AI coding agent. It runs as a
full-screen TUI that understands your codebase, edits files, executes shell
commands, searches the web, and manages long-running tasks — interactively,
headlessly for scripting/CI, or embedded in editors via the Agent Client
Protocol (ACP).

[Installing the released binary](#installing-the-released-binary) ·
[Building from source](#building-from-source) ·
[Documentation](#documentation) ·
[Repository layout](#repository-layout) ·
[Development](#development) ·
[Contributing](#contributing) ·
[License](#license)

![Yis Cli](https://media.x.ai/v1/website/universe-tui-screenshot-6f7a0837.png)

**Learn more about Yis Cli at [x.ai/cli](https://x.ai/cli)**

This repository contains the Rust source for the `yis` CLI/TUI and its agent
runtime. It is synced periodically from the SpaceXAI monorepo.

</div>

---

## Installing the released binary

**Yis Cli（本仓库二次发行）** 预编译包发布在
[GitHub Releases](https://github.com/481617494/yis-Cli/releases)
（macOS + Windows）：

```sh
# macOS
curl -fsSL https://github.com/481617494/yis-Cli/releases/latest/download/install.sh | bash

# Windows PowerShell
irm https://github.com/481617494/yis-Cli/releases/latest/download/install.ps1 | iex

yis models setup
yis --version
```

推送 tag（如 `v0.1.0`）后，GitHub Actions 会自动构建并上传资产。详见 [YIS_CLI.md](YIS_CLI.md)。

> 上游 xAI 官方安装（连云端登录，与本本地版不同）：
> `curl -fsSL https://x.ai/cli/install.sh | bash`

## 本地开发与运维

完整命令清单（编译、模型配置、发版、清理 `target/`）：  
→ **[docs/本地开发与运维.md](docs/本地开发与运维.md)**

## Building from source

Requirements:

- **Rust** — the toolchain is pinned by [`rust-toolchain.toml`](rust-toolchain.toml);
  `rustup` installs it automatically on first build.
- **protoc** — proto codegen resolves [`bin/protoc`](bin/protoc) (a
  [dotslash](https://dotslash-cli.com) launcher) or falls back to a `protoc` on
  `PATH` / `$PROTOC`.
- macOS and Linux are supported build hosts; Windows builds are best-effort
  and not currently tested from this tree.

```sh
cargo run -p xai-yis-pager-bin              # build + launch the TUI
cargo build -p xai-yis-pager-bin --release  # release binary: target/release/xai-yis-pager
cargo check -p xai-yis-pager-bin            # fast validation
```

The binary artifact is named `yis` (package `xai-yis-pager-bin`). **Yis 发行版默认本地安全模式**：不打开浏览器登录 grok.com。首次使用请配置模型：

```sh
yis models setup                          # 交互：选厂商 + API Key
yis models add --preset deepseek --api-key sk-...
```

TUI 内也可用 `/model-add`。详见 [YIS_CLI.md](YIS_CLI.md) 与
[authentication guide](crates/codegen/xai-yis-pager/docs/user-guide/02-authentication.md)。

## Documentation

Full online documentation is available at
[docs.x.ai/build/overview](https://docs.x.ai/build/overview).

The user guide ships with the pager crate:
[`crates/codegen/xai-yis-pager/docs/user-guide/`](crates/codegen/xai-yis-pager/docs/user-guide/)
— getting started, keyboard shortcuts, slash commands, configuration, theming,
MCP servers, skills, plugins, hooks, headless mode, sandboxing, and more.

## Repository layout

| Path | Contents |
|------|----------|
| `crates/codegen/xai-yis-pager-bin` | Composition-root package; builds the `xai-yis-pager` binary |
| `crates/codegen/xai-yis-pager` | The TUI: scrollback, prompt, modals, rendering |
| `crates/codegen/xai-yis-shell` | Agent runtime + leader/stdio/headless entry points |
| `crates/codegen/xai-yis-tools` | Tool implementations (terminal, file edit, search, ...) |
| `crates/codegen/xai-yis-workspace` | Host filesystem, VCS, execution, checkpoints |
| `crates/codegen/...` | The rest of the CLI crate closure (config, MCP, markdown, sandbox, ...) |
| `crates/common/`, `crates/build/`, `prod/mc/` | Small shared leaf crates pulled in by the closure |
| `third_party/` | Vendored upstream source (Mermaid diagram stack) — see below |

> [!IMPORTANT]
> The root `Cargo.toml` (workspace members, dependency versions, lints,
> profiles) is **generated** — treat it as read-only. Prefer editing per-crate
> `Cargo.toml` files.

## Development

```sh
cargo check -p <crate>        # always target specific crates; full-workspace builds are slow
cargo test -p xai-yis-config # per-crate tests
cargo clippy -p <crate>       # lint config: clippy.toml at the repo root
cargo fmt --all               # rustfmt.toml at the repo root
```

## Contributing

> [!NOTE]
> External contributions are not accepted. See [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

First-party code in this repository is licensed under the **Apache License,
Version 2.0** — see [`LICENSE`](LICENSE).

Third-party and vendored code remains under its original licenses. See:

- [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES) — crates.io / git dependencies,
  bundled UI themes, and **in-tree source ports** (including openai/codex and
  sst/opencode tool implementations)
- [`crates/codegen/xai-yis-tools/THIRD_PARTY_NOTICES.md`](crates/codegen/xai-yis-tools/THIRD_PARTY_NOTICES.md)
  — crate-local notice for the codex and opencode ports (license texts +
  Apache §4(b) change notice)
- [`third_party/NOTICE`](third_party/NOTICE) — vendored Mermaid-stack index
