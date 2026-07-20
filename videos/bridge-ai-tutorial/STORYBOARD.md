---
format: 1920x1080
message: "Bridge AI 让使用 AI 变得更简单"
arc: "Hook → Problem → Solution → Step 1 → Step 2 → Step 3 → More → Premium → Summary → CTA"
audience: "完全没用过 AI 工具的初学者"
---

## Video direction

**Palette system:** 深色科技底 `#0B0C15`，主文字 `#E8EBF0`，次要文字 `#A0A7B8`，强调色 `#00D4FF`，辅助渐变 `linear-gradient(135deg, #00D4FF 0%, #7C3AED 100%)`。卡片使用 6% 青色透明填充 + 25% 青色边框，无阴影。

**Motion grammar:** 默认 `power3` 长缓出，平滑克制；所有 reveal 跟随旁白节奏，绝不 front-load；保持期间只允许轻微呼吸/微光，禁止无目的漂浮。转场以 `push-slide` 为主，`crossfade` 为辅。

**Held-frame allocation:** Frame 2、Frame 7、Frame 8 为静止停留帧，让观众消化核心信息；其余帧按 VO 分句逐步揭示。

**Negative list:** 禁止卡通插画、emoji、3D 小人、紫蓝渐变装饰球、复杂拟物阴影、无意义的漂浮粒子。

---

## Frame 1 — Hook / Problem

- scene: AI 工具与配置碎片从四周包围观众，制造混乱感
- voiceover: "第一次用 AI 工具，是不是被各种 API Key 和配置搞懵了？模型、密钥、配置文件，每个工具都要单独设置。"
- duration: 10.872s
- transition_in: cut
- status: animated
- src: compositions/frames/01-hook-problem.html
- type: pain_point
- persuasion: 痛点共鸣 + 具象化
- beat: 困惑 + 认同
- blueprint: overwhelm-surround (Adapt)
- focal: 中心的「你」头像 / 问号
- roles: 周围 AI 工具图标 = supporting；配置标签（API Key / 模型 / 密钥）= supporting；中心用户 = foreground subject
- sfx: whoosh-soft

narrativeRole: 通过 surrounds 视觉让观众立刻产生"对，就是我"的认同感，把抽象的"混乱"具象成包围过来的工具和配置。
keyMessage: AI 工具的 API Key、模型、配置让新手感到混乱。

Adapt: 保留「元素从四周收拢压迫」的 signature move；内容替换为 ChatGPT、Claude、Cursor、OpenCode 等工具图标 + "API Key" / "模型" / "配置文件" 标签，背景为深色科技网格。

Scene 1 (0.0–2.5s): 深色网格背景上只出现一句大号文字 "第一次用 AI 工具？"，居中偏上，底部留出呼吸空间。
Scene 2 (2.5–5.5s): 旁白念到 "API Key 和配置" 时，周围 faint 的图标开始浮现并缓慢向内移动；中央出现一个代表用户的简洁头像/问号。
Scene 3 (5.5–8.0s): 旁白列举 "模型、密钥、配置文件" 时，三个标签依次弹出并围绕中心旋转半圈，最后整体微微收紧，制造压迫感后 hold。

---

## Frame 2 — Solution

- scene: Bridge AI 作为中枢，将散落的工具汇聚成环，然后推入品牌 Logo
- voiceover: "Bridge AI 帮你把模型、密钥、应用统一管理。让使用 AI 变得更简单。"
- duration: 7.536s
- transition_in: crossfade
- status: animated
- src: compositions/frames/02-solution.html
- type: product_intro
- persuasion: 转折 + 聚合隐喻
- beat: 清晰 + 期待
- blueprint: constellation-hub (Reproduce)
- focal: 中央的 Bridge AI Logo
- roles: Bridge AI Logo = foreground subject；环绕的工具图标 = supporting；深色网格 + 微光 = background
- sfx: gather-whoosh

narrativeRole: 从上一帧的混乱中解脱，用"中枢聚合"隐喻引出 Bridge AI 的核心价值。
keyMessage: Bridge AI 统一管理模型、密钥和应用，让 AI 使用变简单。

