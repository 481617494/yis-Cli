//! Yis 模型管理弹窗 — 交互对齐 qoder-switch。
//!
//! ```text
//! 自定义模型
//!   * DeepSeek   deepseek/deepseek-chat   sk-****
//!   > + 新增模型
//!   ↑↓ 移动  Enter 选用/新增  e 编辑  d 删除  Esc 关闭
//! ```
//! 新增：选厂商 → 名称 → API Key → (可选 Base URL) → 拉取模型列表 → 选择

use crate::config_toml_edit::read_config_document_for_edit;
use crate::i18n;
use crate::theme::Theme;
use crate::views::modal_window::{
    self, ModalSizing, ModalWindowConfig, ModalWindowState, Shortcut,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{SystemTime, UNIX_EPOCH};

// ── Data ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedModel {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub model: String,
    pub api_key: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default = "default_format")]
    pub format: String,
    /// config.toml 中的 [model.<slug>]
    pub slug: String,
}

fn default_format() -> String {
    "openai".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoreFile {
    version: u32,
    models: Vec<ManagedModel>,
    #[serde(default)]
    current: Option<String>,
}

impl Default for StoreFile {
    fn default() -> Self {
        Self {
            version: 1,
            models: vec![],
            current: None,
        }
    }
}

#[derive(Clone, Copy)]
struct Vendor {
    id: &'static str,
    label: &'static str,
    model: &'static str,
    base_url: &'static str,
    /// need user to enter base url
    custom_url: bool,
    fallback_models: &'static [&'static str],
}

// Keep in sync with `xai_yis_shell::model_presets::VENDOR_PRESETS`.
const VENDORS: &[Vendor] = &[
    Vendor {
        id: "deepseek",
        label: "DeepSeek",
        model: "deepseek-chat",
        base_url: "https://api.deepseek.com/v1",
        custom_url: false,
        fallback_models: &[
            "deepseek-chat",
            "deepseek-reasoner",
            "deepseek-v4-pro",
            "deepseek-v4-flash",
        ],
    },
    Vendor {
        id: "bailian",
        label: "阿里云百炼",
        model: "qwen3.5-plus-cp",
        base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1",
        custom_url: false,
        fallback_models: &[
            "qwen3.5-plus-cp",
            "qwen3-coder-plus",
            "qwen-max",
            "qwen-plus",
            "qwen-turbo",
        ],
    },
    Vendor {
        id: "zhipu",
        label: "智谱 GLM",
        model: "glm-4.6",
        base_url: "https://open.bigmodel.cn/api/paas/v4",
        custom_url: false,
        fallback_models: &["glm-4.6", "glm-4.5", "glm-5.2", "glm-4-flash", "glm-4-plus"],
    },
    Vendor {
        id: "kimi",
        label: "Kimi",
        model: "kimi-k2.5",
        base_url: "https://api.moonshot.cn/v1",
        custom_url: false,
        fallback_models: &[
            "kimi-k2.5",
            "kimi-k2.7-code",
            "moonshot-v1-128k",
            "moonshot-v1-32k",
            "moonshot-v1-8k",
        ],
    },
    Vendor {
        id: "minimax",
        label: "MiniMax",
        model: "MiniMax-M2.5",
        base_url: "https://api.minimaxi.com/v1",
        custom_url: false,
        fallback_models: &["MiniMax-M2.5", "MiniMax-M3", "MiniMax-Text-01"],
    },
    Vendor {
        id: "mimo",
        label: "小米 MIMO",
        model: "mimo-v2-pro",
        base_url: "https://api.xiaomimimo.com/v1",
        custom_url: false,
        fallback_models: &["mimo-v2-pro", "mimo-v2-flash"],
    },
    Vendor {
        id: "openai",
        label: "OpenAI 兼容",
        model: "gpt-4o",
        base_url: "https://api.openai.com/v1",
        custom_url: true,
        fallback_models: &["gpt-4o", "gpt-4o-mini", "gpt-4.1", "o3", "o4-mini"],
    },
    Vendor {
        id: "anthropic",
        label: "Anthropic 兼容",
        model: "claude-sonnet-4-5",
        base_url: "https://api.anthropic.com",
        custom_url: true,
        fallback_models: &[
            "claude-sonnet-4-5",
            "claude-opus-4",
            "claude-haiku-4-5",
        ],
    },
    Vendor {
        id: "custom",
        label: "自定义",
        model: "custom-model",
        base_url: "",
        custom_url: true,
        fallback_models: &[],
    },
];

// ── Phases ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Phase {
    /// 模型列表 + 新增
    List { cursor: usize, status: String },
    /// 选厂商
    Vendor { idx: usize },
    /// 输入显示名称（`cursor` = 字节偏移，落在 char boundary）
    EnterName {
        vendor: usize,
        buf: String,
        cursor: usize,
        editing_id: Option<String>,
    },
    /// 输入 API Key
    EnterKey {
        vendor: usize,
        name: String,
        buf: String,
        cursor: usize,
        editing_id: Option<String>,
        keep_old_key: Option<String>,
    },
    /// 输入 Base URL（自定义厂商）
    EnterBaseUrl {
        vendor: usize,
        name: String,
        api_key: String,
        buf: String,
        cursor: usize,
        editing_id: Option<String>,
    },
    /// 正在拉取 / 选择模型
    PickModel {
        vendor: usize,
        name: String,
        api_key: String,
        base_url: String,
        editing_id: Option<String>,
        models: Vec<String>,
        cursor: usize,
        note: String,
        fetching: bool,
    },
}

// ── State ─────────────────────────────────────────────────────────────

pub struct ModelManagerState {
    phase: Phase,
    store: StoreFile,
    window: ModalWindowState,
    /// 异步拉模型结果
    fetch_rx: Option<Receiver<(Vec<String>, String)>>,
}

