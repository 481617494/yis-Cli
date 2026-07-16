//! 快捷键弹窗（速查表）中文文案。
//!
//! 英文 `ActionDef` 标识保持不变；仅在渲染/构建弹窗条目时本地化。

use crate::actions::Category;
use crate::i18n::{is_zh, t};

pub fn shortcuts_title() -> &'static str {
    t("键盘快捷键", "Keyboard Shortcuts")
}

pub fn category_label(cat: Category) -> &'static str {
    match cat {
        Category::GettingStarted => t("基础", "Essentials"),
        Category::Input => t("输入", "Input"),
        Category::ConversationNav => t("会话导航", "Conversation Navigation"),
        Category::ConversationAction => t("会话操作", "Conversation Actions"),
        Category::Panels => t("面板", "Panels"),
        Category::Session => t("会话", "Session"),
        Category::Dashboard => t("控制台", "Dashboard"),
    }
}

pub fn dimmed_context_note() -> &'static str {
    t("（当前上下文不可用）", "(not active in current context)")
}

pub fn search_scrollback_desc() -> &'static str {
    t("搜索回看内容", "Search scrollback")
}

pub fn paste_desc() -> &'static str {
    t("从剪贴板粘贴图片（与文本）", "Paste images (and text) from the clipboard")
}

pub fn paste_long_help() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        t(
            "将剪贴板图片以芯片形式粘贴到输入框，纯文本则按键入处理。\n截图、浏览器「复制图片」与文件管理器复制的图片请用 Ctrl+V（许多终端会吞掉 Cmd+V，TUI 收不到）。\n也可将图片文件拖入输入框。",
            "Pastes clipboard images into the prompt as chips, and plain text as typed.\nUse Ctrl+V for screenshots, browser \"Copy Image\", and file-manager image copies (many terminals swallow Cmd+V and never deliver it to the TUI).\nYou can also drag an image file into the prompt.",
        )
    }
    #[cfg(target_os = "windows")]
    {
        t(
            "将剪贴板图片以芯片形式粘贴到输入框，纯文本则按键入处理。\n优先 Ctrl+V；失败时可用 Alt+V（部分终端或配置会丢弃图片剪贴板；旧版 Windows Terminal 可能只粘贴文本）。\n也可从资源管理器拖入图片文件。",
            "Pastes clipboard images into the prompt as chips, and plain text as typed.\nPrefer Ctrl+V. Use Alt+V as a fallback when Ctrl+V fails (some terminals or configs drop image clipboards; older Windows Terminal versions only pasted text).\nYou can also drag an image file from Explorer into the prompt.",
        )
    }
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        t(
            "将剪贴板图片以芯片形式粘贴到输入框，纯文本则按键入处理。\n截图、浏览器「复制图片」与文件管理器复制请用 Ctrl+V。\n也可将图片文件拖入输入框。",
            "Pastes clipboard images into the prompt as chips, and plain text as typed.\nUse Ctrl+V for screenshots, browser \"Copy Image\", and file-manager image copies.\nYou can also drag an image file into the prompt.",
        )
    }
}