Scene 1 (0.0–2.5s): 延续上一帧的包围状态，但工具图标开始减速；中央升起 Bridge AI Logo（发光描边）。
Scene 2 (2.5–5.5s): 工具图标沿弧线飞向 Logo 并组成环绕轨道，像被收纳进中枢；同时出现一行大字 "统一管理"。
Scene 3 (5.5–8.0s): 镜头缓慢推入 Logo，周围图标淡出，最终留下 "Bridge AI 让使用 AI 变得更简单" 居中静止 hold。

---

## Frame 3 — Step 1: 配置模型

- scene: 模拟 Bridge AI 模型管理界面，光标选择供应商并填写 API Key
- voiceover: "第一步，配置你的 AI 模型。在模型管理页面，选择你想要的供应商。输入模型 ID 和 API Key，点击添加配置。"
- duration: 12s
- transition_in: push-slide RIGHT
- status: animated
- src: compositions/frames/03-step1-config.html
- type: feature_showcase
- persuasion: 逐步演示 + 操作映射
- beat: 理解 + 信心
- blueprint: cursor-ui-demo (Adapt)
- focal: 重建的 Bridge AI 模型管理 UI
- roles: UI 窗口 = foreground subject；光标 = supporting；高亮框 = supporting
- sfx: click-soft, success-chime

narrativeRole: 手把手演示第一步：在模型管理中选择供应商并填写 API Key，让观众理解配置模型的完整路径。
keyMessage: 配置模型只需选择供应商、输入模型 ID 和 API Key。

Adapt: 保留「光标驱动 UI 状态变化」的 signature move；把真实界面抽象成深色卡片式 UI，左侧为已配置模型，右侧为供应商列表。

Scene 1 (0.0–2.0s): 屏幕右侧出现供应商目录（火山引擎、百度千帆、阿里百炼、OpenAI、DeepSeek、Anthropic 等），左侧为空状态 "暂无配置"。
Scene 2 (2.0–5.0s): 光标移动到 DeepSeek 供应商，点击 "+" 展开配置表单；表单字段 "模型 ID"、"Base URL"、"API Key" 依次高亮。
Scene 3 (5.0–9.0s): 光标在 "模型 ID" 输入 deepseek-chat，在 "API Key" 输入掩码字符；"添加配置" 按钮由灰变亮。
Scene 4 (9.0–12.0s): 光标点击按钮，按钮变绿色并出现对勾；左侧新增 DeepSeek 模型卡片，连接状态显示 "已连接"。

---

## Frame 4 — Step 2: 管理应用

- scene: 模拟 Bridge AI 应用管理界面，扫描已安装工具、选择模型并一键启动
- voiceover: "第二步，管理你的 AI 应用。Bridge AI 会自动检测你电脑上已安装的 AI 工具。选择一个已配置好的模型。点击启动应用，配置会自动同步过去。"
- duration: 14.592s
- transition_in: push-slide LEFT
- status: animated
- src: compositions/frames/04-step2-apps.html
- type: feature_showcase
- persuasion: 流程演示 + 自动化暗示
- beat: 便利 + 掌控感
- blueprint: cursor-ui-demo (Adapt)
- focal: 应用管理 UI + 启动流程
- roles: 工具卡片网格 = foreground subject；右侧模型选择面板 = supporting；底部操作日志 = supporting
- sfx: scan-line, click-soft, success-chime

narrativeRole: 演示第二步：从自动检测工具到选择模型，再到一键启动并自动同步配置，打通"模型→应用"链路。
keyMessage: Bridge AI 自动检测工具，选择模型后一键启动即可同步配置。

Adapt: 延续 cursor-ui-demo 的 UI 舞台；新增扫描线和操作日志，把"自动检测"和"配置同步"可视化。

