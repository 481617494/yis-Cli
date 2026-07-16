//! Yis Cli 语言与本地模式。
//!
//! ## 本地安全模式（发行默认）
//! Yis 发行版**强制本地模式**，不向 grok.com / auth.x.ai / cli-chat-proxy
//! 发起授权或云端上传。唯一允许的出站请求是用户在配置中指定的模型厂商。
//!
//! - Release：始终本地（忽略 `YIS_LOCAL_MODE=0`）
//! - Debug：可用 `YIS_LOCAL_MODE=0` 临时恢复上游云（开发用）
//! - `YIS_OFFLINE=1`：强制本地
//! - `YIS_LANG=zh|en`：界面语言（默认 zh）
//!
//! 真源：`xai_yis_env::is_local_mode` / shell `util::config::is_local_mode`。

use std::sync::atomic::{AtomicU8, Ordering};

/// 产品显示名（正式更名）。
pub const PRODUCT_NAME: &str = "Yis Cli";
/// 产品短名。
pub const PRODUCT_SHORT: &str = "Yis";

const LANG_ZH: u8 = 0;
const LANG_EN: u8 = 1;

static LANG: AtomicU8 = AtomicU8::new(LANG_ZH);

/// 界面语言。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    ZhCn,
    EnUs,
}

impl Lang {
    pub fn as_code(self) -> &'static str {
        match self {
            Lang::ZhCn => "zh",
            Lang::EnUs => "en",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Lang::ZhCn => "简体中文",
            Lang::EnUs => "English",
        }
    }

    pub fn from_code(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "zh" | "zh-cn" | "zh_cn" | "cn" | "chinese" | "中文" | "简体中文" => Some(Lang::ZhCn),
            "en" | "en-us" | "en_us" | "english" | "英文" => Some(Lang::EnUs),
            _ => None,
        }
    }
}

/// 从环境变量初始化语言（进程启动时调用一次）。
pub fn init_from_env() {
    if let Ok(v) = std::env::var("YIS_LANG")
        && let Some(lang) = Lang::from_code(&v)
    {
        set_lang(lang);
        return;
    }
    if let Ok(v) = std::env::var("YIS_LANG")
        && let Some(lang) = Lang::from_code(&v)
    {
        set_lang(lang);
        return;
    }
    // 默认简体中文
    set_lang(Lang::ZhCn);
}

pub fn set_lang(lang: Lang) {
    LANG.store(
        match lang {
            Lang::ZhCn => LANG_ZH,
            Lang::EnUs => LANG_EN,
        },
        Ordering::Relaxed,
    );
}

pub fn lang() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        LANG_EN => Lang::EnUs,
        _ => Lang::ZhCn,
    }
}

pub fn is_zh() -> bool {
    matches!(lang(), Lang::ZhCn)
}

/// 本地 / 隐私模式。委托 shell 真源（与 `xai_yis_env::is_local_mode` 一致）。
pub fn local_mode() -> bool {
    xai_yis_shell::env::is_local_mode()
}

/// 取文案：中文优先；英文回退到 `en` 参数。
pub fn t(zh: &'static str, en: &'static str) -> &'static str {
    if is_zh() { zh } else { en }
}

/// 底部快捷键栏标签（`Shift+Tab: mode` → `Shift+Tab: 模式`）。
///
/// 内部仍使用英文 `ActionDef.label` 作为标识；仅在渲染时本地化，避免破坏测试与匹配逻辑。
pub fn hint_label(en: &str) -> &str {
    if !is_zh() {
        return en;
    }
    match en {
        "mode" => "模式",
        "shortcuts" => "快捷键",
        "nav" => "导航",
        "send" => "发送",
        "send now" => "立即发送",
        "send to bg" => "后台发送",
        "cancel" => "取消",
        "quit" => "退出",
        "exit" => "退出",
        "commands" => "命令",
        "model" => "模型",
        "settings" => "设置",
        "new" => "新建",
        "prompt" => "输入框",
        "scrollback" => "回看",
        "todos" => "待办",
        "tasks" => "任务",
        "queue" => "队列",
        "sessions" => "会话",
        "extensions" => "扩展",
        "dashboard" => "面板",
        "copy" => "复制",
        "copy cmd" => "复制命令",
        "view" => "查看",
        "link" => "链接",
        "rewind" => "回退",
        "kill" => "终止",
        "fold" => "折叠",
        "all" => "全部",
        "raw" => "原文",
        "expand/collapse thinking" => "展开/折叠思考",
        "turn" => "轮次",
        "response" => "回复",
        "top/btm" => "顶/底",
        "bottom" => "底部",
        "scroll up" => "上滚",
        "scroll down" => "下滚",
        "half page up" => "半页上",
        "half page down" => "半页下",
        "page up" => "上页",
        "page down" => "下页",
        "multiline" => "多行",
        "shell" => "Shell",
        "yolo" => "自动批准",
        "always-approve" => "始终批准",
        "voice mode" => "语音模式",
        "mic" => "麦克风",
        "mouse reporting" => "鼠标报告",
        "next" => "下一个",
        "prev" => "上一个",
        "pin" => "置顶",
        "rename" => "重命名",
        "stop" => "停止",
        "group" => "分组",
        "reorder up" => "上移",
        "reorder down" => "下移",
        "location" => "位置",
        "worktree" => "工作树",
        "close overlay" => "关闭浮层",
        "prev session" => "上一会话",
        "next session" => "下一会话",
        "search" => "搜索",
        "paste" => "粘贴",
        "switch tab" => "切换标签",
        "expand" => "展开",
        "close" => "关闭",
        "delete row" => "删除行",
        "edit" => "编辑",
        "save" => "保存",
        other => other,
    }
}