/// 本地化动作短描述（列表行主文案）。
pub fn description(en: &str) -> &str {
    if !is_zh() {
        return en;
    }
    match en {
        "Back to dashboard" => "返回面板",
        "Cancel turn" => "取消当前回合",
        "Change working directory for new agents" => "更改新代理的工作目录",
        "Close dashboard" => "关闭面板",
        "Collapse selected entry" => "折叠选中项",
        "Command palette" => "命令面板",
        "Copy command / path" => "复制命令/路径",
        "Copy content" => "复制内容",
        "Cycle dispatch mode" => "循环调度模式",
        "Cycle mode (Normal / Plan / Always-approve)" => "循环模式（普通 / 计划 / 始终批准）",
        "Expand / collapse" => "展开/折叠",
        "Expand all / collapse all" => "全部展开/全部折叠",
        "Expand selected entry" => "展开选中项",
        "Focus prompt" => "聚焦输入框",
        "Focus scrollback" => "聚焦回看区",
        "Go to bottom" => "跳到底部",
        "Go to top" => "跳到顶部",
        "Keyboard shortcuts" => "键盘快捷键",
        "Kill background task" => "终止后台任务",
        "New session" => "新建会话",
        "Next link" => "下一个链接",
        "Next response" => "下一条回复",
        "Next session" => "下一会话",
        "Next turn" => "下一回合",
        "Open extensions" => "打开扩展",
        "Open in viewer" => "在查看器中打开",
        "Open sessions" => "打开会话列表",
        "Open the Agent Dashboard" => "打开代理面板",
        "Pin / unpin agent" => "置顶/取消置顶代理",
        "Previous link" => "上一个链接",
        "Previous response" => "上一条回复",
        "Previous session" => "上一会话",
        "Previous turn" => "上一回合",
        "Quit" => "退出",
        "Rename agent" => "重命名代理",
        "Reorder agent down" => "下移代理",
        "Reorder agent up" => "上移代理",
        "Rewind to selected turn" => "回退到选中回合",
        "Scroll down half page" => "向下滚半页",
        "Scroll down one line" => "向下滚一行",
        "Scroll down one page" => "向下滚一页",
        "Scroll up half page" => "向上滚半页",
        "Scroll up one line" => "向上滚一行",
        "Scroll up one page" => "向上滚一页",
        "Select next entry" => "选择下一项",
        "Select next row" => "选择下一行",
        "Select previous entry" => "选择上一项",
        "Select previous row" => "选择上一行",
        "Send" => "发送",
        "Send now while running (cancels the current turn)" => "运行中立即发送（会取消当前回合）",
        "Send running task to background" => "将运行中任务转到后台",
        "Shell mode (type ! on empty prompt)" => "Shell 模式（空输入时键入 !）",
        "Show shortcuts overlay" => "显示快捷键浮层",
        "Start voice dictation (Ctrl+Space / F8)" => "开始语音听写（Ctrl+Space / F8）",
        "Stop / Close agent" => "停止/关闭代理",
        "Stop agent, close session (back to dashboard)" => "停止代理并关闭会话（返回面板）",
        "Toggle all thinking blocks" => "切换全部思考块",
        "Toggle always-approve" => "切换始终批准",
        "Toggle mouse reporting (native copy/paste)" => "切换鼠标报告（原生复制/粘贴）",
        "Toggle multiline" => "切换多行输入",
        "Toggle prompt queue" => "切换提示队列",
        "Toggle raw markdown" => "切换原始 Markdown",
        "Toggle row grouping" => "切换行分组",
        "Toggle tasks pane" => "切换任务面板",
        "Toggle todo pane" => "切换待办面板",
        "Toggle worktree mode for new agents" => "切换新代理的工作树模式",
        "Voice dictation (Ctrl+Space / F8)" => "语音听写（Ctrl+Space / F8）",
        "切换模型" => "切换模型",
        "打开设置" => "打开设置",
        other => other,
    }
}

