//! Interactive model setup (qoder-switch style) for `yis models setup`.
//!
//! Flow: list configured models → add (vendor → api key → model) → set default.
//! Writes only to `~/.yis/config.toml`. Never contacts grok.com / xAI.

use std::io::{self, Write};

use anyhow::{Context, Result, bail};
use xai_yis_shell::model_presets::{
    self, UpsertModelSpec, VendorPreset, VENDOR_PRESETS, mask_api_key,
};

/// Run interactive model setup on a TTY.
pub async fn run_interactive() -> Result<()> {
    println!("Yis Cli · 模型配置（本地 BYOK，不连接 grok.com）");
    println!("配置文件: {}", xai_yis_shell::util::config::user_config_path().display());
    println!();

    loop {
        let models = model_presets::list_user_models().unwrap_or_default();
        println!("已配置模型:");
        if models.is_empty() {
            println!("  （无）请新增至少一个厂商模型");
        } else {
            for (i, m) in models.iter().enumerate() {
                let mark = if m.is_default { "*" } else { " " };
                println!(
                    "  {mark} [{i}] {}  {}  {}  {}",
                    m.name,
                    m.model,
                    m.base_url,
                    mask_api_key(&m.api_key)
                );
            }
        }
        println!();
        println!("操作: [a] 新增  [d] 删除  [s] 设为默认  [l] 刷新列表  [q] 退出");
        print!("> ");
        let _ = io::stdout().flush();
        let cmd = read_line()?.trim().to_ascii_lowercase();
        match cmd.as_str() {
            "q" | "quit" | "exit" | "" => {
                if models.is_empty() {
                    println!("尚未配置模型。下次可运行: yis models setup");
                } else {
                    println!("完成。启动 yis 后可用 /model 切换模型。");
                }
                return Ok(());
            }
            "l" | "list" => continue,
            "a" | "add" | "n" | "+" => {
                if let Err(e) = add_model_flow().await {
                    eprintln!("新增失败: {e}");
                }
            }
            "d" | "del" | "delete" => {
                if models.is_empty() {
                    println!("没有可删除的模型");
                    continue;
                }
                print!("输入要删除的序号或 id: ");
                let _ = io::stdout().flush();
                let sel = read_line()?;
                let id = resolve_id(&models, sel.trim());
                match id {
                    Some(id) => match model_presets::delete_user_model(&id).await {
                        Ok(()) => println!("已删除 {id}"),
                        Err(e) => eprintln!("删除失败: {e}"),
                    },
                    None => println!("无效选择"),
                }
            }
            "s" | "default" => {
                if models.is_empty() {
                    println!("没有可设置的模型");
                    continue;
                }
                print!("输入要设为默认的序号或 id: ");
                let _ = io::stdout().flush();
                let sel = read_line()?;
                let id = resolve_id(&models, sel.trim());
                match id {
                    Some(id) => match model_presets::set_default_user_model(&id).await {
                        Ok(()) => println!("默认模型已设为 {id}"),
                        Err(e) => eprintln!("设置失败: {e}"),
                    },
                    None => println!("无效选择"),
                }
            }
            other => println!("未知命令: {other}"),
        }
        println!();
    }
}