/// 二次确认提示：`press again to quit` → `再按一次以退出`。
pub fn press_again_hint(label_en: &str) -> String {
    if is_zh() {
        format!("再按一次以{}", hint_label(label_en))
    } else {
        format!("press again to {}", label_en)
    }
}

/// 首页副标题。
pub fn hero_subtitle() -> &'static str {
    t(
        "欢迎使用 Yis Cli — 终端 AI 编程助手。输入 /help 查看命令，/model 切换模型。",
        "Welcome to Yis Cli — terminal AI coding assistant. /help for commands, /model to switch models.",
    )
}

/// 版本徽章产品名。
pub fn product_badge() -> &'static str {
    t("Yis Cli  ", "Yis Cli  ")
}

pub fn product_badge_beta() -> &'static str {
    t("Yis Cli  ", "Yis Cli  ")
}

pub fn quit_label() -> &'static str {
    t("退出", "Quit")
}

pub fn logout_label() -> &'static str {
    t("退出登录", "Logout")
}

pub fn switch_account_label() -> &'static str {
    t("切换账号", "Switch account")
}

pub fn trust_yes() -> &'static str {
    t("是，继续", "Yes, proceed")
}

pub fn trust_no() -> &'static str {
    t("否，退出", "No, quit")
}

pub fn trust_question() -> &'static str {
    t(
        "是否信任此目录的内容？",
        "Do you trust the contents of this directory?",
    )
}

pub fn trust_warning() -> &'static str {
    t(
        "Yis Cli 可能在此目录运行命令或修改文件，存在安全风险。",
        "Yis Cli may run or modify contents in this directory, posing security risks.",
    )
}

pub fn zdr_blocked_msg() -> &'static str {
    t(
        "当前账号暂不可用 Yis Cli。",
        "Yis Cli is not yet available for this account.",
    )
}

pub fn logged_in_api_key() -> &'static str {
    t("已使用 API Key 登录", "Logged in with API key")
}

pub fn new_session_label() -> &'static str {
    t("新建会话", "New session")
}

pub fn cli_about() -> &'static str {
    t("Yis Cli — 终端 AI 编程助手", "Yis Cli — terminal AI coding assistant")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lang_is_zh() {
        set_lang(Lang::ZhCn);
        assert!(is_zh());
        assert_eq!(t("中文", "English"), "中文");
    }

    #[test]
    fn en_switch() {
        set_lang(Lang::EnUs);
        assert!(!is_zh());
        assert_eq!(t("中文", "English"), "English");
        set_lang(Lang::ZhCn);
    }

    #[test]
    fn hint_label_zh() {
        set_lang(Lang::ZhCn);
        assert_eq!(hint_label("mode"), "模式");
        assert_eq!(hint_label("shortcuts"), "快捷键");
        assert_eq!(press_again_hint("quit"), "再按一次以退出");
        set_lang(Lang::EnUs);
        assert_eq!(hint_label("mode"), "mode");
        assert_eq!(hint_label("shortcuts"), "shortcuts");
        assert_eq!(press_again_hint("quit"), "press again to quit");
        set_lang(Lang::ZhCn);
    }

    #[test]
    fn lang_from_code() {
        assert_eq!(Lang::from_code("zh"), Some(Lang::ZhCn));
        assert_eq!(Lang::from_code("en-US"), Some(Lang::EnUs));
        assert_eq!(Lang::from_code("nope"), None);
    }
}