/// 本地化动作长说明（详情页 / 行内展开）。
pub fn long_help(en: &str) -> &str {
    if !is_zh() {
        return en;
    }
    match en {
        "Folds or unfolds the selected scrollback entry to hide or show its full body.\nHandy for skimming long tool output or reasoning.\nRelated: E folds/unfolds every entry, Ctrl+E toggles all thinking blocks." => "折叠或展开当前选中的回看条目，隐藏或显示完整内容。\n适合快速浏览较长的工具输出或推理过程。\n相关：E 折叠/展开全部条目，Ctrl+E 切换全部思考块。",
        "Folds or unfolds every scrollback entry at once, unlike e which toggles only the selected row.\nCollapse a long transcript to scan headers, then expand it all back.\nThinking blocks have their own toggle, Ctrl+E." => "一次折叠或展开全部回看条目（e 只切换当前选中行）。\n可先折叠长会话只看标题，再全部展开。\n思考块有独立开关：Ctrl+E。",
        "Shows or hides the agent's reasoning (thinking) blocks across the whole transcript in one keypress.\nReveal how the agent reached an answer, or hide reasoning to focus on results.\nSeparate from E, which folds every entry regardless of type." => "一键显示或隐藏整段会话中的代理思考（reasoning）块。\n可查看推理过程，或隐藏思考以聚焦结果。\n与 E 不同：E 会折叠所有类型的条目。",
        "Switches the selected entry between rendered markdown and its raw source text.\nUse it to copy exact markdown, inspect a link target, or see formatting the renderer hides.\nPress again to return to the rendered view." => "在渲染后的 Markdown 与原始源文本之间切换当前条目。\n便于复制原始 Markdown、检查链接目标，或查看渲染隐藏的格式。\n再按一次回到渲染视图。",
        "Copies the selected block's body to the clipboard: message text, full tool output, or a code block's contents.\nOffered only on blocks that support copy.\nFor just the command or file path, use Y instead." => "将选中块的正文复制到剪贴板：消息文本、完整工具输出或代码块内容。\n仅对支持复制的块可用。\n若只要命令或文件路径，请用 Y。",
        "Copies only the block's identifier: a tool call's command line or a file block's path, not the body.\nHandy to re-run a command or paste a path elsewhere.\nUse lowercase y to copy the full content instead." => "只复制块的标识：工具调用的命令行或文件块路径，不含正文。\n便于重跑命令或在别处粘贴路径。\n复制完整内容请用小写 y。",
        "Opens the selected block in a focused, scrollable full-screen viewer.\nBest for long tool output, large files, or code you want to read away from the surrounding transcript.\nEsc returns to the conversation." => "在可滚动的全屏查看器中打开选中块。\n适合阅读较长工具输出、大文件或需要脱离会话上下文的代码。\n按 Esc 返回对话。",
        "Rewinds the conversation to an earlier turn, restoring the file snapshot taken then and discarding later changes.\nPick a turn from the list and choose what to restore (everything, conversation only, or files only); a running turn is offered for cancel first, and any conflicts or errors are reported after it runs.\nDestructive: later turns are dropped.\nAlso reachable idle with an empty prompt via Esc Esc (within 800ms), same as `/rewind`." => "将对话回退到更早的回合，恢复当时的文件快照并丢弃之后的更改。\n从列表选择回合并选择恢复范围（全部、仅对话或仅文件）；若有进行中的回合会先提示取消，冲突或错误会在执行后报告。\n破坏性操作：之后的回合会被丢弃。\n空闲且输入框为空时也可 Esc Esc（800ms 内）触发，等同 `/rewind`。",
        "Terminates the background task owned by the selected task block (e.g. a long shell command sent to the background).\nReach for it to stop a runaway or no-longer-needed process.\nApplies only to a live task; finished ones are unaffected." => "终止选中任务块对应的后台任务（例如已转到后台的长时间 shell 命令）。\n用于停止失控或不需要的进程。\n仅对进行中的任务有效；已结束的不受影响。",
        "Moves focus from the prompt to the scrollback so you can navigate the transcript.\nTab works in both simple and vim scrollback modes.\nEsc is reserved for clear / rewind (idle) policy, not focus." => "将焦点从输入框移到回看区，以便浏览会话记录。\n在简单与 vim 回看模式下 Tab 均可用。\nEsc 保留给清空/回退（空闲）策略，不用于切换焦点。",
        "Interrupts the agent's current turn and stops generation, keeping the session open.\nCtrl+C cancels when the prompt is empty; with a non-empty draft it clears the prompt first and leaves the turn running.\nIt stops the turn, not the app; use the quit shortcut to exit." => "中断代理当前回合并停止生成，会话保持打开。\n输入框为空时 Ctrl+C 取消回合；有草稿时先清空输入，回合继续运行。\n只停止回合不退出应用；退出请用退出快捷键。",
        "Steps the session mode: Normal -> Plan -> Always-Approve -> Normal.\nPlan keeps the agent planning first and writes no files; Always-Approve runs every tool call without asking.\nCtrl+O toggles auto-approve directly." => "循环会话模式：普通 → 计划 → 始终批准 → 普通。\n计划模式先规划且不写文件；始终批准会在不询问的情况下执行每个工具调用。\nCtrl+O 可直接切换自动批准。",
        "Shows or hides the todo pane: the agent's live task checklist for the current work.\nWatch what it plans to do and what's left as the turn runs.\nA side pane; toggle it off to reclaim width." => "显示或隐藏待办面板：代理当前工作的实时任务清单。\n可查看计划事项与剩余工作。\n侧边面板；关闭可腾出宽度。",
        "Shows or hides the tasks pane, which lists background tasks and their status.\nUse it to monitor or return to work you sent to the background with Ctrl+G.\nA side pane; toggle off to reclaim width." => "显示或隐藏任务面板，列出后台任务及其状态。\n用于监控或回到用 Ctrl+G 发到后台的工作。\n侧边面板；关闭可腾出宽度。",
        "Shows or hides the prompt queue.\nThe queue lets you line up follow-up prompts while a turn is running; each is sent automatically when the agent finishes.\nLocal macOS VS Code family: Ctrl+4 primary (Ctrl+; / Ctrl+' alts). Otherwise Ctrl+; with Ctrl+' alt." => "显示或隐藏提示队列。\n可在回合运行时排队后续提示，代理完成后会自动发送。\n本机 macOS VS Code 系列：主键 Ctrl+4（Ctrl+; / Ctrl+' 为备选）。其它环境主键 Ctrl+;，备选 Ctrl+'。",
        "Opens the session browser to resume or switch between past conversations.\nSelect one to reattach to its full history.\nSeparate from the Agent Dashboard (Ctrl+\\), which manages many live agents at once." => "打开会话浏览器，恢复或切换历史对话。\n选择一项即可重新接入完整历史。\n不同于代理面板（Ctrl+\\），后者用于管理多个在线代理。",
        "Opens the extensions manager for MCP servers and plugins: see what's connected and the tools they add.\nUse it to confirm an integration loaded or browse available tools.\nDistinct from settings, which holds general app options." => "打开扩展管理器（MCP 服务器与插件）：查看已连接项及其工具。\n用于确认集成是否加载，或浏览可用工具。\n与设置不同，设置面向通用应用选项。",
        "Detaches the running turn so it keeps working in the background while you read, queue prompts, or start something else.\nTrack and resume it from the tasks pane (Ctrl+B).\nOnly meaningful while a turn is actually running." => "将运行中的回合转到后台继续执行，便于阅读、排队提示或开始其它工作。\n可在任务面板（Ctrl+B）跟踪与恢复。\n仅在回合确实运行中时有意义。",
        "Sends a message to the agent mid-turn without cancelling it (interject), so you can steer or add context while it keeps working.\nPlain Enter while a turn is running queues a follow-up for later; this chord merges composer text into the current turn instead.\nWith an empty composer, bare Enter (or this chord) force-sends the top queued follow-up from the prompt — no need to focus the queue pane. On the queue pane, this chord force-sends the selected row.\nReach for it to correct course without losing the turn's progress." => "在回合进行中向代理发消息且不取消当前回合（插话），便于引导或补充上下文。\n运行中按 Enter 会排队后续消息；此快捷键改为将输入内容并入当前回合。\n输入框为空时，单独 Enter（或此快捷键）会强制发送队列顶部的后续提示，无需聚焦队列面板。在队列面板上，此快捷键强制发送选中行。\n用于纠偏而不丢失当前回合进度。",
        "Microphone capture for dictation, bound to Ctrl+Space (or F8 — handy where Ctrl+Space is taken, e.g. macOS input-source switching; use Fn+F8 on a laptop).\nBehavior follows the Voice capture setting: toggle (press to start, press again to stop) or hold-to-talk (hold to record, release to stop), where hold needs a Kitty-protocol terminal and falls back to toggle elsewhere. `/voice` toggles everywhere.\nSpeech is transcribed straight into the prompt." => "麦克风听写采集，绑定 Ctrl+Space（或 F8——当 Ctrl+Space 被占用时更方便，例如 macOS 输入法切换；笔记本可用 Fn+F8）。\n行为遵循语音采集设置：切换（按一下开始，再按停止）或按住说话（按住录音，松开停止）；按住模式需要 Kitty 协议终端，其它环境回退为切换。`/voice` 在各处切换语音。\n语音会直接转写到输入框。",
        "Toggles a persistent multi-line prompt so the editor stays expanded for composing longer messages.\nInsert newlines with Shift+Enter or Alt+Enter (or a trailing backslash); bare Enter still sends.\nCtrl+M toggles multiline in the prompt; off the prompt it opens the model picker." => "切换持久多行输入，便于撰写较长消息。\n用 Shift+Enter 或 Alt+Enter（或行尾反斜杠）换行；单独 Enter 仍为发送。\n输入框内 Ctrl+M 切换多行；离开输入框时打开模型选择器。",
        "Runs a shell command without leaving the chat: type ! at the start of an empty prompt, then the command.\nThe command output is captured into the scrollback.\nDelete the leading ! to go back to a normal prompt." => "不离开聊天即可运行 shell 命令：在空输入框开头输入 !，再跟命令。\n命令输出会写入回看区。\n删除开头的 ! 即可回到普通输入。",
        "Turns auto-approve (YOLO) on or off for this session.\nWhile on, the agent runs every tool call (edits, shell, deletes) with no per-action confirmation.\nSame state as the Shift+Tab cycle's Always-Approve; use with care." => "为本会话打开或关闭自动批准（YOLO）。\n开启后，代理执行每个工具调用（编辑、shell、删除）都不再逐项确认。\n与 Shift+Tab 循环中的「始终批准」状态相同；请谨慎使用。",
        "Starts a fresh session with empty scrollback and context.\nRequires confirmation: press it twice (the first press arms, the second starts)\nso you don't discard the current conversation by accident." => "开启空白回看与上下文的全新会话。\n需要确认：连按两次（第一次预备，第二次开始），\n避免误丢当前对话。",
        "Exits the app. Requires confirmation: press twice in quick succession;\na lone press is treated as a stray key and ignored.\nBound to Ctrl+Q, with Ctrl+D as an alias (Ctrl+D is primary in VS Code's terminal)." => "退出应用。需要确认：快速连按两次；\n单独一次按键会被忽略。\n绑定 Ctrl+Q，Ctrl+D 为别名（在 VS Code 终端中 Ctrl+D 更常用）。",
        "Fuzzy-search every action and slash command, then run it by name.\nUseful when you don't remember a key binding.\nAlso opens with ? while the scrollback is focused." => "模糊搜索全部操作与斜杠命令，按名称运行。\n记不住快捷键时很有用。\n回看区聚焦时按 ? 也可打开。",
        "Opens this keyboard cheatsheet.\nBrowse with j/k, expand a row's inline help with e, or press Enter for a shortcut's full detail page.\nBound to both Ctrl+. and Ctrl+X; the bar advertises whichever your terminal sends reliably." => "打开本键盘速查表。\n用 j/k 浏览，e 展开行内帮助，Enter 打开某快捷键的详情页。\n绑定 Ctrl+. 与 Ctrl+X；底部栏会显示你终端能可靠送达的那个。",
        "打开模型选择器，切换当前会话使用的模型（后续对话生效）。\n绑定 Ctrl+M；在输入框聚焦时该快捷键改为切换多行输入。\n也可在滚动区、命令面板，或输入 /model 打开。" => "打开模型选择器，切换当前会话使用的模型（后续对话生效）。\n绑定 Ctrl+M；在输入框聚焦时该快捷键改为切换多行输入。\n也可在滚动区、命令面板，或输入 /model 打开。",
        "Opens the Agent Dashboard: a list of all your running and recent agents to monitor and switch between.\nWorks from anywhere, including the welcome screen and inside a session.\nFrom there you can dispatch, attach, stop, group, and reorder agents." => "打开代理面板：列出全部运行中与最近的代理，便于监控与切换。\n可从欢迎页或会话内任意位置打开。\n可在此派发、附着、停止、分组与重排代理。",
        "Pins or unpins the selected agent so it stays at the top of the list regardless of sorting or grouping.\nKeep the agents you care about in view as others come and go.\nPins persist across dashboard sessions." => "置顶或取消置顶选中代理，使其在排序/分组变化时仍保持在列表顶部。\n便于始终看到关心的代理。\n置顶在面板会话间保持。",
        "Stops the selected agent and removes its row from the dashboard; a running turn is interrupted first.\nUse it to clear finished or unwanted agents without attaching to them.\nThe in-overlay equivalent (Ctrl+X) confirms before stopping." => "停止选中代理并从面板移除其行；若回合进行中会先中断。\n用于清理已完成或不需要的代理，无需进入该代理。\n浮层中的等价操作（Ctrl+X）停止前会要求确认。",
        "Cycles the dispatch mode for agents you launch from the dashboard: Normal, Plan, then Always-Approve.\nPlan has new agents plan before changing files; Always-Approve runs their tools without prompting.\nMirrors the in-session Shift+Tab cycle, applied to new dispatches." => "循环从面板启动代理时的调度模式：普通、计划、始终批准。\n计划模式先规划再改文件；始终批准不经确认即运行工具。\n与会话内 Shift+Tab 循环对应，作用于新派发。",
        "Switches the dashboard between a flat list and rows grouped by state, such as working versus idle.\nGrouping surfaces the agents that need attention; the flat list keeps a stable order.\nYour choice persists across sessions." => "在扁平列表与按状态分组（如工作中/空闲）之间切换面板。\n分组便于发现需要关注的代理；扁平列表顺序更稳定。\n选择会跨会话保留。",
        "Closes the dashboard and returns to where you were.\nEsc is a cascade: it first dismisses an open peek or clears an active filter, and only exits once nothing else is pending.\nRebind this action to a different key to exit directly." => "关闭面板并返回原处。\nEsc 为级联：先关闭预览或清除筛选，没有其它待处理项时才退出。\n可将此操作绑定到其它键以实现一键退出。",
        "Toggles auto-approve (YOLO) for the selected agent right from the dashboard, without attaching to it.\nWhile on, that agent runs every tool call with no per-action confirmation.\nThe per-session equivalent is Ctrl+O inside a session." => "在面板上直接为选中代理切换自动批准（YOLO），无需进入该代理。\n开启后该代理的每个工具调用都不再确认。\n会话内等价操作为 Ctrl+O。",
        "Opens a picker to set the working directory that newly dispatched dashboard agents run in.\nLaunch agents against a different repo or folder without leaving the dashboard.\nAffects new dispatches only, not agents already running." => "打开选择器，设置从面板新派发代理的工作目录。\n无需离开面板即可在不同仓库或目录启动代理。\n仅影响新派发；已运行代理不受影响。",
        "Arms the next dashboard-dispatched agent to spawn in a fresh git worktree, isolating its work on a separate checkout.\nOnly applies when the working directory is a git repo.\nAffects newly dispatched agents, not ones already running." => "使下一次从面板派发的代理在全新 git worktree 中启动，隔离到独立检出。\n仅当工作目录为 git 仓库时生效。\n只影响新派发的代理，已运行的不受影响。",
        "Leaves the attached session overlay and returns to the dashboard list, without stopping the agent.\nAlso reachable via q on the scrollback, a neutral Esc, or the close button.\nTo stop the agent instead of just detaching, use Ctrl+X." => "离开已附着的会话浮层并返回面板列表，不停止代理。\n也可在回看区按 q、中性 Esc 或关闭按钮。\n若要停止代理而非仅分离，请用 Ctrl+X。",
        "Inside a session overlay, stops the attached agent and closes it, returning you to the dashboard list.\nRequires confirmation: press Ctrl+X twice.\nCtrl+. still opens the cheatsheet here; only Ctrl+X is taken over by stop." => "在会话浮层中停止已附着代理并关闭，返回面板列表。\n需要确认：连按两次 Ctrl+X。\n此处 Ctrl+. 仍打开速查表；只有 Ctrl+X 被停止操作占用。",
        other => other,
    }
}

// ── 底部提示条 ──────────────────────────────────────────

pub fn footer_nav() -> &'static str { t("↑/↓ 导航", "↑/↓ nav") }
pub fn footer_filter(active: bool) -> &'static str {
    if active {
        t("f 显示全部", "f show all")
    } else {
        t("f 筛选", "f filter")
    }
}
pub fn footer_expand() -> &'static str { t("e/Space/→ 展开", "e/Space/→ expand") }
pub fn footer_collapse() -> &'static str { t("← 折叠", "← collapse") }
pub fn footer_details() -> &'static str { t("Enter 详情", "Enter details") }
pub fn footer_search() -> &'static str { t("/ 搜索", "/ search") }
pub fn footer_close() -> &'static str { t("Esc 关闭", "Esc close") }
pub fn footer_back() -> &'static str { t("Esc 返回", "Esc back") }
pub fn footer_scroll() -> &'static str { t("↑/↓ 滚动", "↑/↓ scroll") }
pub fn footer_close_chord() -> &'static str {
    if cfg!(target_os = "macos") {
        t("⌃+./X 关闭", "⌃+./X close")
    } else {
        t("Ctrl+./X 关闭", "Ctrl+./X close")
    }
}