async fn add_model_flow() -> Result<()> {
    println!();
    println!("选择厂商:");
    for (i, p) in VENDOR_PRESETS.iter().enumerate() {
        println!("  [{i}] {}  ({})", p.name, p.base_url);
    }
    print!("序号: ");
    let _ = io::stdout().flush();
    let idx: usize = read_line()?
        .trim()
        .parse()
        .context("请输入数字序号")?;
    let preset: &VendorPreset = VENDOR_PRESETS
        .get(idx)
        .ok_or_else(|| anyhow::anyhow!("序号越界"))?;

    let mut base_url = preset.base_url.to_string();
    if base_url.is_empty() || preset.id == "custom" || preset.id.contains("compatible") {
        print!(
            "base_url [{}]: ",
            if base_url.is_empty() {
                "必填"
            } else {
                &base_url
            }
        );
        let _ = io::stdout().flush();
        let line = read_line()?;
        if !line.trim().is_empty() {
            base_url = line.trim().to_string();
        }
        if base_url.is_empty() {
            bail!("base_url 不能为空");
        }
    }

    print!("API Key: ");
    let _ = io::stdout().flush();
    let api_key = read_line()?;
    if api_key.trim().is_empty() {
        bail!("API Key 不能为空");
    }

    println!("正在拉取模型列表（仅请求该厂商，失败则用内置列表）…");
    let ids = model_presets::fetch_vendor_model_ids(&base_url, api_key.trim(), preset.models).await;
    let model = if ids.is_empty() {
        print!("模型 ID [{}]: ", preset.default_model);
        let _ = io::stdout().flush();
        let line = read_line()?;
        if line.trim().is_empty() {
            preset.default_model.to_string()
        } else {
            line.trim().to_string()
        }
    } else {
        println!("可选模型:");
        for (i, id) in ids.iter().enumerate() {
            let mark = if id == preset.default_model { "*" } else { " " };
            println!("  {mark} [{i}] {id}");
        }
        print!("序号或直接输入模型 ID [默认 {}]: ", preset.default_model);
        let _ = io::stdout().flush();
        let line = read_line()?;
        let t = line.trim();
        if t.is_empty() {
            preset.default_model.to_string()
        } else if let Ok(i) = t.parse::<usize>() {
            ids.get(i).cloned().unwrap_or_else(|| t.to_string())
        } else {
            t.to_string()
        }
    };

    let mut spec = UpsertModelSpec::from_preset(preset, &model, api_key.trim(), true);
    spec.base_url = base_url;
    model_presets::upsert_user_model(spec).await?;
    println!("已写入模型「{model}」并设为默认。");
    Ok(())
}

fn resolve_id(models: &[model_presets::UserModelConfig], sel: &str) -> Option<String> {
    if sel.is_empty() {
        return None;
    }
    if let Ok(i) = sel.parse::<usize>() {
        return models.get(i).map(|m| m.id.clone());
    }
    models
        .iter()
        .find(|m| m.id == sel || m.model == sel)
        .map(|m| m.id.clone())
}

fn read_line() -> Result<String> {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .context("read stdin")?;
    Ok(buf.trim_end_matches(['\r', '\n']).to_string())
}

/// Non-interactive add: `yis models add --preset deepseek --api-key sk-... [--model id]`
pub async fn run_add(preset_id: &str, api_key: &str, model: Option<&str>, base_url: Option<&str>) -> Result<()> {
    let preset = model_presets::find_preset(preset_id)
        .ok_or_else(|| anyhow::anyhow!("未知厂商预设: {preset_id}"))?;
    let model = model.unwrap_or(preset.default_model);
    let mut spec = UpsertModelSpec::from_preset(preset, model, api_key, true);
    if let Some(u) = base_url {
        if !u.trim().is_empty() {
            spec.base_url = u.trim().to_string();
        }
    }
    if spec.base_url.is_empty() {
        bail!("该预设需要 --base-url");
    }
    model_presets::upsert_user_model(spec).await?;
    println!(
        "已配置模型 {}（{}）并设为默认 → {}",
        model,
        preset.name,
        xai_yis_shell::util::config::user_config_path().display()
    );
    Ok(())
}

/// Print presets for `yis models presets`.
pub fn print_presets() {
    println!("内置厂商预设:");
    for p in VENDOR_PRESETS {
        println!(
            "  {:20}  {:28}  {}",
            p.id,
            p.name,
            if p.base_url.is_empty() {
                "(需手动 base_url)"
            } else {
                p.base_url
            }
        );
        if !p.notes.is_empty() {
            println!("      {}", p.notes);
        }
    }
}