pub enum ModelManagerOutcome {
    Close,
    Changed,
    Unchanged,
    /// 已写入 config，提示用户重启后切换（slug）
    Applied { slug: String, name: String },
}

impl ModelManagerState {
    pub fn open() -> Self {
        let store = load_store();
        let cursor = store
            .current
            .as_ref()
            .and_then(|id| store.models.iter().position(|m| &m.id == id))
            .unwrap_or(0);
        Self {
            phase: Phase::List {
                cursor,
                status: String::new(),
            },
            store,
            window: ModalWindowState::new(),
            fetch_rx: None,
        }
    }

    /// 轮询异步拉取
    pub fn poll_fetch(&mut self) -> bool {
        let Some(rx) = &self.fetch_rx else {
            return false;
        };
        match rx.try_recv() {
            Ok((models, note)) => {
                self.fetch_rx = None;
                if let Phase::PickModel {
                    vendor,
                    name,
                    api_key,
                    base_url,
                    editing_id,
                    ..
                } = &self.phase
                {
                    let mut models = models;
                    let note = note;
                    if models.is_empty() {
                        let fb = VENDORS
                            .get(*vendor)
                            .map(|v| v.fallback_models.to_vec())
                            .unwrap_or_default();
                        models = fb.into_iter().map(|s| s.to_string()).collect();
                    }
                    self.phase = Phase::PickModel {
                        vendor: *vendor,
                        name: name.clone(),
                        api_key: api_key.clone(),
                        base_url: base_url.clone(),
                        editing_id: editing_id.clone(),
                        models,
                        cursor: 0,
                        note,
                        fetching: false,
                    };
                    return true;
                }
                false
            }
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                self.fetch_rx = None;
                false
            }
        }
    }
}

// ── Store / config ────────────────────────────────────────────────────

fn store_path() -> PathBuf {
    xai_yis_tools::util::yis_home::yis_home().join("yis-models.json")
}

fn load_store() -> StoreFile {
    let path = store_path();
    if let Ok(raw) = std::fs::read_to_string(&path)
        && let Ok(s) = serde_json::from_str::<StoreFile>(&raw)
    {
        return s;
    }
    StoreFile::default()
}

fn save_store(store: &StoreFile) {
    let path = store_path();
    if let Some(p) = path.parent() {
        let _ = std::fs::create_dir_all(p);
    }
    if let Ok(raw) = serde_json::to_string_pretty(store) {
        let _ = std::fs::write(&path, raw + "\n");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
    }
}

fn mask_key(k: &str) -> String {
    if k.is_empty() {
        return if i18n::is_zh() {
            "未配置".into()
        } else {
            "unset".into()
        };
    }
    if k.len() <= 8 {
        return "****".into();
    }
    format!("{}****{}", &k[..3], &k[k.len() - 4..])
}

fn sanitize_slug(raw: &str) -> String {
    let s: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    let s = s.trim_matches(|c| c == '-' || c == '.' || c == '_').to_string();
    if s.is_empty() {
        format!("m-{}", now_id())
    } else if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("m-{s}")
    } else {
        s
    }
}

fn now_id() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| format!("{:x}", d.as_millis()))
        .unwrap_or_else(|_| "x".into())
}