Scene 1 (0.0–2.5s): 应用管理页面进入，顶部标签 "桌面端" 高亮；页面中央显示 "正在扫描..." 扫描线从上至下扫过。
Scene 2 (2.5–6.0s): 扫描结束后，ChatGPT、OpenCode、Claude Desktop、Cursor 等工具卡片依次亮起；未安装项保持 faint。
Scene 3 (6.0–9.0s): 光标选中 OpenCode 卡片，右侧面板出现模型单选列表；光标点击选择 "DeepSeek (deepseek-chat)"。
Scene 4 (9.0–12.0s): 光标移动到下方 "启动应用" 按钮并点击；底部操作日志逐行出现 "准备同步模型配置 → 正在同步配置到 OpenCode... → 配置同步完成"。
Scene 5 (12.0–14.0s): 日志显示 "启动成功"，OpenCode 卡片边框变为绿色对勾，整体 hold。

---

## Frame 5 — Step 3: AI 对话

- scene: 模拟 Bridge AI AI 对话界面，输入问题并展示流式回复
- voiceover: "第三步，直接和 AI 对话。在输入框里输入问题，AI 会实时回复你。支持多轮对话、代码高亮，还能一键复制消息。"
- duration: 11.784s
- transition_in: push-slide RIGHT
- status: animated
- src: compositions/frames/05-step3-chat.html
- type: feature_showcase
- persuasion: 即时反馈 + 功能展示
- beat: 流畅 + 惊喜
- blueprint: cursor-ui-demo (Adapt)
- focal: AI 对话输入框与回复区
- roles: 聊天窗口 = foreground subject；流式文字 = foreground subject；代码块高亮 = supporting
- sfx: message-send, typing-tick

narrativeRole: 演示第三步：直接在 Bridge AI 里聊天，体验流式回复、代码高亮和复制功能。
keyMessage: Bridge AI 内置 AI 对话，支持流式回复、代码高亮和消息复制。

Adapt: 保留光标/输入驱动的 UI 演示；把真实聊天界面抽象为深色对话面板。

Scene 1 (0.0–2.5s): AI 对话界面进入，输入框为空，历史区显示 "Bridge AI 助手" 欢迎语。
Scene 2 (2.5–5.0s): 光标在输入框中输入 "你好，Bridge AI"，点击发送按钮，消息气泡飞出。
Scene 3 (5.0–9.0s): AI 回复逐字打出："你好！我是 Bridge AI，很高兴能帮到你。" 打字机效果；接着出现一段带语法高亮的代码块，复制按钮闪烁一次。
Scene 4 (9.0–12.0s): 输入框继续保留，提示可继续对话；界面轻微 hold，强调"多轮对话"。

---

## Frame 6 — 更多能力 / 技能商店

- scene: 技能商店卡片网格飞入，展示可下载的 AI 技能包
- voiceover: "除了这些基础功能，Bridge AI 还有更多能力。技能商店里可以下载各种 AI 技能包。"
- duration: 8.184s
- transition_in: crossfade
- status: animated
- src: compositions/frames/06-skill-market.html
- type: benefit_highlight
- persuasion: 扩展性展示
- beat: 好奇 + 期待
- blueprint: grid-card-assemble (Reproduce)
- focal: 技能卡片网格
- roles: 技能卡片 = foreground subject；会员标签角标 = supporting；深色网格背景 = background
- sfx: card-pop

narrativeRole: 过渡到高级能力，用卡片网格让观众知道 Bridge AI 可以通过技能商店扩展功能。
keyMessage: Bridge AI 技能商店提供编程、写作、数据分析等 AI 技能包下载。

Scene 1 (0.0–2.5s): 深色背景中央出现文字 "更多能力"，随后淡出。
Scene 2 (2.5–6.5s): 四张技能卡片（编程助手、写作助手、数据分析、AI 绘图）从四周 stagger 飞入，排列成 2×2 网格。
Scene 3 (6.5–9.0s): 每张卡片hover微亮，右下角出现 "下载" 按钮；其中一张卡片显示 "会员专享" 角标。
Scene 4 (9.0–10.0s): 网格 hold，保持可读。

---

## Frame 7 — 会员高级功能

