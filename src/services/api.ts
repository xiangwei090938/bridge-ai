import { ToolInfo, UserModel, ProviderDirectory, SyncResult, LicenseInfo, SkillInfo } from "../types";

let tauriInvoke: any = undefined;

// 检测 Tauri 环境
function detectTauri(): boolean {
  if (typeof window === "undefined") return false;
  
  // Tauri 2.x
  if ((window as any).__TAURI__) {
    tauriInvoke = (window as any).__TAURI__.invoke;
    return true;
  }
  
  // Tauri 1.x fallback
  if ((window as any).__TAURI_INTERNALS__) {
    tauriInvoke = (window as any).__TAURI_INTERNALS__.invoke;
    return true;
  }
  
  return false;
}

// 初始化时检测
const isTauri = detectTauri();

// 如果不是 Tauri 环境，尝试动态导入（开发模式）
if (!isTauri && typeof window !== "undefined") {
  import("@tauri-apps/api/core").then((mod) => {
    if (mod.invoke) {
      tauriInvoke = mod.invoke;
      console.log("[Bridge AI] Tauri invoke loaded via dynamic import");
    }
  }).catch(() => {
    console.warn("[Bridge AI] Tauri not available");
  });
}

// Timeout wrapper
function withTimeout<T>(promise: Promise<T>, ms: number): Promise<T> {
  return Promise.race([
    promise,
    new Promise<T>((_, reject) =>
      setTimeout(() => reject(new Error("Tauri invoke timeout after " + ms + "ms")), ms)
    ),
  ]);
}

/**
 * 核心调用函数 - 强制使用真实后端
 * 如果 Tauri 后端未连接，直接抛出错误
 */
async function invoke<T>(cmd: string, args?: any): Promise<T> {
  if (!tauriInvoke) {
    throw new Error(
      "[Bridge AI] Tauri 后端未连接。请使用桌面模式运行：npm run tauri:dev"
    );
  }
  
  try {
    return await withTimeout(tauriInvoke(cmd, args), 10000);
  } catch (e: any) {
    console.error("[Bridge AI] Command failed:", cmd, e);
    throw new Error(`[Bridge AI] 执行失败：${e.message || e}`);
  }
}

// ============ 公开 API ============

export async function scanTools(): Promise<ToolInfo[]> {
  return invoke<ToolInfo[]>("scan_tools");
}

export async function listModels(): Promise<UserModel[]> {
  return invoke<UserModel[]>("list_models");
}

export async function listProviders(): Promise<ProviderDirectory> {
  return invoke<ProviderDirectory>("list_providers");
}

export async function addModel(model: Partial<UserModel>): Promise<void> {
  return invoke<void>("add_model", {
    name: model.name || "",
    modelId: model.modelId || "",
    baseUrl: model.baseUrl || "",
    apiKey: model.apiKey || "",
    anthropicUrl: model.anthropicUrl || null,
    modelType: model.type || "openai",
  });
}

export async function deleteModel(internalId: string): Promise<void> {
  return invoke<void>("delete_model", { internalId: internalId });
}

export async function updateApiKey(internalId: string, apiKey: string): Promise<void> {
  return invoke<void>("update_api_key", { internalId: internalId, apiKey: apiKey });
}

export async function testConnection(providerName: string, baseUrl: string, apiKey: string): Promise<string> {
  return invoke<string>("test_connection", { 
    providerName: providerName, 
    baseUrl: baseUrl, 
    apiKey: apiKey 
  });
}

export async function syncToolConfig(toolId: string, modelInternalId: string): Promise<SyncResult> {
  return invoke<SyncResult>("sync_tool_config", { 
    toolId: toolId, 
    modelInternalId: modelInternalId 
  });
}

export async function syncAllTools(modelInternalId: string): Promise<SyncResult[]> {
  return invoke<SyncResult[]>("sync_all_tools", { modelInternalId: modelInternalId });
}

export async function launchTool(toolId: string): Promise<string> {
  return invoke<string>("launch_tool", { toolId: toolId });
}

export async function installTool(toolId: string): Promise<string> {
  return invoke<string>("install_tool", { toolId: toolId });
}

export async function checkPremiumStatus(): Promise<LicenseInfo> {
  return invoke<LicenseInfo>("check_premium_status");
}

export async function activatePremium(code: string): Promise<LicenseInfo> {
  return invoke<LicenseInfo>("activate_premium", { code });
}

export async function startPremiumTrial(): Promise<LicenseInfo> {
  return invoke<LicenseInfo>("start_premium_trial");
}

export async function listAvailableSkills(): Promise<SkillInfo[]> {
  return invoke<SkillInfo[]>("list_available_skills");
}

export async function installSkill(skillId: string): Promise<any> {
  return invoke<any>("install_skill", { skillId: skillId });
}

export async function listInstalledSkills(): Promise<any[]> {
  return invoke<any[]>("list_installed_skills");
}

export async function uninstallSkill(skillId: string): Promise<void> {
  return invoke<void>("uninstall_skill", { skillId: skillId });
}

export async function getSetting(key: string): Promise<string | null> {
  return invoke<string | null>("get_setting", { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  return invoke<void>("set_setting", { key, value });
}

export async function openUrl(url: string): Promise<void> {
  return invoke<void>("open_url", { url });
}

export async function chat(messages: any[], modelInternalId: string): Promise<string> {
  return invoke<string>("chat", { messages, modelInternalId: modelInternalId });
}

export async function webSearch(query: string): Promise<string> {
  return invoke<string>("web_search", { query });
}






// Check if Tauri is available
export function isTauriAvailable(): boolean {
  return !!tauriInvoke;
}
