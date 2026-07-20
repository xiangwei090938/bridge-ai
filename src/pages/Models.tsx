import React, { useState, useEffect } from "react";
import { listProviders, listModels, addModel, deleteModel, updateApiKey, testConnection, openUrl, isTauriAvailable } from "../services/api";

// Open URL in system browser
function openExternalUrl(url: string) {
  if (!url || url === "#") return;
  openUrl(url).catch(() => {
    window.open(url, "_blank");
  });
}
import { UserModel, ProviderEntry } from "../types";
import providerBaiduIcon from "../assets/provider-baidu-icon.svg";
import providerDeepseekIcon from "../assets/provider-deepseek-icon.svg";
import providerKimiIcon from "../assets/provider-kimi-icon.svg";
import providerMinimaxIcon from "../assets/provider-minimax-icon.svg";
import providerOllamaIcon from "../assets/provider-ollama-icon.svg";
import providerQwenIcon from "../assets/provider-qwen-icon.svg";
import providerTencentIcon from "../assets/provider-tencent-icon.svg";


const PROVIDER_META: Record<string, { icon: string; color: string; url: string }> = {
  "火山引擎": { icon: "V", color: "#FF6B35", url: "https://www.volcengine.com" },
  "ERNIE 百度千帆": { icon: providerBaiduIcon, color: "#2932E1", url: "https://console.bce.baidu.com" },
  "Qwen 阿里百炼": { icon: providerQwenIcon, color: "#FF6A00", url: "https://bailian.console.alibabacloud.com" },
  "Hunyuan 腾讯混元": { icon: providerTencentIcon, color: "#00A4FF", url: "https://console.cloud.tencent.com" },
  "Stepfun 阶跃星辰": { icon: "S", color: "#6C5CE7", url: "https://www.stepfun.com" },
  "UCloud 优云智算": { icon: "U", color: "#00B4D8", url: "https://passport.compshare.cn" },
  "MiniMax": { icon: providerMinimaxIcon, color: "#FF4757", url: "https://www.minimax.io" },
  "Z.ai": { icon: "Z", color: "#2D3436", url: "https://api.z.ai" },
  "Kimi Global": { icon: providerKimiIcon, color: "#6C5CE7", url: "https://platform.kimi.ai" },
  "OpenAI": { icon: "O", color: "#10A37F", url: "https://openai.com" },
  "Anthropic": { icon: "A", color: "#D97757", url: "https://anthropic.com" },
  "Google Gemini": { icon: "G", color: "#4285F4", url: "https://gemini.google.com" },
  "DeepSeek": { icon: providerDeepseekIcon, color: "#4F46E5", url: "https://deepseek.com" },
  "Ollama": { icon: providerOllamaIcon, color: "#8B5CF6", url: "https://ollama.ai" },
};

function getProvMeta(name: string, type: string): { icon: string; bg: string } {
  const pm = PROVIDER_META[name];
  if (pm) return { icon: pm.icon, bg: pm.color + "22" };
  const fallback: Record<string, { icon: string; color: string }> = {
    openai: { icon: "O", color: "#10A37F" },
    anthropic: { icon: "A", color: "#D97757" },
    deepseek: { icon: "D", color: "#4F46E5" },
    tongyi: { icon: "T", color: "#EC4899" },
    ollama: { icon: "Ol", color: "#8B5CF6" },
    volcengine: { icon: "V", color: "#FF6B35" },
    baidu: { icon: "B", color: "#2932E1" },
    alibaba: { icon: "Q", color: "#FF6A00" },
    tencent: { icon: "H", color: "#00A4FF" },
    stepfun: { icon: "S", color: "#6C5CE7" },
    ucloud: { icon: "U", color: "#00B4D8" },
    minimax: { icon: "M", color: "#FF4757" },
    zai: { icon: "Z", color: "#2D3436" },
    kimi: { icon: "K", color: "#6C5CE7" },
    gemini: { icon: "G", color: "#4285F4" },
  };
  const f = fallback[type];
  if (f) return { icon: f.icon, bg: f.color + "22" };
  return { icon: name.charAt(0).toUpperCase(), bg: "var(--bg-hover)" };
}

