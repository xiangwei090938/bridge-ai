# 应用管理页面 — 实施方案

## 1. 当前状态分析
- ToolsView.tsx 已有基础三栏布局：搜索栏 + 分类标签 + 工具卡片网格 + 详情面板
- pi.ts 有完整 Mock 数据
- 	ypes/index.ts 有完整类型定义
- global.css 有对应样式

## 2. 需要改动

### 2.1 类型层 (types/index.ts)
- ToolInfo 接口已经完整覆盖 EchoBird 的字段
- 无需大改，保持现有接口

### 2.2 API 层 (services/api.ts)
- Mock 数据已覆盖所有主流工具
- 需要补充更多 EchoBird 中出现的工具（如 reversi, geminidesktop 等）

### 2.3 UI 层 (routes/ToolsView.tsx)
- 保持现有三栏布局
- 完善工具图标映射（覆盖所有工具）
- 优化详情面板的信息展示
- 改进操作日志的展示样式
- 增加加载状态和空状态提示

### 2.4 样式层 (styles/global.css)
- 保持现有样式，优化卡片交互细节
- 确保与 EchoBird 风格一致

## 3. 数据流
- 前端调用 scanTools() → Mock 返回工具列表
- 选中工具 → 详情面板展示
- 点击同步 → 调用 syncToolConfig() → 日志展示
- 点击安装 → 调用 installTool() → 日志展示
