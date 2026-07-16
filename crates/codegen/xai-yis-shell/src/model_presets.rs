//! Vendor model presets and config.toml CRUD (qoder-switch style).
//!
//! Users configure third-party / domestic providers without any xAI cloud
//! login. Presets are written to `~/.yis/config.toml` as `[model.<id>]`
//! tables with `base_url` + `api_key`.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use toml::Value as TomlValue;
use toml::map::Map as TomlMap;

use crate::util::config::{atomic_write_string, lock_config_writes, read_to_string_or_empty, user_config_path};

/// API wire format for a preset / user model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelFormat {
    /// OpenAI Chat Completions (`/v1/chat/completions`).
    Openai,
    /// Anthropic Messages (`/v1/messages`).
    Anthropic,
}

impl ModelFormat {
    pub fn api_backend(self) -> &'static str {
        match self {
            ModelFormat::Openai => "chat_completions",
            ModelFormat::Anthropic => "messages",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "anthropic" | "messages" | "claude" => ModelFormat::Anthropic,
            _ => ModelFormat::Openai,
        }
    }
}

/// Built-in vendor preset (qoder-switch compatible fields).
#[derive(Debug, Clone)]
pub struct VendorPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub provider: &'static str,
    pub default_model: &'static str,
    pub base_url: &'static str,
    pub format: ModelFormat,
    pub models: &'static [&'static str],
    pub website: &'static str,
    pub notes: &'static str,
}

/// All built-in presets shown in CLI / TUI setup wizards.
pub static VENDOR_PRESETS: &[VendorPreset] = &[
    VendorPreset {
        id: "deepseek",
        name: "DeepSeek",
        provider: "deepseek",
        default_model: "deepseek-chat",
        base_url: "https://api.deepseek.com/v1",
        format: ModelFormat::Openai,
        models: &[
            "deepseek-chat",
            "deepseek-reasoner",
            "deepseek-v4-pro",
            "deepseek-v4-flash",
        ],
        website: "https://platform.deepseek.com",
        notes: "填 API Key 即可",
    },
    VendorPreset {
        id: "bailian",
        name: "阿里云百炼",
        provider: "bailian",
        default_model: "qwen3.5-plus-cp",
        base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1",
        format: ModelFormat::Openai,
        models: &[
            "qwen3.5-plus-cp",
            "qwen3-coder-plus",
            "qwen-max",
            "qwen-plus",
            "qwen-turbo",
        ],
        website: "https://bailian.console.aliyun.com",
        notes: "OpenAI 兼容模式",
    },
    VendorPreset {
        id: "zhipu",
        name: "智谱 GLM",
        provider: "zhipu",
        default_model: "glm-4.6",
        base_url: "https://open.bigmodel.cn/api/paas/v4",
        format: ModelFormat::Openai,
        models: &["glm-4.6", "glm-4.5", "glm-5.2", "glm-4-flash", "glm-4-plus"],
        website: "https://open.bigmodel.cn",
        notes: "",
    },
    VendorPreset {
        id: "kimi",
        name: "Kimi (Moonshot)",
        provider: "moonshot",
        default_model: "kimi-k2.5",
        base_url: "https://api.moonshot.cn/v1",
        format: ModelFormat::Openai,
        models: &[
            "kimi-k2.5",
            "kimi-k2.7-code",
            "moonshot-v1-128k",
            "moonshot-v1-32k",
            "moonshot-v1-8k",
        ],
        website: "https://platform.moonshot.cn",
        notes: "",
    },
    VendorPreset {
        id: "minimax",
        name: "MiniMax",
        provider: "minimax",
        default_model: "MiniMax-M2.5",
        base_url: "https://api.minimaxi.com/v1",
        format: ModelFormat::Openai,
        models: &["MiniMax-M2.5", "MiniMax-M3", "MiniMax-Text-01"],
        website: "https://platform.minimaxi.com",
        notes: "",
    },
    VendorPreset {
        id: "mimo",
        name: "小米 MIMO",
        provider: "mimo",
        default_model: "mimo-v2-pro",
        base_url: "https://api.xiaomimimo.com/v1",
        format: ModelFormat::Openai,
        models: &["mimo-v2-pro", "mimo-v2-flash"],
        website: "https://platform.xiaomimimo.com",
        notes: "",
    },
    VendorPreset {
        id: "openai-compatible",
        name: "OpenAI 兼容中转",
        provider: "openai",
        default_model: "gpt-4o",
        base_url: "https://api.openai.com/v1",
        format: ModelFormat::Openai,
        models: &["gpt-4o", "gpt-4o-mini", "gpt-4.1", "o3", "o4-mini"],
        website: "",
        notes: "任意 OpenAI 兼容网关：改 base_url + model + api_key",
    },
    VendorPreset {
        id: "anthropic-compatible",
        name: "Anthropic 兼容中转",
        provider: "anthropic",
        default_model: "claude-sonnet-4-5",
        base_url: "https://api.anthropic.com",
        format: ModelFormat::Anthropic,
        models: &[
            "claude-sonnet-4-5",
            "claude-opus-4",
            "claude-haiku-4-5",
        ],
        website: "",
        notes: "Anthropic Messages；Key 走 extra_headers x-api-key",
    },
    VendorPreset {
        id: "custom",
        name: "自定义",
        provider: "custom",
        default_model: "custom-model",
        base_url: "",
        format: ModelFormat::Openai,
        models: &[],
        website: "",
        notes: "手动填写 base_url / model / api_key",
    },
];