- scene: 高级版功能列表逐项点亮
- voiceover: "会员还能解锁批量配置、本地模型部署等高级功能。"
- duration: 5.352s
- transition_in: push-slide UP
- status: animated
- src: compositions/frames/07-premium.html
- type: benefit_highlight
- persuasion: 价值升级
- beat: 向往 + 掌控
- blueprint: titlecard-reveal (Adapt)
- focal: "高级版" 标题 + 功能列表
- roles: 标题 = foreground subject；功能列表 = supporting；会员徽章 = supporting
- sfx: accent-rise

narrativeRole: 提示进阶路径：会员可解锁批量配置、本地模型部署等效率工具。
keyMessage: 会员可解锁批量配置、本地部署等高级功能。

Adapt: 把 titlecard-reveal 的单一标题扩展为「标题 + 逐项点亮列表」，保持克制。

Scene 1 (0.0–2.5s): 居中显示 "高级版" 标题，下方出现四行 faint 的功能项（批量配置导入导出、一键本地模型部署、自定义 Prompt 模板库、C 盘缓存自动迁移）。
Scene 2 (2.5–7.5s): 旁白每念到一个功能，对应行亮起并带一个青色对勾；其余保持 faint。
Scene 3 (7.5–10.0s): 列表全部亮起，右侧出现 "会员专享" 徽章，整体 hold。

---

## Frame 8 — 总结

- scene: 三步操作流程图标横向排开，下方是 Bridge AI Logo
- voiceover: "配置模型、管理应用、AI 对话，三步就能上手。"
- duration: 5.136s
- transition_in: crossfade
- status: animated
- src: compositions/frames/08-summary.html
- type: branding
- persuasion: 规则三 + 回调
- beat: 清晰 + 满足
- blueprint: grid-card-assemble (Adapt)
- focal: 三个步骤图标
- roles: 步骤卡片 = foreground subject；Bridge AI Logo = supporting；连接箭头 = supporting
- sfx: whoosh-soft

narrativeRole: 用"三步"总结整个教学，让观众记住操作路径，回扣开头的痛点。
keyMessage: 只需配置模型、管理应用、AI 对话三步即可上手 Bridge AI。

Adapt: 把 grid-card-assemble 改为横向三步卡片，强调顺序和简洁。

Scene 1 (0.0–2.5s): 深色背景中央出现 "三步上手" 小标题。
Scene 2 (2.5–6.5s): 三张步骤卡片依次从下方滑入并横向排列：01 配置模型、02 管理应用、03 AI 对话；卡片间有微光箭头连接。
Scene 3 (6.5–9.0s): 步骤卡片依次高亮（青→紫渐变描边），下方浮现 Bridge AI Logo。
Scene 4 (9.0–10.0s): 全部 hold，静止读取。

---

## Frame 9 — CTA

- scene: Bridge AI Logo 凝聚成 CTA 按钮，光标点击
- voiceover: "现在打开 Bridge AI，开启你的 AI 之旅。"
- duration: 3.984s
- transition_in: zoom-through
- status: animated
- src: compositions/frames/09-cta.html
- type: cta
- persuasion: 行动召唤
- beat: 跃跃欲试 + 决心
- blueprint: cta-morph-press (Reproduce)
- focal: "立即开始" CTA 按钮
- roles: CTA 按钮 = foreground subject；Bridge AI Logo = supporting；渐变光晕 = background
- sfx: cta-rise

narrativeRole: 最后引导用户打开 Bridge AI，完成行动召唤。
keyMessage: 立即打开 Bridge AI，开始使用。

Scene 1 (0.0–2.0s): Bridge AI Logo 居中，周围有淡紫色/青色呼吸光晕。
Scene 2 (2.0–4.0s): Logo 收缩并变形为一个圆角药丸按钮 "立即开始"，颜色使用蓝紫渐变。
Scene 3 (4.0–6.0s): 光标从画面外进入，落在按钮上点击，按钮出现涟漪反馈；下方出现 "bridge-ai.com" 或 "打开 Bridge AI"。