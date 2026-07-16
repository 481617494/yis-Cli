//! `grok models` subcommand.

use anyhow::Result;
use tokio_util::sync::CancellationToken;
use xai_yis_shell::agent::config::Config as AgentConfig;
use xai_yis_shell::cli_models::{AuthStatus, list_models};

use crate::client_identity::{PAGER_CLIENT_TYPE, PAGER_CLIENT_VERSION};

pub async fn list_available_models(agent_config: &AgentConfig) -> Result<()> {
    // Prefer listing user BYOK models from config (no cloud).
    if let Ok(user_models) = xai_yis_shell::model_presets::list_user_models()
        && !user_models.is_empty()
    {
        println!("本地已配置模型（~/.yis/config.toml）:");
        for m in &user_models {
            let mark = if m.is_default { "*" } else { " " };
            println!(
                "  {mark} {}  {}  {}",
                m.name,
                m.model,
                m.base_url
            );
        }
        println!();
        println!("管理: yis models setup  |  yis models presets  |  TUI 内 /model-add");
        return Ok(());
    }

    match AuthStatus::resolve(agent_config) {
        AuthStatus::ApiKey => println!("当前使用 XAI_API_KEY 认证。"),
        AuthStatus::LoggedIn(host) => println!("已登录：{}。", host),
        AuthStatus::ModelCredentials(model) => {
            println!("模型「{model}」使用其独立 API Key。");
        }
        AuthStatus::DeploymentKey => println!("已通过 deployment key 认证。"),
        AuthStatus::NotAuthenticated => {
            println!("本地模式：未配置模型 API Key。");
            println!("请运行: yis models setup");
            println!("或:     yis models add --preset deepseek --api-key sk-...");
            println!("预设:   yis models presets");
            return Ok(());
        }
    }
    println!();

    let cancel = CancellationToken::new();
    let spawned = crate::acp::spawn::spawn_yis_shell(agent_config.clone(), &cancel, None).await?;

    let state = list_models(&spawned.channel.tx, PAGER_CLIENT_TYPE, PAGER_CLIENT_VERSION).await?;

    if state.available_models.is_empty() {
        println!("无可用模型。本地模式请先配置厂商 API：");
        println!("  yis models setup");
        cancel.cancel();
        return Ok(());
    }

    println!("默认模型: {}", state.current_model_id.0);
    println!();
    println!("可用模型:");
    for m in state.available_models {
        if m.model_id == state.current_model_id {
            println!("  * {} (默认)", m.model_id.0);
        } else {
            println!("  - {}", m.model_id.0);
        }
    }

    cancel.cancel();
    Ok(())
}