fn write_config_model(m: &ManagedModel) -> Result<(), String> {
    let path = xai_yis_tools::util::yis_home::yis_home().join("config.toml");
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
    }
    let mut doc =
        read_config_document_for_edit(&path).ok_or_else(|| "config.toml 无法解析".to_string())?;

    if doc.get("models").and_then(|v| v.as_table()).is_none() {
        doc["models"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    doc["models"]["default"] = toml_edit::value(&m.slug);

    if doc.get("features").and_then(|v| v.as_table()).is_none() {
        doc["features"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    doc["features"]["remote_fetch"] = toml_edit::value(false);

    let mut mt = toml_edit::Table::new();
    mt["model"] = toml_edit::value(&m.model);
    mt["name"] = toml_edit::value(&m.name);
    if !m.base_url.is_empty() {
        mt["base_url"] = toml_edit::value(&m.base_url);
    }
    mt["api_key"] = toml_edit::value(&m.api_key);
    mt["api_backend"] = toml_edit::value(if m.format == "anthropic" {
        "messages"
    } else {
        "chat_completions"
    });
    mt["context_window"] = toml_edit::value(128000i64);
    if m.format == "anthropic" {
        let mut headers = toml_edit::InlineTable::new();
        headers.insert("x-api-key", m.api_key.as_str().into());
        headers.insert("anthropic-version", "2023-06-01".into());
        mt["extra_headers"] = toml_edit::value(headers);
    }

    if doc.get("model").and_then(|v| v.as_table()).is_none() {
        doc["model"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    if let Some(model_table) = doc["model"].as_table_mut() {
        model_table.set_implicit(true);
        model_table[&m.slug] = toml_edit::Item::Table(mt);
    }

    std::fs::write(&path, doc.to_string()).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

fn remove_config_model_slug(slug: &str) {
    let path = xai_yis_tools::util::yis_home::yis_home().join("config.toml");
    let Some(mut doc) = read_config_document_for_edit(&path) else {
        return;
    };
    if let Some(model_table) = doc.get_mut("model").and_then(|v| v.as_table_mut()) {
        model_table.remove(slug);
    }
    let _ = std::fs::write(&path, doc.to_string());
}

// ── Fetch models via curl ─────────────────────────────────────────────

fn start_fetch(base_url: String, api_key: String, vendor_id: String) -> Receiver<(Vec<String>, String)> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let result = fetch_models_curl(&base_url, &api_key, &vendor_id);
        let _ = tx.send(result);
    });
    rx
}

fn fetch_models_curl(base_url: &str, api_key: &str, vendor_id: &str) -> (Vec<String>, String) {
    let base = base_url.trim().trim_end_matches('/');
    let urls = if base.is_empty() {
        vec![]
    } else if base.ends_with("/models") {
        vec![base.to_string()]
    } else {
        let mut u = vec![format!("{base}/models")];
        if !base.ends_with("/v1") && !base.ends_with("/v4") {
            u.push(format!("{base}/v1/models"));
        }
        u
    };

    for url in &urls {
        let out = std::process::Command::new("curl")
            .args([
                "-fsSL",
                "-m",
                "12",
                "-H",
                &format!("Authorization: Bearer {api_key}"),
                "-H",
                "Content-Type: application/json",
                url,
            ])
            .output();
        let Ok(o) = out else { continue };
        if !o.status.success() {
            // try x-api-key for anthropic-ish
            let out2 = std::process::Command::new("curl")
                .args([
                    "-fsSL",
                    "-m",
                    "12",
                    "-H",
                    &format!("x-api-key: {api_key}"),
                    "-H",
                    "anthropic-version: 2023-06-01",
                    url,
                ])
                .output();
            if let Ok(o2) = out2
                && o2.status.success()
                && let Ok(ids) = parse_models_json(&String::from_utf8_lossy(&o2.stdout))
                && !ids.is_empty()
            {
                return (ids, format!("接口: {url}"));
            }
            continue;
        }
        if let Ok(ids) = parse_models_json(&String::from_utf8_lossy(&o.stdout))
            && !ids.is_empty()
        {
            return (ids, format!("接口: {url}"));
        }
    }

    let fb: Vec<String> = VENDORS
        .iter()
        .find(|v| v.id == vendor_id)
        .map(|v| v.fallback_models.iter().map(|s| (*s).to_string()).collect())
        .unwrap_or_default();
    if fb.is_empty() {
        (
            vec![],
            if i18n::is_zh() {
                "远程失败，无内置列表，请按 m 手动输入".into()
            } else {
                "Remote failed; press m to type model id".into()
            },
        )
    } else {
        (
            fb,
            if i18n::is_zh() {
                "远程失败，使用内置列表".into()
            } else {
                "Remote failed; using built-in list".into()
            },
        )
    }
}

fn parse_models_json(raw: &str) -> Result<Vec<String>, ()> {
    let v: serde_json::Value = serde_json::from_str(raw).map_err(|_| ())?;
    let mut ids = Vec::new();
    let items = if let Some(arr) = v.get("data").and_then(|x| x.as_array()) {
        arr.clone()
    } else if let Some(arr) = v.get("models").and_then(|x| x.as_array()) {
        arr.clone()
    } else if let Some(arr) = v.as_array() {
        arr.clone()
    } else {
        return Err(());
    };
    for it in items {
        if let Some(s) = it.as_str() {
            ids.push(s.to_string());
        } else if let Some(id) = it
            .get("id")
            .or_else(|| it.get("name"))
            .or_else(|| it.get("model"))
            .and_then(|x| x.as_str())
        {
            ids.push(id.to_string());
        }
    }
    // dedup
    let mut out = Vec::new();
    for id in ids {
        if !out.contains(&id) {
            out.push(id);
        }
    }
    Ok(out)
}

// ── Input ─────────────────────────────────────────────────────────────

pub fn handle_key(state: &mut ModelManagerState, key: &KeyEvent) -> ModelManagerOutcome {
    // Always allow Esc to go back / close
    if matches!(key.code, KeyCode::Esc)
        && !matches!(key.modifiers, KeyModifiers::CONTROL | KeyModifiers::ALT)
    {
        return handle_esc(state);
    }

    // Poll in-flight fetch
    let _ = state.poll_fetch();

    // Clone phase snapshot to avoid borrow conflicts while mutating `state`.
    let phase = state.phase.clone();
    match phase {
        Phase::List { cursor, status } => handle_list(state, key, cursor, status),
        Phase::Vendor { idx } => handle_vendor(state, key, idx),
        Phase::EnterName {
            vendor,
            buf,
            cursor,
            editing_id,
        } => handle_text(
            state,
            key,
            buf,
            cursor,
            TextPhase::Name {
                vendor,
                editing_id,
            },
        ),
        Phase::EnterKey {
            vendor,
            name,
            buf,
            cursor,
            editing_id,
            keep_old_key,
        } => handle_text(
            state,
            key,
            buf,
            cursor,
            TextPhase::Key {
                vendor,
                name,
                editing_id,
                keep_old_key,
            },
        ),
        Phase::EnterBaseUrl {
            vendor,
            name,
            api_key,
            buf,
            cursor,
            editing_id,
        } => handle_text(
            state,
            key,
            buf,
            cursor,
            TextPhase::BaseUrl {
                vendor,
                name,
                api_key,
                editing_id,
            },
        ),
        Phase::PickModel {
            vendor,
            name,
            api_key,
            base_url,
            editing_id,
            models,
            cursor,
            note,
            fetching,
        } => handle_pick(
            state,
            key,
            vendor,
            name,
            api_key,
            base_url,
            editing_id,
            models,
            cursor,
            note,
            fetching,
        ),
    }
}

fn handle_esc(state: &mut ModelManagerState) -> ModelManagerOutcome {
    match &state.phase {
        Phase::List { .. } => ModelManagerOutcome::Close,
        Phase::Vendor { .. }
        | Phase::EnterName { .. }
        | Phase::EnterKey { .. }
        | Phase::EnterBaseUrl { .. }
        | Phase::PickModel { .. } => {
            state.fetch_rx = None;
            state.phase = Phase::List {
                cursor: 0,
                status: if i18n::is_zh() {
                    "已取消".into()
                } else {
                    "Cancelled".into()
                },
            };
            ModelManagerOutcome::Changed
        }
    }
}

fn handle_list(
    state: &mut ModelManagerState,
    key: &KeyEvent,
    cursor: usize,
    _status: String,
) -> ModelManagerOutcome {
    let n = state.store.models.len() + 1; // + add row
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let c = if cursor == 0 { n - 1 } else { cursor - 1 };
            state.phase = Phase::List {
                cursor: c,
                status: String::new(),
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let c = (cursor + 1) % n;
            state.phase = Phase::List {
                cursor: c,
                status: String::new(),
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if cursor >= state.store.models.len() {
                state.phase = Phase::List {
                    cursor,
                    status: if i18n::is_zh() {
                        "「新增模型」不可删除".into()
                    } else {
                        "Cannot delete the add row".into()
                    },
                };
                return ModelManagerOutcome::Changed;
            }
            let target = state.store.models[cursor].clone();
            if state.store.current.as_deref() == Some(&target.id) {
                state.phase = Phase::List {
                    cursor,
                    status: if i18n::is_zh() {
                        "不能删除正在使用的模型，请先切换".into()
                    } else {
                        "Cannot delete the active model".into()
                    },
                };
                return ModelManagerOutcome::Changed;
            }
            remove_config_model_slug(&target.slug);
            state.store.models.retain(|m| m.id != target.id);
            save_store(&state.store);
            let new_c = cursor.min(state.store.models.len());
            state.phase = Phase::List {
                cursor: new_c,
                status: if i18n::is_zh() {
                    format!("已删除：{}", target.name)
                } else {
                    format!("Deleted: {}", target.name)
                },
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            if cursor >= state.store.models.len() {
                state.phase = Phase::List {
                    cursor,
                    status: if i18n::is_zh() {
                        "请先选中已有模型再编辑".into()
                    } else {
                        "Select a model to edit".into()
                    },
                };
                return ModelManagerOutcome::Changed;
            }
            let m = &state.store.models[cursor];
            let vendor = VENDORS
                .iter()
                .position(|v| v.id == m.provider)
                .unwrap_or(VENDORS.len() - 1);
            state.phase = Phase::EnterName {
                vendor,
                buf: m.name.clone(),
                cursor: m.name.len(),
                editing_id: Some(m.id.clone()),
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Enter => {
            if cursor >= state.store.models.len() {
                // add
                state.phase = Phase::Vendor { idx: 0 };
                return ModelManagerOutcome::Changed;
            }
            let m = state.store.models[cursor].clone();
            if m.api_key.is_empty() {
                state.phase = Phase::List {
                    cursor,
                    status: if i18n::is_zh() {
                        "该模型没有 API Key".into()
                    } else {
                        "Missing API key".into()
                    },
                };
                return ModelManagerOutcome::Changed;
            }
            match write_config_model(&m) {
                Ok(()) => {
                    state.store.current = Some(m.id.clone());
                    save_store(&state.store);
                    state.phase = Phase::List {
                        cursor,
                        status: if i18n::is_zh() {
                            format!("已启用 {} · 请重启 Yis Cli 后生效", m.name)
                        } else {
                            format!("Enabled {} · restart Yis Cli", m.name)
                        },
                    };
                    ModelManagerOutcome::Applied {
                        slug: m.slug,
                        name: m.name,
                    }
                }
                Err(e) => {
                    state.phase = Phase::List {
                        cursor,
                        status: format!("{}: {e}", if i18n::is_zh() { "失败" } else { "Error" }),
                    };
                    ModelManagerOutcome::Changed
                }
            }
        }
        KeyCode::Char('q') => ModelManagerOutcome::Close,
        _ => ModelManagerOutcome::Unchanged,
    }
}

fn handle_vendor(state: &mut ModelManagerState, key: &KeyEvent, idx: usize) -> ModelManagerOutcome {
    let n = VENDORS.len();
    match key.code {
        KeyCode::Left | KeyCode::Up => {
            state.phase = Phase::Vendor {
                idx: if idx == 0 { n - 1 } else { idx - 1 },
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Right | KeyCode::Down => {
            state.phase = Phase::Vendor {
                idx: (idx + 1) % n,
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Enter => {
            let v = &VENDORS[idx];
            let label = v.label.to_string();
            let len = label.len();
            state.phase = Phase::EnterName {
                vendor: idx,
                buf: label,
                cursor: len,
                editing_id: None,
            };
            ModelManagerOutcome::Changed
        }
        _ => ModelManagerOutcome::Unchanged,
    }
}

enum TextPhase {
    Name {
        vendor: usize,
        editing_id: Option<String>,
    },
    Key {
        vendor: usize,
        name: String,
        editing_id: Option<String>,
        keep_old_key: Option<String>,
    },
    BaseUrl {
        vendor: usize,
        name: String,
        api_key: String,
        editing_id: Option<String>,
    },
}

fn clamp_cursor(buf: &str, cursor: usize) -> usize {
    if cursor >= buf.len() {
        return buf.len();
    }
    if buf.is_char_boundary(cursor) {
        cursor
    } else {
        (0..=cursor)
            .rev()
            .find(|&i| buf.is_char_boundary(i))
            .unwrap_or(0)
    }
}

fn prev_boundary(buf: &str, cursor: usize) -> usize {
    if cursor == 0 {
        return 0;
    }
    (0..cursor)
        .rev()
        .find(|&i| buf.is_char_boundary(i))
        .unwrap_or(0)
}

fn next_boundary(buf: &str, cursor: usize) -> usize {
    if cursor >= buf.len() {
        return buf.len();
    }
    (cursor + 1..=buf.len())
        .find(|&i| buf.is_char_boundary(i))
        .unwrap_or(buf.len())
}

/// 粘贴文本到当前输入框（名称 / Key / Base URL）。
pub fn handle_paste(state: &mut ModelManagerState, text: &str) -> ModelManagerOutcome {
    // 去掉粘贴里的换行，避免密钥被截断
    let cleaned: String = text
        .chars()
        .filter(|c| *c != '\n' && *c != '\r')
        .collect();
    if cleaned.is_empty() {
        return ModelManagerOutcome::Unchanged;
    }
    let phase = state.phase.clone();
    match phase {
        Phase::EnterName {
            vendor,
            mut buf,
            cursor,
            editing_id,
        } => {
            let cur = clamp_cursor(&buf, cursor);
            buf.insert_str(cur, &cleaned);
            let new_cur = cur + cleaned.len();
            state.phase = Phase::EnterName {
                vendor,
                buf,
                cursor: new_cur,
                editing_id,
            };
            ModelManagerOutcome::Changed
        }
        Phase::EnterKey {
            vendor,
            name,
            mut buf,
            cursor,
            editing_id,
            keep_old_key,
        } => {
            let cur = clamp_cursor(&buf, cursor);
            buf.insert_str(cur, &cleaned);
            let new_cur = cur + cleaned.len();
            state.phase = Phase::EnterKey {
                vendor,
                name,
                buf,
                cursor: new_cur,
                editing_id,
                keep_old_key,
            };
            ModelManagerOutcome::Changed
        }
        Phase::EnterBaseUrl {
            vendor,
            name,
            api_key,
            mut buf,
            cursor,
            editing_id,
        } => {
            let cur = clamp_cursor(&buf, cursor);
            buf.insert_str(cur, &cleaned);
            let new_cur = cur + cleaned.len();
            state.phase = Phase::EnterBaseUrl {
                vendor,
                name,
                api_key,
                buf,
                cursor: new_cur,
                editing_id,
            };
            ModelManagerOutcome::Changed
        }
        _ => ModelManagerOutcome::Unchanged,
    }
}

fn handle_text(
    state: &mut ModelManagerState,
    key: &KeyEvent,
    mut buf: String,
    mut cursor: usize,
    phase: TextPhase,
) -> ModelManagerOutcome {
    cursor = clamp_cursor(&buf, cursor);
    match key.code {
        KeyCode::Left => {
            let prev = prev_boundary(&buf, cursor);
            rewrite_text_phase(state, buf, prev, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Right => {
            let next = next_boundary(&buf, cursor);
            rewrite_text_phase(state, buf, next, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Home => {
            rewrite_text_phase(state, buf, 0, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::End => {
            let end = buf.len();
            rewrite_text_phase(state, buf, end, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Backspace => {
            if cursor == 0 {
                return ModelManagerOutcome::Unchanged;
            }
            let prev = prev_boundary(&buf, cursor);
            buf.replace_range(prev..cursor, "");
            rewrite_text_phase(state, buf, prev, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Delete => {
            if cursor >= buf.len() {
                return ModelManagerOutcome::Unchanged;
            }
            let next = next_boundary(&buf, cursor);
            buf.replace_range(cursor..next, "");
            rewrite_text_phase(state, buf, cursor, phase);
            ModelManagerOutcome::Changed
        }
        // Ctrl+A → 全选效果：光标到末尾（全选删除用 Ctrl+U 清行）
        KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let end = buf.len();
            rewrite_text_phase(state, buf, end, phase);
            ModelManagerOutcome::Changed
        }
        // Ctrl+U → 清空整行
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            rewrite_text_phase(state, String::new(), 0, phase);
            ModelManagerOutcome::Changed
        }
        // Ctrl+K → 删除光标到行尾
        KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            buf.truncate(cursor);
            rewrite_text_phase(state, buf, cursor, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Char(c)
            if key.modifiers.is_empty()
                || key.modifiers == KeyModifiers::SHIFT
                || crate::input::key::is_text_input_key(key) =>
        {
            if c.is_control() {
                return ModelManagerOutcome::Unchanged;
            }
            buf.insert(cursor, c);
            let new_cur = cursor + c.len_utf8();
            rewrite_text_phase(state, buf, new_cur, phase);
            ModelManagerOutcome::Changed
        }
        KeyCode::Enter => match phase {
            TextPhase::Name { vendor, editing_id } => {
                // 手动输入模型 ID（从选模型页按 m）
                if let Some(raw) = editing_id.as_deref().filter(|s| s.starts_with("__mid__")) {
                    let rest = &raw["__mid__".len()..];
                    let parts: Vec<&str> = rest.split('\x1f').collect();
                    if parts.len() >= 4 {
                        let vendor: usize = parts[0].parse().unwrap_or(vendor);
                        let name = parts[1].to_string();
                        let api_key = parts[2].to_string();
                        let base_url = parts[3].to_string();
                        let mid = buf.trim().to_string();
                        if mid.is_empty() {
                            return ModelManagerOutcome::Unchanged;
                        }
                        return finish_save(state, vendor, name, api_key, base_url, mid, None);
                    }
                }
                let name = if buf.trim().is_empty() {
                    VENDORS[vendor].label.to_string()
                } else {
                    buf.trim().to_string()
                };
                let keep_old = editing_id.as_ref().and_then(|id| {
                    state
                        .store
                        .models
                        .iter()
                        .find(|m| &m.id == id)
                        .map(|m| m.api_key.clone())
                });
                state.phase = Phase::EnterKey {
                    vendor,
                    name,
                    buf: String::new(),
                    cursor: 0,
                    editing_id,
                    keep_old_key: keep_old,
                };
                ModelManagerOutcome::Changed
            }
            TextPhase::Key {
                vendor,
                name,
                editing_id,
                keep_old_key,
            } => {
                let api_key = if buf.trim().is_empty() {
                    keep_old_key.unwrap_or_default()
                } else {
                    buf.trim().to_string()
                };
                if api_key.is_empty() {
                    state.phase = Phase::EnterKey {
                        vendor,
                        name,
                        buf: String::new(),
                        cursor: 0,
                        editing_id,
                        keep_old_key: None,
                    };
                    return ModelManagerOutcome::Changed;
                }
                let v = &VENDORS[vendor];
                if v.custom_url {
                    let default_url = editing_id
                        .as_ref()
                        .and_then(|id| {
                            state
                                .store
                                .models
                                .iter()
                                .find(|m| &m.id == id)
                                .map(|m| m.base_url.clone())
                        })
                        .unwrap_or_else(|| v.base_url.to_string());
                    let cur = default_url.len();
                    state.phase = Phase::EnterBaseUrl {
                        vendor,
                        name,
                        api_key,
                        buf: default_url,
                        cursor: cur,
                        editing_id,
                    };
                } else {
                    begin_pick(
                        state,
                        vendor,
                        name,
                        api_key,
                        v.base_url.to_string(),
                        editing_id,
                    );
                }
                ModelManagerOutcome::Changed
            }
            TextPhase::BaseUrl {
                vendor,
                name,
                api_key,
                editing_id,
            } => {
                let base = buf.trim().to_string();
                begin_pick(state, vendor, name, api_key, base, editing_id);
                ModelManagerOutcome::Changed
            }
        },
        _ => ModelManagerOutcome::Unchanged,
    }
}

fn rewrite_text_phase(
    state: &mut ModelManagerState,
    buf: String,
    cursor: usize,
    phase: TextPhase,
) {
    let cursor = clamp_cursor(&buf, cursor);
    state.phase = match phase {
        TextPhase::Name { vendor, editing_id } => Phase::EnterName {
            vendor,
            buf,
            cursor,
            editing_id,
        },
        TextPhase::Key {
            vendor,
            name,
            editing_id,
            keep_old_key,
        } => Phase::EnterKey {
            vendor,
            name,
            buf,
            cursor,
            editing_id,
            keep_old_key,
        },
        TextPhase::BaseUrl {
            vendor,
            name,
            api_key,
            editing_id,
        } => Phase::EnterBaseUrl {
            vendor,
            name,
            api_key,
            buf,
            cursor,
            editing_id,
        },
    };
}

fn begin_pick(
    state: &mut ModelManagerState,
    vendor: usize,
    name: String,
    api_key: String,
    base_url: String,
    editing_id: Option<String>,
) {
    let vendor_id = VENDORS[vendor].id.to_string();
    state.fetch_rx = Some(start_fetch(base_url.clone(), api_key.clone(), vendor_id));
    state.phase = Phase::PickModel {
        vendor,
        name,
        api_key,
        base_url,
        editing_id,
        models: vec![],
        cursor: 0,
        note: if i18n::is_zh() {
            "正在获取模型列表…".into()
        } else {
            "Fetching models…".into()
        },
        fetching: true,
    };
}

fn handle_pick(
    state: &mut ModelManagerState,
    key: &KeyEvent,
    vendor: usize,
    name: String,
    api_key: String,
    base_url: String,
    editing_id: Option<String>,
    models: Vec<String>,
    cursor: usize,
    note: String,
    fetching: bool,
) -> ModelManagerOutcome {
    if fetching {
        return ModelManagerOutcome::Changed; // wait for poll
    }
    let n = models.len().max(1);
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let c = if cursor == 0 { n - 1 } else { cursor - 1 };
            state.phase = Phase::PickModel {
                vendor,
                name,
                api_key,
                base_url,
                editing_id,
                models,
                cursor: c.min(n - 1),
                note,
                fetching: false,
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let c = if models.is_empty() {
                0
            } else {
                (cursor + 1) % models.len()
            };
            state.phase = Phase::PickModel {
                vendor,
                name,
                api_key,
                base_url,
                editing_id,
                models,
                cursor: c,
                note,
                fetching: false,
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            begin_pick(state, vendor, name, api_key, base_url, editing_id);
            ModelManagerOutcome::Changed
        }
        KeyCode::Char('m') | KeyCode::Char('M') => {
            // 手动输入模型 ID
            let seed = models.get(cursor).cloned().unwrap_or_default();
            let cur = seed.len();
            state.phase = Phase::EnterName {
                vendor,
                buf: seed,
                cursor: cur,
                editing_id: Some(format!("__mid__{vendor}\x1f{name}\x1f{api_key}\x1f{base_url}")),
            };
            ModelManagerOutcome::Changed
        }
        KeyCode::Enter => {
            if models.is_empty() {
                return ModelManagerOutcome::Unchanged;
            }
            let mid = models[cursor.min(models.len() - 1)].clone();
            finish_save(state, vendor, name, api_key, base_url, mid, editing_id)
        }
        _ => ModelManagerOutcome::Unchanged,
    }
}

/// Intercept EnterName when editing_id is __mid__... for manual model id entry
fn finish_save(
    state: &mut ModelManagerState,
    vendor: usize,
    name: String,
    api_key: String,
    base_url: String,
    model_id: String,
    editing_id: Option<String>,
) -> ModelManagerOutcome {
    let v = &VENDORS[vendor];
    let slug = sanitize_slug(&format!("{}-{}", v.id, model_id));
    let id = editing_id
        .clone()
        .filter(|s| !s.starts_with("__"))
        .unwrap_or_else(now_id);

    let format = if v.id == "anthropic" {
        "anthropic".into()
    } else {
        "openai".into()
    };
    let entry = ManagedModel {
        id: id.clone(),
        name: name.clone(),
        provider: v.id.to_string(),
        model: model_id,
        api_key,
        base_url,
        format,
        slug: slug.clone(),
    };

    if let Some(pos) = state.store.models.iter().position(|m| m.id == id) {
        state.store.models[pos] = entry.clone();
    } else {
        state.store.models.push(entry.clone());
    }
    save_store(&state.store);

    // auto-apply
    match write_config_model(&entry) {
        Ok(()) => {
            state.store.current = Some(id);
            save_store(&state.store);
            let cursor = state
                .store
                .models
                .iter()
                .position(|m| m.id == entry.id)
                .unwrap_or(0);
            state.phase = Phase::List {
                cursor,
                status: if i18n::is_zh() {
                    format!("已保存并启用 {} · 请重启 Yis Cli", entry.name)
                } else {
                    format!("Saved & enabled {} · restart Yis Cli", entry.name)
                },
            };
            ModelManagerOutcome::Applied {
                slug,
                name: entry.name,
            }
        }
        Err(e) => {
            state.phase = Phase::List {
                cursor: 0,
                status: format!("保存失败: {e}"),
            };
            ModelManagerOutcome::Changed
        }
    }
}

// Fix EnterName for __mid__ path - patch handle_text Name branch
// We need to re-open handle_text for Name when editing_id starts with __mid__

// Actually the handle_text for Name always goes to EnterKey. Fix by checking in TextPhase::Name enter:

// I'll update rewrite - the handle_text Name Enter already exists. I need to modify it.

// Re-read handle_text for Name - I'll use a search_replace after.

// ── Render ────────────────────────────────────────────────────────────

pub fn render_model_manager(
    buf: &mut Buffer,
    area: Rect,
    state: &mut ModelManagerState,
    theme: &Theme,
) {
    let _ = state.poll_fetch();

    let title = if i18n::is_zh() {
        "Yis · 模型管理"
    } else {
        "Yis · Model Manager"
    };

    let sc = |label: &'static str| Shortcut {
        label,
        clickable: false,
        id: 0,
    };

    let shortcuts: Vec<Shortcut<'_>> = match &state.phase {
        Phase::List { .. } => vec![
            sc(if i18n::is_zh() {
                "↑↓ 移动"
            } else {
                "↑↓ move"
            }),
            sc(if i18n::is_zh() {
                "Enter 选用/新增"
            } else {
                "Enter select/add"
            }),
            sc(if i18n::is_zh() { "e 编辑" } else { "e edit" }),
            sc(if i18n::is_zh() { "d 删除" } else { "d del" }),
            sc("Esc"),
        ],
        Phase::Vendor { .. } => vec![
            sc(if i18n::is_zh() {
                "←→ 选厂商"
            } else {
                "←→ vendor"
            }),
            sc("Enter"),
            sc("Esc"),
        ],
        Phase::EnterName { .. } | Phase::EnterKey { .. } | Phase::EnterBaseUrl { .. } => {
            vec![sc("Enter"), sc("Esc")]
        }
        Phase::PickModel { .. } => vec![
            sc(if i18n::is_zh() {
                "↑↓ 选择"
            } else {
                "↑↓ select"
            }),
            sc("Enter"),
            sc(if i18n::is_zh() {
                "m 手动"
            } else {
                "m manual"
            }),
            sc(if i18n::is_zh() { "r 重拉" } else { "r reload" }),
            sc("Esc"),
        ],
    };

    let config = ModalWindowConfig {
        title,
        tabs: None,
        shortcuts: &shortcuts,
        sizing: ModalSizing {
            width_pct: 0.72,
            max_width: 88,
            min_width: 48,
            v_margin: 3,
            h_pad: 2,
            v_pad: 1,
            footer_lines: 2,
        },
        fold_info: None,
    };

    let Some(content) =
        modal_window::render_modal_window(buf, area, &mut state.window, &config, theme)
    else {
        return;
    };
    let lines = build_lines(state, theme);
    Paragraph::new(lines).render(content.content, buf);
}

/// 在 `cursor` 字节位置插入光标符；`mask` 时用 * 显示内容。
fn render_with_cursor(buf: &str, cursor: usize, mask: bool) -> String {
    let cur = clamp_cursor(buf, cursor);
    let (left, right) = buf.split_at(cur);
    if mask {
        let l = "*".repeat(left.chars().count());
        let r = "*".repeat(right.chars().count());
        format!("{l}▌{r}")
    } else {
        format!("{left}▌{right}")
    }
}

fn build_lines(state: &ModelManagerState, theme: &Theme) -> Vec<Line<'static>> {
    let accent = Style::default()
        .fg(theme.accent_user)
        .add_modifier(Modifier::BOLD);
    let normal = Style::default().fg(theme.text_primary);
    let dim = Style::default().fg(theme.gray);
    let ok = Style::default().fg(theme.accent_success);

    let mut lines = Vec::new();
    match &state.phase {
        Phase::List { cursor, status } => {
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "自定义模型（Enter 选用 · e 编辑 · d 删除）"
                } else {
                    "Custom models"
                },
                accent,
            )));
            lines.push(Line::from(""));
            if state.store.models.is_empty() {
                lines.push(Line::from(Span::styled(
                    if i18n::is_zh() {
                        "  (暂无模型，选最下面「+ 新增模型」)"
                    } else {
                        "  (empty — choose + Add model)"
                    },
                    dim,
                )));
            }
            for (i, m) in state.store.models.iter().enumerate() {
                let on = i == *cursor;
                let star = if state.store.current.as_deref() == Some(&m.id) {
                    "* "
                } else {
                    "  "
                };
                let text = format!(
                    "{star}{:<12}  {}/{}  {}",
                    m.name,
                    m.provider,
                    m.model,
                    mask_key(&m.api_key)
                );
                if on {
                    lines.push(Line::from(Span::styled(
                        format!("  > {text}"),
                        Style::default()
                            .fg(theme.text_primary)
                            .bg(theme.bg_highlight)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(format!("    {text}"), normal)));
                }
            }
            let add_i = state.store.models.len();
            let add_label = if i18n::is_zh() {
                "+ 新增模型"
            } else {
                "+ Add model"
            };
            if *cursor == add_i {
                lines.push(Line::from(Span::styled(
                    format!("  > {add_label}"),
                    Style::default()
                        .fg(theme.accent_user)
                        .bg(theme.bg_highlight)
                        .add_modifier(Modifier::BOLD),
                )));
            } else {
                lines.push(Line::from(Span::styled(
                    format!("    {add_label}"),
                    Style::default().fg(theme.accent_user),
                )));
            }
            if !status.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(status.clone(), ok)));
            }
        }
        Phase::Vendor { idx } => {
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "新增模型 — 选择厂商"
                } else {
                    "Add model — pick vendor"
                },
                accent,
            )));
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "← → / ↑↓ 切换   Enter 下一步"
                } else {
                    "arrows to move · Enter next"
                },
                dim,
            )));
            lines.push(Line::from(""));
            for (i, v) in VENDORS.iter().enumerate() {
                if i == *idx {
                    lines.push(Line::from(Span::styled(
                        format!("  > {}  {}/{}", v.label, v.id, v.model),
                        Style::default()
                            .fg(theme.text_primary)
                            .bg(theme.bg_highlight)
                            .add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(format!("    {}", v.label), normal)));
                }
            }
        }
        Phase::EnterName {
            buf,
            cursor,
            editing_id,
            ..
        } => {
            let title = if editing_id.as_deref().is_some_and(|s| s.starts_with("__mid__")) {
                if i18n::is_zh() {
                    "手动输入模型 ID"
                } else {
                    "Type model id"
                }
            } else if i18n::is_zh() {
                "显示名称"
            } else {
                "Display name"
            };
            lines.push(Line::from(Span::styled(title, accent)));
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "←→ 移动光标 · Ctrl+V/Cmd+V 粘贴 · Enter 确认"
                } else {
                    "←→ move · paste · Enter confirm"
                },
                dim,
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  > {}", render_with_cursor(buf, *cursor, false)),
                Style::default().fg(theme.text_primary).bg(theme.bg_highlight),
            )));
        }
        Phase::EnterKey {
            buf,
            cursor,
            keep_old_key,
            ..
        } => {
            lines.push(Line::from(Span::styled("API Key", accent)));
            if let Some(old) = keep_old_key {
                lines.push(Line::from(Span::styled(
                    if i18n::is_zh() {
                        format!("  回车保留 {} · 可粘贴新密钥", mask_key(old))
                    } else {
                        format!("  Enter keeps {} · or paste new", mask_key(old))
                    },
                    dim,
                )));
            } else {
                lines.push(Line::from(Span::styled(
                    if i18n::is_zh() {
                        "←→ 移动 · 粘贴 · Enter 确认"
                    } else {
                        "←→ move · paste · Enter"
                    },
                    dim,
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  > {}", render_with_cursor(buf, *cursor, true)),
                Style::default().fg(theme.text_primary).bg(theme.bg_highlight),
            )));
        }
        Phase::EnterBaseUrl { buf, cursor, .. } => {
            lines.push(Line::from(Span::styled("Base URL", accent)));
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "←→ 移动 · 粘贴 · Enter 确认"
                } else {
                    "←→ move · paste · Enter"
                },
                dim,
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("  > {}", render_with_cursor(buf, *cursor, false)),
                Style::default().fg(theme.text_primary).bg(theme.bg_highlight),
            )));
        }
        Phase::PickModel {
            models,
            cursor,
            note,
            fetching,
            ..
        } => {
            lines.push(Line::from(Span::styled(
                if i18n::is_zh() {
                    "选择模型"
                } else {
                    "Pick model"
                },
                accent,
            )));
            lines.push(Line::from(Span::styled(note.clone(), dim)));
            lines.push(Line::from(""));
            if *fetching {
                lines.push(Line::from(Span::styled(
                    if i18n::is_zh() {
                        "  正在拉取…"
                    } else {
                        "  Loading…"
                    },
                    dim,
                )));
            } else if models.is_empty() {
                lines.push(Line::from(Span::styled(
                    if i18n::is_zh() {
                        "  (空列表，按 m 手动输入)"
                    } else {
                        "  (empty — press m)"
                    },
                    dim,
                )));
            } else {
                let win = 12usize;
                let start = cursor.saturating_sub(win / 2).min(models.len().saturating_sub(win));
                let end = (start + win).min(models.len());
                if start > 0 {
                    lines.push(Line::from(Span::styled(
                        format!("  … +{start}"),
                        dim,
                    )));
                }
                for i in start..end {
                    let label = &models[i];
                    if i == *cursor {
                        lines.push(Line::from(Span::styled(
                            format!("  > {label}"),
                            Style::default()
                                .fg(theme.text_primary)
                                .bg(theme.bg_highlight)
                                .add_modifier(Modifier::BOLD),
                        )));
                    } else {
                        lines.push(Line::from(Span::styled(format!("    {label}"), normal)));
                    }
                }
                if end < models.len() {
                    lines.push(Line::from(Span::styled(
                        format!("  … +{}", models.len() - end),
                        dim,
                    )));
                }
            }
        }
    }
    lines
}