/// User-facing model entry for list / setup UIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModelConfig {
    /// Catalog key / config.toml table name (`[model."<id>"]`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Model id sent to the provider.
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub api_backend: String,
    pub context_window: u64,
    /// Whether this id is `[models].default`.
    pub is_default: bool,
}

/// Spec for creating or updating a model entry.
#[derive(Debug, Clone)]
pub struct UpsertModelSpec {
    pub id: String,
    pub name: String,
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub api_backend: String,
    pub context_window: u64,
    /// Also set `[models].default = id`.
    pub set_as_default: bool,
}

impl UpsertModelSpec {
    pub fn from_preset(preset: &VendorPreset, model: &str, api_key: &str, set_as_default: bool) -> Self {
        let model = if model.trim().is_empty() {
            preset.default_model
        } else {
            model.trim()
        };
        // Use model id as catalog key so /model deepseek-chat works naturally.
        let id = model.to_string();
        Self {
            id: id.clone(),
            name: preset.name.to_string(),
            model: model.to_string(),
            base_url: preset.base_url.to_string(),
            api_key: api_key.trim().to_string(),
            api_backend: preset.format.api_backend().to_string(),
            context_window: 128_000,
            set_as_default,
        }
    }
}

pub fn find_preset(id_or_provider: &str) -> Option<&'static VendorPreset> {
    let q = id_or_provider.trim().to_ascii_lowercase();
    VENDOR_PRESETS.iter().find(|p| {
        p.id.eq_ignore_ascii_case(&q)
            || p.provider.eq_ignore_ascii_case(&q)
            || p.name.to_ascii_lowercase() == q
    })
}

