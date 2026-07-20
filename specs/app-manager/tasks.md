# 应用管理页面 — 任务清单

## [P] Task 1: 完善工具类型定义 (types/index.ts)
- 确保 ToolInfo 接口覆盖 EchoBird 所有字段
- 文件: src/types/index.ts

## [P] Task 2: 扩充 Mock 工具数据 (services/api.ts)
- 补充 EchoBird 风格的工具数据（覆盖所有 27+ 工具）
- 确保每个工具都有 name, category, is_installed, api_protocols 等字段
- 文件: src/services/api.ts

## [P] Task 3: 重写 ToolsView 组件 (routes/ToolsView.tsx)
- 实现三栏布局：搜索/筛选 → 工具网格 → 详情面板
- 完善图标映射系统
- 实现详情面板完整功能（安装状态、路径、API 协议、同步配置、一键安装、操作日志）
- 文件: src/routes/ToolsView.tsx

## [P] Task 4: 完善中文界面 (App.tsx)
- 确保所有菜单项均为中文
- 文件: src/App.tsx

## [P] Task 5: 验证与测试
- 启动开发服务器
- 验证所有功能正常
