//! `/language` (alias `/lang`) — 切换界面语言。

use crate::i18n::{self, Lang};
use crate::slash::command::{AppCtx, ArgItem, CommandExecCtx, CommandResult, SlashCommand};

/// Switch UI language between 简体中文 and English.
pub struct LanguageCommand;

impl SlashCommand for LanguageCommand {
    fn name(&self) -> &str {
        "language"
    }

    fn aliases(&self) -> &[&str] {
        &["lang", "语言"]
    }

    fn description(&self) -> &str {
        // 描述本身随当前语言变化
        if i18n::is_zh() {
            "切换界面语言（简体中文 / English）"
        } else {
            "Switch UI language (简体中文 / English)"
        }
    }

    fn usage(&self) -> &str {
        "/language [zh|en]"
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn args_required(&self) -> bool {
        false
    }

    fn arg_placeholder(&self) -> Option<&str> {
        Some("[zh|en]")
    }

    fn suggest_args(&self, _ctx: &AppCtx, _args_query: &str) -> Option<Vec<ArgItem>> {
        let cur = i18n::lang();
        Some(vec![
            ArgItem {
                display: if cur == Lang::ZhCn {
                    "简体中文 (当前)".into()
                } else {
                    "简体中文".into()
                },
                match_text: "zh".into(),
                insert_text: "zh".into(),
                description: "Chinese (Simplified)".into(),
            },
            ArgItem {
                display: if cur == Lang::EnUs {
                    "English (current)".into()
                } else {
                    "English".into()
                },
                match_text: "en".into(),
                insert_text: "en".into(),
                description: "English".into(),
            },
        ])
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let trimmed = args.trim();
        if trimmed.is_empty() {
            // 无参数：在中英文之间切换
            let next = match i18n::lang() {
                Lang::ZhCn => Lang::EnUs,
                Lang::EnUs => Lang::ZhCn,
            };
            i18n::set_lang(next);
            return CommandResult::Message(format!(
                "✓ {}: {}",
                if i18n::is_zh() {
                    "界面语言"
                } else {
                    "UI language"
                },
                next.label()
            ));
        }
        match Lang::from_code(trimmed) {
            Some(lang) => {
                i18n::set_lang(lang);
                CommandResult::Message(format!(
                    "✓ {}: {}",
                    if i18n::is_zh() {
                        "界面语言"
                    } else {
                        "UI language"
                    },
                    lang.label()
                ))
            }
            None => CommandResult::Error(if i18n::is_zh() {
                format!("未知语言: {trimmed}（可用: zh, en）")
            } else {
                format!("Unknown language: {trimmed} (use: zh, en)")
            }),
        }
    }
}

