import type { ToolInfo, UserModel, ProviderDirectory, SyncResult, LicenseInfo, SkillInfo } from "../types";

let tauriInvoke: any = undefined;
if (typeof window !== "undefined" && (window as any).__TAURI_INTERNALS__) {
  import("@tauri-apps/api/core").then((mod) => { tauriInvoke = mod.invoke; });
}

async function invoke<T>(cmd: string, args?: any): Promise<T> {
  if (tauriInvoke) {
    try { return await tauriInvoke(cmd, args); } catch (e) { console.warn("[Tauri]", cmd, e); }
  }
  return mockInvoke<T>(cmd, args);
}

function mockInvoke<T>(cmd: string, args?: any): T {
  switch (cmd) {
    case "scan_tools": return [
      { id: "chatgptdesktop", name: "ChatGPT", category: "桌面端", is_installed: true, install_path: "C:\\Program Files\\WindowsApps\\OpenAI.Codex_26.715.2305.0_x64__2p2nqsd0c76g0\\app\\ChatGPT.exe", config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: "shell:AppsFolder\OpenAI.Codex_2p2nqsd0c76g0!App" },
      { id: "cursor", name: "Cursor", category: "IDE", is_installed: true, install_path: "C:\Users\Admin\AppData\Local\Programs\cursor\Cursor.exe", config_path: null, api_protocols: ["openai"], has_config: true, sync_supported: true, launch_uri: null },
      { id: "hermes", name: "Hermes", category: "桌面端", is_installed: false, install_path: "C:\Program Files\Hermes\Hermes.exe", config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "opencode", name: "OpenCode", category: "桌面端", is_installed: true, install_path: "C:\\Users\\Administrator\\AppData\\Local\\Programs\\@opencode-aidesktop\\OpenCode.exe", config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "claudedesktop", name: "Claude Desktop", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: ["anthropic"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "claudecode", name: "Claude Code", category: "CLI 命令行", is_installed: true, install_path: "C:\Users\Admin\.local\bin\claude.exe", config_path: null, api_protocols: ["anthropic"], has_config: true, sync_supported: true, launch_uri: null },
      { id: "vscode", name: "VS Code", category: "IDE", is_installed: true, install_path: "C:\Program Files\Microsoft VS Code\Code.exe", config_path: null, api_protocols: [], has_config: false, sync_supported: false, launch_uri: null },
      { id: "geminidesktop", name: "Gemini Desktop", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "aider", name: "Aider", category: "CLI 命令行", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: true, sync_supported: true, launch_uri: null },
      { id: "claudescience", name: "Claude Science", category: "科学", is_installed: false, install_path: null, config_path: null, api_protocols: ["anthropic"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "codex", name: "Codex CLI", category: "CLI 命令行", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: true, sync_supported: true, launch_uri: null },
      { id: "coffeecli", name: "Coffee CLI", category: "CLI 命令行", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "grok", name: "Grok", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "kimicode", name: "Kimi Code", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "mimocode", name: "Mimo Code", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "openclaw", name: "OpenClaw", category: "CLI 命令行", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "opencodedesktop", name: "OpenCode Desktop", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "openscience", name: "OpenScience", category: "科学", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "pi", name: "Pi Assistant", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: [], has_config: false, sync_supported: false, launch_uri: null },
      { id: "qwencode", name: "Qwen Code", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "reversi", name: "Reversi", category: "内置工具", is_installed: true, install_path: null, config_path: null, api_protocols: [], has_config: false, sync_supported: false, launch_uri: null },
      { id: "trae", name: "Trae", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "traecn", name: "Trae CN", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "translator", name: "Translator", category: "内置工具", is_installed: true, install_path: null, config_path: null, api_protocols: [], has_config: false, sync_supported: false, launch_uri: null },
      { id: "vibe-trading", name: "Vibe Trading", category: "内置工具", is_installed: true, install_path: null, config_path: null, api_protocols: [], has_config: false, sync_supported: false, launch_uri: null },
      { id: "workbuddy", name: "WorkBuddy", category: "桌面端", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
      { id: "zcode", name: "Z Code", category: "IDE", is_installed: false, install_path: null, config_path: null, api_protocols: ["openai"], has_config: false, sync_supported: false, launch_uri: null },
    ] as any;
    case "list_providers": return { providers: [
      { name: "OpenAI", url: "https://openai.com", baseUrl: "https://api.openai.com/v1", anthropicUrl: null, modelId: "gpt-4o", modelIds: ["gpt-4o","gpt-4o-mini"], region: "global" },
      { name: "DeepSeek", url: "https://deepseek.com", baseUrl: "https://api.deepseek.com", anthropicUrl: "https://api.deepseek.com/anthropic", modelId: "deepseek-v4-pro", modelIds: ["deepseek-v4-pro","deepseek-v4-flash"], region: "cn" },
      { name: "Ollama", url: "https://ollama.ai", baseUrl: "http://localhost:11434/v1", anthropicUrl: null, modelId: "", modelIds: null, region: "global" },
    ]} as any;
    case "list_models": return [
  { internalId: "openai-gpt4o", name: "OpenAI", modelId: "gpt-4o", baseUrl: "https://api.openai.com/v1", apiKey: "", anthropicUrl: null, type: "openai" },
  { internalId: "openai-gpt4o-mini", name: "OpenAI", modelId: "gpt-4o-mini", baseUrl: "https://api.openai.com/v1", apiKey: "", anthropicUrl: null, type: "openai" },
  { internalId: "deepseek-v4-pro", name: "DeepSeek", modelId: "deepseek-v4-pro", baseUrl: "https://api.deepseek.com", apiKey: "", anthropicUrl: "https://api.deepseek.com/anthropic", type: "openai" },
  { internalId: "deepseek-v4-flash", name: "DeepSeek", modelId: "deepseek-v4-flash", baseUrl: "https://api.deepseek.com", apiKey: "", anthropicUrl: "https://api.deepseek.com/anthropic", type: "openai" },
  { internalId: "ollama-llama3", name: "Ollama", modelId: "llama3", baseUrl: "http://localhost:11434/v1", apiKey: "", anthropicUrl: null, type: "openai" },
] as any;
    case "add_model": return undefined;
    case "delete_model": return undefined;
    case "update_api_key": return undefined;
    case "test_connection": return "Connection OK" as any;
    case "sync_tool_config": return { tool_id: args?.toolId || ", tool_name: args?.toolId || ", success: true, message: "Sync completed (mock)", backed_up: true } as any;
    case "sync_all_tools": return [] as any;
    case "launch_tool": {
  const toolId = args?.toolId || "";
  const toolMap: Record<string, {name:string,path:string,uri:string}> = {
    chatgptdesktop: {name:"ChatGPT",path:"C:\\Program Files\\WindowsApps\\OpenAI.Codex_26.715.2305.0_x64__2p2nqsd0c76g0\\app\\ChatGPT.exe", uri:"shell:AppsFolder\\OpenAI.ChatGPT_2p2nqsd0c76g0!App"},
    opencode: {name:"OpenCode",path:"C:\\Users\\Administrator\\AppData\\Local\\Programs\\@opencode-aidesktop\\OpenCode.exe", uri:""},
    vscode: {name:"VS Code",path:"C:\\Program Files\\Microsoft VS Code\\Code.exe", uri:""},
  };
  const tool = toolMap[toolId];
  if (tool) {
    if (tool.uri) { window.open(tool.uri, "_blank"); return `正在启动 ${tool.name}...` as any; }
    window.open(tool.path, "_blank");
    return `正在从 ${tool.path} 启动 ${tool.name}...` as any;
  }
  return `未找到 ${toolId} 的安装信息，请在桌面 Tauri 模式下启动` as any;
}
    case "install_tool": return "Install started (mock)" as any;
    case "check_premium_status": return { is_premium: false, activated_at: null, expires_at: null, days_remaining: 0 } as any;
    case "activate_premium": return { is_premium: true, activated_at: new Date().toISOString(), expires_at: new Date(Date.now()+365*86400000).toISOString(), days_remaining: 365 } as any;
    case "start_premium_trial": return { is_premium: true, activated_at: new Date().toISOString(), expires_at: new Date(Date.now()+7*86400000).toISOString(), days_remaining: 7 } as any;
    case "list_available_skills": return [
      { id: "writing-assistant", name: "Writing Assistant", description: "AI writing assistant for polishing, summarizing, translating", author: "Bridge AI", version: "1.0.0", size_bytes: 512000, downloads: 1234, category: "Writing", tags: ["writing","polish","translate"], requires_premium: true, icon: String.fromCharCode(128221) },
      { id: "code-reviewer", name: "Code Reviewer", description: "Automated code review, bug detection, optimization suggestions", author: "Bridge AI", version: "1.0.0", size_bytes: 380000, downloads: 892, category: "Programming", tags: ["code","review","bugs"], requires_premium: true, icon: String.fromCharCode(128187) },
      { id: "translator-pro", name: "Pro Translator", description: "Multi-language translation supporting technical docs", author: "Bridge AI", version: "1.2.0", size_bytes: 280000, downloads: 1567, category: "Translation", tags: ["translate","multi-language"], requires_premium: true, icon: String.fromCharCode(127760) },
      { id: "study-assistant", name: "Study Assistant", description: "Knowledge summaries, problem solving, study plans", author: "Bridge AI", version: "1.0.0", size_bytes: 420000, downloads: 2103, category: "Learning", tags: ["study","education"], requires_premium: true, icon: String.fromCharCode(128218) },
      { id: "data-analyzer", name: "Data Analyzer", description: "Data visualization, trend analysis, report generation", author: "Bridge AI", version: "1.0.0", size_bytes: 650000, downloads: 678, category: "Tools", tags: ["data","analysis"], requires_premium: true, icon: String.fromCharCode(128202) },
      { id: "ai-chat-enhancer", name: "AI Chat Enhancer", description: "Enhanced chat with role-play, sentiment analysis", author: "Bridge AI", version: "2.0.0", size_bytes: 340000, downloads: 3210, category: "Tools", tags: ["chat","roleplay"], requires_premium: false, icon: String.fromCharCode(128172) },
      { id: "image-prompt", name: "Image Prompt", description: "Generate high-quality AI image prompts", author: "Bridge AI", version: "1.0.0", size_bytes: 190000, downloads: 456, category: "Creative", tags: ["ai-art","prompt"], requires_premium: true, icon: String.fromCharCode(127912) },
    ] as any;
    case "install_skill": return { id: args?.skillId, skill_id: args?.skillId, name: args?.skillId || "", version: "1.0.0", is_enabled: true, install_path: args?.skillId || "", installed_at: new Date().toISOString() } as any;
    case "list_installed_skills": return [] as any;
    case "uninstall_skill": return undefined;
    case "get_setting": return null as any;
    case "set_setting": return undefined;
    default: console.warn("[Mock] Unknown command:", cmd, args); return undefined as any;
  }
}

export const scanTools = () => invoke<ToolInfo[]>("scan_tools");
export const listProviders = () => invoke<ProviderDirectory>("list_providers");
export const listModels = () => invoke<UserModel[]>("list_models");
export const addModel = (name: string, modelId: string, baseUrl: string, apiKey: string, anthropicUrl: string|null, modelType: string) =>
  invoke<void>("add_model", { name, modelId, baseUrl, apiKey, anthropicUrl, modelType });
export const deleteModel = (internalId: string) => invoke<void>("delete_model", { internalId });
export const updateApiKey = (internalId: string, apiKey: string) => invoke<void>("update_api_key", { internalId, apiKey });
export const testConnection = (providerName: string, baseUrl: string, apiKey: string) => invoke<string>("test_connection", { providerName, baseUrl, apiKey });
export const syncToolConfig = (toolId: string, modelInternalId: string) => invoke<SyncResult>("sync_tool_config", { toolId, modelInternalId });
export const syncAllTools = (modelInternalId: string) => invoke<SyncResult[]>("sync_all_tools", { modelInternalId });
export const launchTool = (toolId: string) => invoke<string>("launch_tool", { toolId });
export const installTool = (toolName: string) => invoke<string>("install_tool", { toolName });
export const sendMessage = (conversationId: string, content: string, modelId: string, providerId: string) => invoke<string>("send_message", { conversationId, content, modelId, providerId });
export const listConversations = () => invoke<any[]>("list_conversations");
export const getMessages = (conversationId: string) => invoke<any[]>("get_messages", { conversationId });
export const getSetting = (key: string) => invoke<string|null>("get_setting", { key });
export const setSetting = (key: string, value: string) => invoke<void>("set_setting", { key, value });
export const checkPremiumStatus = () => invoke<LicenseInfo>("check_premium_status");
export const activatePremium = (licenseKey: string) => invoke<LicenseInfo>("activate_premium", { licenseKey });
export const startPremiumTrial = () => invoke<LicenseInfo>("start_premium_trial");
export const listAvailableSkills = () => invoke<any[]>("list_available_skills");
export const installSkill = (skillId: string) => invoke<any>("install_skill", { skillId });
export const listInstalledSkills = () => invoke<any[]>("list_installed_skills");
export const uninstallSkill = (skillId: string) => invoke<void>("uninstall_skill", { skillId });
export const webSearch = (query: string, apiKey: string) => invoke<any[]>("web_search", { query, apiKey });
export const webSearchContext = (query: string, apiKey: string) => invoke<string>("web_search_context", { query, apiKey });
export const checkOllama = () => invoke<boolean>("check_ollama");
export const listModelsCatalog = () => invoke<any[]>("list_models_catalog");
export const pullLocalModel = (modelName: string) => invoke<string>("pull_local_model", { modelName });
export const deleteLocalModel = (modelName: string) => invoke<string>("delete_local_model", { modelName });
export const aiDraw = (prompt: string, apiKey: string, model: string) => invoke<any>("ai_draw", { prompt, apiKey, model });