export default function Models() {
  const [providers, setProviders] = useState<ProviderEntry[]>([]);
  const [models, setModels] = useState<UserModel[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedProv, setSelectedProv] = useState<ProviderEntry | null>(null);
  const [configForm, setConfigForm] = useState({
    name: "", modelId: "", baseUrl: "", anthropicUrl: "", apiKey: "", type: "openai"
  });
  const [saving, setSaving] = useState(false);
  const [editingKey, setEditingKey] = useState<string | null>(null);
  const [editingKeyValue, setEditingKeyValue] = useState("");
  const [revealedKeys, setRevealedKeys] = useState<Set<string>>(new Set());
  const [testResult, setTestResult] = useState<{ text: string; ok: boolean } | null>(null);
  const [rightTab, setRightTab] = useState<"providers" | "relay">("providers");

  useEffect(() => { 
    if (!isTauriAvailable()) {
      console.warn('[Models] Tauri not available - running in browser mode');
      setTestResult({ text: "️ 浏览器模式无法保存模型，请使用桌面端运行", ok: false });
    }
    load(); 
  }, []);

  async function load() {
    setLoading(true);
    try {
      const [p, m] = await Promise.all([listProviders(), listModels()]);
      setProviders(p.providers);
      setModels(m);
    } catch (e) { console.error(e); }
    setLoading(false);
  }

  function selectProvider(p: ProviderEntry) {
    if (selectedProv?.name === p.name) { setSelectedProv(null); return; }
    setSelectedProv(p);
    setConfigForm({
      name: p.name,
      modelId: p.modelIds?.[0] || p.modelId || "",
      baseUrl: p.baseUrl || "",
      anthropicUrl: p.anthropicUrl || "",
      apiKey: "",
      type: p.name.toLowerCase() === "ollama" ? "ollama" : "openai",
    });
    setTestResult(null);
  }

  async function handleSave() {
    if (!configForm.apiKey) {
      setTestResult({ text: "请先输入 API Key", ok: false });
      return;
    }
    setSaving(true);
    try {
      await addModel(configForm);
      setTestResult({ text: "模型配置成功！", ok: true });
      await load();
    } catch (e: any) {
      const errorMsg = e?.message || e?.toString() || '未知错误';
      setTestResult({ text: "失败：" + errorMsg, ok: false });
    }
    setSaving(false);
  }

  async function handleDelete(id: string) {
    try { await deleteModel(id); await load(); } catch (e: any) { alert("删除失败：" + e); }
  }

  async function handleTest(name: string, baseUrl: string, apiKey: string) {
    setTestResult({ text: "测试中...", ok: false });
    try {
      const result = await testConnection(name, baseUrl, apiKey);
      setTestResult({ text: "连接成功：" + result, ok: true });
    } catch (e: any) {
      setTestResult({ text: "失败：" + (e?.toString() || "未知错误"), ok: false });
    }
  }

  async function handleUpdateKey(internalId: string) {
    try { await updateApiKey(internalId, editingKeyValue); setEditingKey(null); setEditingKeyValue(""); await load(); } catch (e: any) { alert("更新失败：" + e); }
  }

  return (
    <div className="panel-row">
      <div className="panel-left models-middle-col">
        <div className="models-middle-header">
          <h2 className="models-middle-title">已配置模型</h2>
          <p className="models-middle-subtitle">
            {models.length > 0 ? "共 " + models.length + " 个模型" : "请在右侧选择供应商并输入 API Key"}
          </p>
        </div>
        <div className="models-middle-body">
          {loading ? (
            <div className="models-empty">
              <div className="loading-spinner" />
              <span>加载中...</span>
            </div>
          ) : models.length === 0 ? (
            <div className="models-empty">
              <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--text-muted)" strokeWidth="1.5" opacity="0.3">
                <path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
              <span className="models-empty-text">尚未配置模型</span>
              <span className="models-empty-hint">请在右侧面板选择供应商并输入 API Key</span>
            </div>
          ) : (
            <div className="models-grid">
              {models.map(m => {
                const { icon, bg } = getProvMeta(m.name, m.type);
                return (
                  <div key={m.internalId} className="model-grid-card">
                    <div className="model-card-top">
                      <span className="model-type-badge">云端</span>
                      <div className="model-card-actions-top">
                        <button className="btn-link" onClick={() => handleDelete(m.internalId)}>删除</button>
                        <button className="btn-link" onClick={() => { setEditingKey(m.internalId); setEditingKeyValue(""); }}>编辑</button>
                      </div>
                    </div>
                    <div className="model-card-name-row">
                      <div className="model-icon-small" style={{ background: bg }}>
                        {typeof icon === 'string' && icon.length <= 3 ? icon : <img src={icon} alt={m.name} className="model-icon-img" />}
                      </div>
                      <span className="model-card-name">{m.name}</span>
                    </div>
                    <div className="model-card-info">
                      <div className="model-info-row"><span className="info-label">模型:</span> {m.modelId}</div>
                      <div className="model-info-row"><span className="info-label">来源:</span> {m.baseUrl || "N/A"}</div>
                      <div className="model-info-row"><span className="info-label">延迟:</span> 未测试</div>
                    </div>
                    <div className="model-card-protocols">
                      <span className="protocol-tag">[OpenAI]</span>
                      <span className="protocol-tag">[Anthropic]</span>
                    </div>
                  </div>
                );
              })}
              <div className="model-grid-card model-add-card" onClick={() => setSelectedProv(providers[0] || null)}>
                <div className="model-add-content">
                  <div className="model-add-icon">+</div>
                  <div className="model-add-text">添加模型</div>
                  <div className="model-add-hint">OpenAI / Anthropic API</div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>

      <div className="detail-panel models-right-col">
        <div className="models-right-header">
          <div className="models-right-tabs">
            <button className={"models-right-tab" + (rightTab === "providers" ? " active" : "")} onClick={() => setRightTab("providers")}>大模型厂商</button>
            <button className={"models-right-tab" + (rightTab === "relay" ? " active" : "")} onClick={() => setRightTab("relay")}>模型中转站</button>
          </div>
        </div>
        <div className="models-right-body">
          <div className="provider-list">
            {providers.map(p => {
              const pm = PROVIDER_META[p.name];
              const icon = pm?.icon || "O";
              const color = pm?.color || "#10A37F";
              const isSelected = selectedProv?.name === p.name;
              return (
                <div key={p.name}>
                  <div className={"provider-list-item" + (isSelected ? " selected" : "")} onClick={() => selectProvider(p)}>
                    <button className="provider-list-add" onClick={(e) => { e.stopPropagation(); selectProvider(p); }} title="添加此供应商">+</button>
                    <div className="provider-list-icon" style={{ background: color + "22", color: color }}>
                    {typeof icon === 'string' && icon.length <= 3 ? icon : <img src={icon} alt={p.name} className="provider-icon-img" />}
                  </div>
                    <div className="provider-list-info">
                      <div className="provider-list-name">{p.name}</div>
                      <div className="provider-list-url">{p.url || "无地址"}</div>
                    </div>
                    <button className="provider-list-external" onClick={(e) => { e.stopPropagation(); openExternalUrl(p.url || ""); }} title="打开官网申请API">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                        <polyline points="15 3 21 3 21 9" />
                        <line x1="10" y1="14" x2="21" y2="3" />
                      </svg>
                    </button>
                  </div>
                  {isSelected && (
                    <div className="provider-config">
                      <div className="provider-config-inner">
                        <div className="field-group">
                          <label className="field-label">名称</label>
                          <input className="input" value={configForm.name} onChange={e => setConfigForm({ ...configForm, name: e.target.value })} />
                        </div>
                        <div className="field-group">
                          <label className="field-label">模型 ID</label>
                          <input className="input" value={configForm.modelId} onChange={e => setConfigForm({ ...configForm, modelId: e.target.value })} />
                        </div>
                        <div className="field-group">
                          <label className="field-label">OpenAI 兼容地址</label>
                          <input className="input" value={configForm.baseUrl} onChange={e => setConfigForm({ ...configForm, baseUrl: e.target.value })} />
                        </div>
                        <div className="field-group">
                          <label className="field-label">Anthropic 地址（可选）</label>
                          <input className="input" value={configForm.anthropicUrl} onChange={e => setConfigForm({ ...configForm, anthropicUrl: e.target.value })} />
                        </div>
                        <div className="field-group">
                          <label className="field-label">API 密钥</label>
                          <input className="input" type="password" value={configForm.apiKey} onChange={e => setConfigForm({ ...configForm, apiKey: e.target.value })} />
                        </div>
                        <div className="field-group">
                          <label className="field-label">协议类型</label>
                          <select className="select" value={configForm.type} onChange={e => setConfigForm({ ...configForm, type: e.target.value })}>
                            <option value="openai">OpenAI 兼容</option>
                            <option value="anthropic">Anthropic 兼容</option>
                            <option value="deepseek">DeepSeek</option>
                            <option value="tongyi">通义千问</option>
                            <option value="ollama">Ollama</option>
                          </select>
                        </div>
                        <button className="btn btn-primary btn-full" onClick={handleSave} disabled={saving}>
                          {saving ? "保存中..." : (configForm.apiKey ? "添加配置" : "请输入 API Key")}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