/// List `[model.*]` entries from the user config.toml (not effective merge).
pub fn list_user_models() -> Result<Vec<UserModelConfig>> {
    let path = user_config_path();
    let raw = read_to_string_or_empty(&path).with_context(|| format!("read {}", path.display()))?;
    let root: TomlValue = if raw.trim().is_empty() {
        TomlValue::Table(TomlMap::new())
    } else {
        toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?
    };
    let default_id = root
        .get("models")
        .and_then(|m| m.get("default"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut out = Vec::new();
    let Some(model_table) = root.get("model").and_then(|v| v.as_table()) else {
        return Ok(out);
    };
    for (id, entry) in model_table {
        let Some(tbl) = entry.as_table() else {
            continue;
        };
        let model = tbl
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or(id)
            .to_string();
        let name = tbl
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&model)
            .to_string();
        let base_url = tbl
            .get("base_url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let api_key = tbl
            .get("api_key")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let api_backend = tbl
            .get("api_backend")
            .and_then(|v| v.as_str())
            .unwrap_or("chat_completions")
            .to_string();
        let context_window = tbl
            .get("context_window")
            .and_then(|v| v.as_integer())
            .map(|n| n as u64)
            .unwrap_or(128_000);
        out.push(UserModelConfig {
            is_default: id.as_str() == default_id,
            id: id.clone(),
            name,
            model,
            base_url,
            api_key,
            api_backend,
            context_window,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}

/// Whether any user model has its own API key (or non-empty env would be needed
/// at runtime — here we only check inline `api_key`).
pub fn has_configured_byok_model() -> bool {
    list_user_models()
        .map(|models| models.iter().any(|m| !m.api_key.trim().is_empty() && !m.base_url.trim().is_empty()))
        .unwrap_or(false)
}

/// Insert or replace `[model."<id>"]` and optionally set default.
pub async fn upsert_user_model(spec: UpsertModelSpec) -> Result<()> {
    if spec.id.trim().is_empty() {
        bail!("model id must not be empty");
    }
    if spec.base_url.trim().is_empty() {
        bail!("base_url must not be empty");
    }
    if spec.api_key.trim().is_empty() {
        bail!("api_key must not be empty");
    }

    let _guard = lock_config_writes().await;
    let path = user_config_path();
    let raw = read_to_string_or_empty(&path).with_context(|| format!("read {}", path.display()))?;
    let mut root: TomlValue = if raw.trim().is_empty() {
        TomlValue::Table(TomlMap::new())
    } else {
        toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?
    };
    if !matches!(root, TomlValue::Table(_)) {
        root = TomlValue::Table(TomlMap::new());
    }
    let table = root.as_table_mut().expect("root table");

    // [models].default
    if spec.set_as_default {
        let models = table
            .entry("models".to_string())
            .or_insert_with(|| TomlValue::Table(TomlMap::new()));
        if !matches!(models, TomlValue::Table(_)) {
            *models = TomlValue::Table(TomlMap::new());
        }
        if let TomlValue::Table(mt) = models {
            mt.insert(
                "default".to_string(),
                TomlValue::String(spec.id.clone()),
            );
        }
    }

    // [model."<id>"]
    let model_root = table
        .entry("model".to_string())
        .or_insert_with(|| TomlValue::Table(TomlMap::new()));
    if !matches!(model_root, TomlValue::Table(_)) {
        *model_root = TomlValue::Table(TomlMap::new());
    }
    if let TomlValue::Table(models) = model_root {
        let mut entry = TomlMap::new();
        entry.insert("model".into(), TomlValue::String(spec.model.clone()));
        entry.insert("name".into(), TomlValue::String(spec.name.clone()));
        entry.insert(
            "base_url".into(),
            TomlValue::String(spec.base_url.trim_end_matches('/').to_string()),
        );
        entry.insert(
            "api_backend".into(),
            TomlValue::String(spec.api_backend.clone()),
        );
        entry.insert("api_key".into(), TomlValue::String(spec.api_key.clone()));
        entry.insert(
            "context_window".into(),
            TomlValue::Integer(spec.context_window as i64),
        );
        // Anthropic: also set x-api-key header so messages backend auth works.
        if spec.api_backend == "messages" {
            let mut headers = TomlMap::new();
            headers.insert(
                "x-api-key".into(),
                TomlValue::String(spec.api_key.clone()),
            );
            headers.insert(
                "anthropic-version".into(),
                TomlValue::String("2023-06-01".into()),
            );
            entry.insert("extra_headers".into(), TomlValue::Table(headers));
        }
        models.insert(spec.id.clone(), TomlValue::Table(entry));
    }

    let toml_str = toml::to_string_pretty(&root)?;
    atomic_write_string(&path, &toml_str)
        .with_context(|| format!("write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

/// Remove a `[model."<id>"]` entry. If it was default, clear default.
pub async fn delete_user_model(id: &str) -> Result<()> {
    let id = id.trim();
    if id.is_empty() {
        bail!("model id must not be empty");
    }
    let _guard = lock_config_writes().await;
    let path = user_config_path();
    let raw = read_to_string_or_empty(&path).with_context(|| format!("read {}", path.display()))?;
    if raw.trim().is_empty() {
        bail!("config.toml is empty; nothing to delete");
    }
    let mut root: TomlValue =
        toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    let table = root
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("config root is not a table"))?;

    let removed = table
        .get_mut("model")
        .and_then(|v| v.as_table_mut())
        .map(|m| m.remove(id).is_some())
        .unwrap_or(false);
    if !removed {
        bail!("model '{id}' not found in config.toml");
    }

    if let Some(TomlValue::Table(models)) = table.get_mut("models")
        && models
            .get("default")
            .and_then(|v| v.as_str())
            .is_some_and(|d| d == id)
    {
        models.remove("default");
    }

    let toml_str = toml::to_string_pretty(&root)?;
    atomic_write_string(&path, &toml_str)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

/// Set `[models].default` only.
pub async fn set_default_user_model(id: &str) -> Result<()> {
    let id = id.trim();
    if id.is_empty() {
        bail!("model id must not be empty");
    }
    // Verify the model exists.
    let models = list_user_models()?;
    if !models.iter().any(|m| m.id == id) {
        bail!("model '{id}' not found; add it first with model setup");
    }
    crate::util::config::set_default_model(id.to_string()).await
}

/// Fetch model ids from an OpenAI-compatible `GET {base}/models`.
/// On failure returns the preset fallback list (or empty).
pub async fn fetch_vendor_model_ids(
    base_url: &str,
    api_key: &str,
    fallback: &[&str],
) -> Vec<String> {
    let base = base_url.trim().trim_end_matches('/');
    if base.is_empty() {
        return fallback.iter().map(|s| (*s).to_string()).collect();
    }
    let url = if base.ends_with("/models") {
        base.to_string()
    } else {
        format!("{base}/models")
    };
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .build()
    {
        Ok(c) => c,
        Err(_) => return fallback.iter().map(|s| (*s).to_string()).collect(),
    };
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .send()
        .await;
    let Ok(resp) = resp else {
        return fallback.iter().map(|s| (*s).to_string()).collect();
    };
    if !resp.status().is_success() {
        return fallback.iter().map(|s| (*s).to_string()).collect();
    }
    let Ok(body) = resp.json::<serde_json::Value>().await else {
        return fallback.iter().map(|s| (*s).to_string()).collect();
    };
    let mut ids: Vec<String> = body
        .get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| item.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return fallback.iter().map(|s| (*s).to_string()).collect();
    }
    ids.sort();
    ids.dedup();
    ids
}

/// Mask an API key for display: `sk-****abcd`.
pub fn mask_api_key(key: &str) -> String {
    let k = key.trim();
    if k.is_empty() {
        return "(empty)".into();
    }
    if k.len() <= 8 {
        return "****".into();
    }
    format!("{}****{}", &k[..4.min(k.len())], &k[k.len().saturating_sub(4)..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_include_deepseek() {
        assert!(find_preset("deepseek").is_some());
        assert!(find_preset("DeepSeek").is_some());
        assert_eq!(
            find_preset("deepseek").unwrap().base_url,
            "https://api.deepseek.com/v1"
        );
    }

    #[test]
    fn format_api_backend_mapping() {
        assert_eq!(ModelFormat::Openai.api_backend(), "chat_completions");
        assert_eq!(ModelFormat::Anthropic.api_backend(), "messages");
    }

    #[test]
    fn mask_key() {
        assert_eq!(mask_api_key("sk-abcdefghijklmnop"), "sk-a****mnop");
        assert_eq!(mask_api_key("short"), "****");
    }

    #[test]
    fn upsert_spec_from_preset() {
        let p = find_preset("deepseek").unwrap();
        let s = UpsertModelSpec::from_preset(p, "deepseek-reasoner", "sk-test", true);
        assert_eq!(s.id, "deepseek-reasoner");
        assert_eq!(s.model, "deepseek-reasoner");
        assert_eq!(s.api_backend, "chat_completions");
        assert!(s.set_as_default);
    }
}
