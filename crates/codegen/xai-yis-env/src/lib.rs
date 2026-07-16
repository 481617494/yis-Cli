#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    dead_code
)]
//! Backend environment presets for the Yis Cli crate family: endpoint URL
//! defaults, environment selection, and env-var test support.
//!
//! Public builds expose production endpoints. Values resolve as a `YIS_*`
//! env-var override when set, else the compiled production default.
//!
//! ## Local Mode (Yis 发行默认)
//! `YIS_LOCAL_MODE=1` (默认) 禁用所有外部网络访问，使用 `LOCAL_ENDPOINTS`。
//! 模型推理端点需用户自行配置：
//!   `YIS_XAI_API_BASE_URL` 或 `YIS_MODELS_BASE_URL` 环境变量。

/// The endpoint set for one backend environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YisBuildEndpoints {
    pub cli_chat_proxy_base_url: &'static str,
    pub asset_server_url: &'static str,
    pub relay_ws_url: &'static str,
    pub gateway_ws_url: &'static str,
    pub ws_origin: &'static str,
}

/// Production endpoints pointing to grok.com / x.ai cloud services.
const PRODUCTION_ENDPOINTS: YisBuildEndpoints = YisBuildEndpoints {
    cli_chat_proxy_base_url: "https://cli-chat-proxy.grok.com/v1",
    asset_server_url: "https://assets.grok.com",
    relay_ws_url: "wss://code.grok.com/ws/code-agent",
    gateway_ws_url: "wss://grok.com/ws/gw/",
    ws_origin: "https://grok.com",
};

/// Local-mode endpoints: all external services disabled.
/// Model inference endpoint must be configured by the user via
/// `YIS_XAI_API_BASE_URL` or `YIS_MODELS_BASE_URL` environment variables.
const LOCAL_ENDPOINTS: YisBuildEndpoints = YisBuildEndpoints {
    cli_chat_proxy_base_url: "", // no proxy in local mode
    asset_server_url: "",         // no asset server
    relay_ws_url: "",             // no relay
    gateway_ws_url: "",           // no gateway
    ws_origin: "",                // no web origin
};

pub const PROD_CLI_CHAT_PROXY_BASE_URL: &str = PRODUCTION_ENDPOINTS.cli_chat_proxy_base_url;
pub const PROD_ASSET_SERVER_URL: &str = PRODUCTION_ENDPOINTS.asset_server_url;
pub const PROD_RELAY_WS_URL: &str = PRODUCTION_ENDPOINTS.relay_ws_url;
pub const PROD_GATEWAY_WS_URL: &str = PRODUCTION_ENDPOINTS.gateway_ws_url;
pub const PROD_WS_ORIGIN: &str = PRODUCTION_ENDPOINTS.ws_origin;

/// Check whether local / privacy mode is active.
///
/// Yis 发行版以本地安全为默认：
/// - **Release 构建始终开启本地模式**（忽略 `YIS_LOCAL_MODE=0`），禁止
///   grok.com / auth.x.ai / cli-chat-proxy 授权与云端上传。
/// - **Debug 构建**仍可用 `YIS_LOCAL_MODE=0` 临时恢复上游云行为以便开发。
/// - `YIS_OFFLINE=1` 在任何构建下强制本地模式。
///
/// This is the canonical source of truth shared between all crates.
pub fn is_local_mode() -> bool {
    if env_truthy("YIS_OFFLINE") {
        return true;
    }
    // Release: always local for privacy (no cloud auth / upload escape hatch).
    #[cfg(not(debug_assertions))]
    {
        return true;
    }
    #[cfg(debug_assertions)]
    {
        match std::env::var("YIS_LOCAL_MODE") {
            Ok(v) => {
                let t = v.trim().to_ascii_lowercase();
                !(t == "0" || t == "false" || t == "no" || t == "off")
            }
            // Debug 默认本地模式
            Err(_) => true,
        }
    }
}

