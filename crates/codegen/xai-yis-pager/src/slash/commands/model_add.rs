//! `/model-add` / `/models` — 打开 qoder-switch 式模型管理弹窗。

use crate::app::actions::Action;
use crate::i18n;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

/// Open the interactive model manager (list / add vendor / key / fetch / pick).
pub struct ModelAddCommand;

impl SlashCommand for ModelAddCommand {
    fn name(&self) -> &str {
        "model-add"
    }

    fn aliases(&self) -> &[&str] {
        &["models", "add-model", "新增模型", "模型管理"]
    }

    fn description(&self) -> &str {
        if i18n::is_zh() {
            "管理自定义模型（选厂商 / 密钥 / 拉列表 / 编辑删除）"
        } else {
            "Manage custom models (vendor / key / fetch / edit / delete)"
        }
    }

    fn usage(&self) -> &str {
        "/model-add"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::OpenModelManager)
    }
}