/// Check if the given env var has a truthy value.
pub fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| {
            let t = v.trim().to_ascii_lowercase();
            t == "1" || t == "true" || t == "yes" || t == "on"
        })
        .unwrap_or(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YisBuildEnvironment {
    #[default]
    Production,
}
impl YisBuildEnvironment {
    pub fn from_flags(_dev: bool, _staging: bool) -> Self {
        YisBuildEnvironment::Production
    }
    /// Indicator string for display; `None` for Production.
    pub fn indicator(&self) -> Option<&'static str> {
        match self {
            YisBuildEnvironment::Production => None,
        }
    }
    pub fn is_production(&self) -> bool {
        matches!(self, YisBuildEnvironment::Production)
    }
    fn env_prefix(&self) -> &'static str {
        match self {
            YisBuildEnvironment::Production => "YIS_PRODUCTION",
        }
    }
    /// Compiled endpoint set for this environment.
    ///
    /// In local mode (`YIS_LOCAL_MODE=1`, the Yis default), returns
    /// [`LOCAL_ENDPOINTS`] — all external services disabled, model
    /// inference points to localhost.
    pub fn endpoints(&self) -> YisBuildEndpoints {
        if is_local_mode() {
            return LOCAL_ENDPOINTS;
        }
        match self {
            YisBuildEnvironment::Production => PRODUCTION_ENDPOINTS,
        }
    }
    /// Env-var override when set, else the compiled endpoint.
    fn resolve(&self, var_suffix: &str, compiled: &'static str) -> String {
        std::env::var(format!("{}{var_suffix}", self.env_prefix()))
            .unwrap_or_else(|_| compiled.to_string())
    }
    pub fn cli_chat_proxy_base_url(&self) -> String {
        self.resolve(
            "_CLI_CHAT_PROXY_BASE_URL",
            self.endpoints().cli_chat_proxy_base_url,
        )
    }
    pub fn ws_origin(&self) -> String {
        self.resolve("_WS_ORIGIN", self.endpoints().ws_origin)
    }
    pub fn asset_server_url(&self) -> String {
        self.resolve("_ASSET_SERVER_URL", self.endpoints().asset_server_url)
    }
    /// The relay WebSocket URL (Web Frontend at `grok.com/code` driving a
    /// local agent). Not the cloud-sandbox gateway ([`Self::gateway_ws_url`]);
    /// the two speak different protocols.
    pub fn relay_ws_url(&self) -> String {
        self.resolve("_WS_URL", self.endpoints().relay_ws_url)
    }
    /// The gateway WebSocket URL for `/cloud new` sandboxes. The shell's
    /// `YIS_GATEWAY_URL` opt-in takes precedence.
    pub fn gateway_ws_url(&self) -> String {
        self.resolve("_GATEWAY_WS_URL", self.endpoints().gateway_ws_url)
    }
}
impl std::fmt::Display for YisBuildEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YisBuildEnvironment::Production => write!(f, "production"),
        }
    }
}
/// Serializes env-var mutation across tests; `std::env` is process-global.
#[cfg(test)]
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
#[cfg(test)]
fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner())
}
/// RAII env-var override for tests: constructors snapshot the prior value
/// under [`ENV_LOCK`], `Drop` restores it, panics included.
#[cfg(test)]
pub struct EnvVarGuard {
    key: &'static str,
    prev: Option<String>,
    _lock: std::sync::MutexGuard<'static, ()>,
}
#[cfg(test)]
impl EnvVarGuard {
    pub fn set(key: &'static str, value: &str) -> Self {
        let lock = env_lock();
        let prev = std::env::var(key).ok();
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            prev,
            _lock: lock,
        }
    }
    pub fn remove(key: &'static str) -> Self {
        let lock = env_lock();
        let prev = std::env::var(key).ok();
        unsafe { std::env::remove_var(key) };
        Self {
            key,
            prev,
            _lock: lock,
        }
    }
    /// Update the value while still holding the env lock.
    pub fn set_value(&self, value: &str) {
        unsafe { std::env::set_var(self.key, value) };
    }
}
#[cfg(test)]
impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match self.prev.take() {
            Some(prev) => unsafe { std::env::set_var(self.key, prev) },
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /// The env-var prefixes are an operator interface; do not rename.
    #[test]
    fn test_env_prefix() {
        assert_eq!(
            YisBuildEnvironment::Production.env_prefix(),
            "YIS_PRODUCTION"
        );
    }
    #[test]
    fn env_var_guard_set_value_updates_then_restores_on_drop() {
        const KEY: &str = "YIS_ENV_VAR_GUARD_SET_VALUE_PROBE";
        let before = std::env::var(KEY).ok();
        {
            let guard = EnvVarGuard::set(KEY, "initial");
            assert_eq!(std::env::var(KEY).ok().as_deref(), Some("initial"));
            guard.set_value("updated");
            assert_eq!(
                std::env::var(KEY).ok().as_deref(),
                Some("updated"),
                "set_value must update the env var while the guard is live"
            );
        }
        assert_eq!(
            std::env::var(KEY).ok(),
            before,
            "Drop must restore the pre-guard snapshot (was {before:?})"
        );
    }
    /// Guards against conflating the relay and gateway endpoints (a relay
    /// loop mistakenly connecting to `wss://grok.com/ws/gw/`).
    #[test]
    fn relay_and_gateway_urls_are_distinct() {
        assert_ne!(
            YisBuildEnvironment::Production.relay_ws_url(),
            YisBuildEnvironment::Production.gateway_ws_url(),
        );
    }
    #[test]
    fn test_from_flags() {
        assert_eq!(
            YisBuildEnvironment::from_flags(false, false),
            YisBuildEnvironment::Production
        );
    }
}
