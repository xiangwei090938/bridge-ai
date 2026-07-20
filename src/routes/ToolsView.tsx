import React, { useState, useEffect, useRef } from "react";
import { scanTools, listModels, launchTool, installTool, syncToolConfig } from "../services/api";
import { ToolInfo, UserModel } from "../types";
import aiderIcon from "../assets/aider-icon.svg";
import chatgptIcon from "../assets/chatgpt-icon.png";
import chatgptdesktopIcon from "../assets/chatgptdesktop-icon.png";
import claudecodeIcon from "../assets/claudecode-icon.svg";
import claudedesktopIcon from "../assets/claudedesktop-icon.svg";
import claudescienceIcon from "../assets/claudescience-icon.svg";
import codexIcon from "../assets/codex-icon.png";
import coffeecliIcon from "../assets/coffeecli-icon.svg";
import cursorIcon from "../assets/cursor-icon.svg";
import geminidesktopIcon from "../assets/geminidesktop-icon.png";
import grokIcon from "../assets/grok-icon.png";
import hermesIcon from "../assets/hermes-icon.svg";
import kimicodeIcon from "../assets/kimicode-icon.svg";
import mimocodeIcon from "../assets/mimocode-icon.svg";
import openclawIcon from "../assets/openclaw-icon.png";
import opencodeIcon from "../assets/opencode-icon.png";
import opencodedesktopIcon from "../assets/opencodedesktop-icon.png";
import openscienceIcon from "../assets/openscience-icon.png";
import piIcon from "../assets/pi-icon.svg";
import qwencodeIcon from "../assets/qwencode-icon.png";
import traeIcon from "../assets/trae-icon.svg";
import traecnIcon from "../assets/traecn-icon.svg";
import translatorIcon from "../assets/translator-icon.svg";
import vibe_tradingIcon from "../assets/vibe-trading-icon.svg";
import vscodeIcon from "../assets/vscode-icon.png";
import workbuddyIcon from "../assets/workbuddy-icon.png";
import reversiIcon from "../assets/reversi-icon.svg";
import zcodeIcon from "../assets/zcode-icon.svg";

const CATEGORIES = ["全部", "桌面端"];

const TOOL_ICONS: Record<string, string> = {
  chatgptdesktop: chatgptIcon,
  claudedesktop: "📥",
  cursor: "⚙",
  opencode: "",
  hermes: "📬",
  claudecode: "👇",
  vscode: "📑",
  geminidesktop: "⭐",
  aider: "🔧",
  claudescience: "🔬",
  codex: "💬",
  coffeecli: "☕",
  grok: "🤖",
  kimicode: "💠",
  mimocode: "🌐",
  openclaw: "🧠",
  opencodedesktop: "💻",
  openscience: "🔭",
  pi: "",
  qwencode: "📝",
  reversi: "🎮",
  trae: "🌍",
  traecn: "🌏",
  translator: "",
  "vibe-trading": "",
  workbuddy: "",
  zcode: "📁",
};

const TOOL_IMAGES: Record<string, string> = {
  aider: aiderIcon,
  chatgpt: chatgptIcon,
  chatgptdesktop: chatgptdesktopIcon,
  claudecode: claudecodeIcon,
  claudedesktop: claudedesktopIcon,
  claudescience: claudescienceIcon,
  codex: codexIcon,
  coffeecli: coffeecliIcon,
  cursor: cursorIcon,
  geminidesktop: geminidesktopIcon,
  grok: grokIcon,
  hermes: hermesIcon,
  kimicode: kimicodeIcon,
  mimocode: mimocodeIcon,
  openclaw: openclawIcon,
  opencode: opencodeIcon,
  opencodedesktop: opencodedesktopIcon,
  openscience: openscienceIcon,
  pi: piIcon,
  qwencode: qwencodeIcon,
  trae: traeIcon,
  traecn: traecnIcon,
  translator: translatorIcon,
  "vibe-trading": vibe_tradingIcon,
  vscode: vscodeIcon,
  workbuddy: workbuddyIcon,
  zcode: zcodeIcon,
};

const TOOL_COLORS: Record<string, string> = {
  chatgptdesktop: "#10A37F",
  claudedesktop: "#D97757",
  cursor: "#6C47FF",
  opencode: "#3B82F6",
  hermes: "#8B5CF6",
  claudecode: "#D97757",
  vscode: "#007ACC",
  geminidesktop: "#4285F4",
  aider: "#10B981",
  claudescience: "#8B5CF6",
  codex: "#10A37F",
  coffeecli: "#6B7280",
  grok: "#EF4444",
  kimicode: "#EC4899",
  mimocode: "#14B8A6",
  openclaw: "#F59E0B",
  opencodedesktop: "#3B82F6",
  openscience: "#22D3EE",
  pi: "#6366F1",
  qwencode: "#06B6D4",
  reversi: "#F97316",
  trae: "#8B5CF6",
  traecn: "#DC2626",
  translator: "#10B981",
  "vibe-trading": "#22C55E",
  workbuddy: "#A855F7",
  zcode: "#2563EB",
};


interface LogEntry {
  id: string;
  time: string;
  message: string;
  status: "pending" | "success" | "error" | "info";
}

export default function ToolsView() {
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [models, setModels] = useState<UserModel[]>([]);
  const [selectedCategory, setSelectedCategory] = useState("全部");
  const [selectedTool, setSelectedTool] = useState<ToolInfo | null>(null);
  const [selectedModelId, setSelectedModelId] = useState("");
  const [autoLaunch, setAutoLaunch] = useState(true);
  const [modifyConfig, setModifyConfig] = useState(true);
  const [responsesEnabled, setResponsesEnabled] = useState(false);
  const [webSearchEnabled, setWebSearchEnabled] = useState(false);
  const [loading, setLoading] = useState(true);
  const [isLaunching, setIsLaunching] = useState(false);
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const logEndRef = useRef<HTMLDivElement>(null);


  useEffect(() => { loadData(); }, []);


  useEffect(() => {
    logEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  async function loadData() {
    setLoading(true);
    try {
      const [t, m] = await Promise.all([scanTools(), listModels()]);
      setTools(t);
      setModels(m);
      if (m.length > 0) setSelectedModelId(m[0].internalId);
    } catch (e) { console.error(e); }
    setLoading(false);
  }

  const filteredTools = selectedCategory === "全部"
    ? tools
    : tools.filter(t => t.category === selectedCategory);

  const installedTools = filteredTools.filter(t => t.is_installed);
  const uninstalledTools = filteredTools.filter(t => !t.is_installed);

  function addLog(message: string, status: LogEntry["status"] = "info") {
    const now = new Date();
    const time = now.toLocaleTimeString("zh-CN", { hour12: false });
    setLogs(prev => [...prev, { id: Date.now().toString() + Math.random(), time, message, status }]);
  }

  function clearLogs() {
    setLogs([]);
  }

  async function handleLaunch() {
    if (!selectedTool) return;
    if (!selectedModelId) {
      addLog("请先在右侧选择一个模型", "error");
      return;
    }

    setIsLaunching(true);
    setLogs([]);

    const selectedModel = models.find(m => m.internalId === selectedModelId);
    const modelName = selectedModel
      ? selectedModel.name + " (" + selectedModel.modelId + ")"
      : "未知模型";

    addLog("准备同步模型配置: " + modelName, "info");

    if (modifyConfig) {
      try {
        addLog("正在同步配置到 " + selectedTool.name + "...", "pending");
        const syncResult = await syncToolConfig(selectedTool.id, selectedModelId);
        if (syncResult && syncResult.success) {
          addLog("配置同步成功: " + syncResult.message, "success");
        } else {
          addLog("配置同步完成: " + (syncResult ? syncResult.message : "已更新"), "success");
        }
      } catch (e: any) {
        addLog("配置同步失败: " + (e ? e.toString() : "未知错误"), "error");
        setIsLaunching(false);
        return;
      }
    } else {
      addLog("已跳过配置同步（未勾选“修改模型配置”）", "info");
    }

    try {
      addLog("正在启动 " + selectedTool.name + "...", "pending");
      const launchMsg = await launchTool(selectedTool.id);
      addLog("启动成功: " + launchMsg, "success");
    } catch (e: any) {
      addLog("启动失败: " + (e ? e.toString() : "未知错误"), "error");
    }

    setIsLaunching(false);
  }

  function handleInstall(toolName: string) {
    installTool(toolName).then(msg => {
      addLog("安装 " + toolName + ": " + msg, "info");
    });
  }

  return (
    <div className="panel-row">
      {/* Center: Tool Cards Grid */}
      <div className="panel-left">
        {/* Category Tabs */}
        <div className="tools-category-tabs">
          {CATEGORIES.map(cat => (
            <button
              key={cat}
              className={"cat-tab" + (selectedCategory === cat ? " active" : "")}
              onClick={() => setSelectedCategory(cat)}
            >
              {cat}
            </button>
          ))}
        </div>

        {/* Tool Cards Grid */}
        <div className="tools-grid-container">
          {loading ? (
            <div className="tools-loading">
              <div className="loading-spinner" />
              <span>Loading...</span>
            </div>
          ) : (
            <>
              {/* Installed Tools */}
              {installedTools.map(tool => {
                const icon = TOOL_ICONS[tool.id] || tool.name.charAt(0);
                const color = TOOL_COLORS[tool.id] || "#10A37F";
                const isSelected = selectedTool?.id === tool.id;
                return (
                  <div
                    key={tool.id}
                    className={"tool-card-grid" + (isSelected ? " selected" : "")}
                    onClick={() => setSelectedTool(tool)}
                  >
                    <div className="tool-card-header">
                      <span className="tool-card-name">{tool.name}</span>
                      <div className="tool-icon-wrapper" style={{ background: color + "20", color: color }}>
                        {TOOL_IMAGES[tool.id] ? (
                          <img src={TOOL_IMAGES[tool.id]} alt={tool.name} className="tool-icon-img" />
                        ) : (
                          <span className="tool-icon-text">{icon}</span>
                        )}
                      </div>
                    </div>
                    <div className="tool-card-details">
                      <div className="tool-detail-row">
                        <span className="detail-label">模型:</span>
                        <span className="detail-value">{tool.api_protocols?.[0] || "N/A"}</span>
                      </div>
                      <div className="tool-detail-row">
                        <span className="detail-label">应用:</span>
                        <span className="detail-value">{tool.install_path || "N/A"}</span>
                      </div>
                      <div className="tool-detail-row">
                        <span className="detail-label">配置:</span>
                        <span className="detail-value">{tool.config_path || "N/A"}</span>
                      </div>
                      <div className="tool-detail-row">
                        <span className="detail-label">版本:</span>
                        <span className="detail-value">-</span>
                      </div>
                    </div>
                  </div>
                );
              })}

              {/* Uninstalled Tools */}
              {uninstalledTools.map(tool => {
                const icon = TOOL_ICONS[tool.id] || tool.name.charAt(0);
                const color = TOOL_COLORS[tool.id] || "#10A37F";
                return (
                  <div key={tool.id} className="tool-card-grid uninstalled">
                    <div className="tool-card-header">
                      <span className="tool-card-name">{tool.name}</span>
                      <div className="tool-icon-wrapper" style={{ background: color + "20", color: color }}>
                        {TOOL_IMAGES[tool.id] ? (
                          <img src={TOOL_IMAGES[tool.id]} alt={tool.name} className="tool-icon-img" />
                        ) : (
                          <span className="tool-icon-text">{icon}</span>
                        )}
                      </div>
                    </div>
                    <button
                      className="btn-install"
                      onClick={(e) => { e.stopPropagation(); handleInstall(tool.name); }}
                    >
                      AI 自动安装
                    </button>
                  </div>
                );
              })}
            </>
          )}
        </div>

        {/* Operation Log Panel */}
        <div className="tools-log-panel">
          <div className="log-header">
            <span className="log-title">操作日志</span>
            {logs.length > 0 && (
              <button className="btn-clear-log" onClick={clearLogs} title="清空日志">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                  <polyline points="3 6 5 6 21 6" />
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
                </svg>
              </button>
            )}
          </div>
          <div className="log-body">
            {logs.length === 0 ? (
              <div className="log-empty">
                <span>选择工具并点击“启动应用”后，操作日志将显示在这里</span>
              </div>
            ) : (
              logs.map(log => (
                <div key={log.id} className={"log-entry log-" + log.status}>
                  <span className="log-time">{log.time}</span>
                  <span className="log-status-icon">
                    {log.status === "success" && "✓"}
                    {log.status === "error" && "✗"}
                    {log.status === "pending" && "○"}
                    {log.status === "info" && "ℹ"}
                  </span>
                  <span className="log-message">{log.message}</span>
                </div>
              ))
            )}
            <div ref={logEndRef} />
          </div>
        </div>

        {/* Bottom Action Bar */}
        <div className="tools-bottom-bar">
          <div className="bottom-warning">
            切换 ChatGPT / Codex CLI / Claude 桌面端 / Claude Code 的模型后，请保持 Bridge AI 运行。
          </div>
          <button className="btn-launch" onClick={handleLaunch} disabled={!selectedTool || isLaunching}>
            {isLaunching ? "启动中..." : "启动应用"}
          </button>
          <div className="bottom-checkboxes">
            <label className="checkbox-label">
              <input type="checkbox" checked={autoLaunch} onChange={e => setAutoLaunch(e.target.checked)} />
              直接启动应用
            </label>
            <label className="checkbox-label">
              <input type="checkbox" checked={modifyConfig} onChange={e => setModifyConfig(e.target.checked)} />
              修改模型配置
            </label>
          </div>
        </div>
      </div>

      {/* Right Panel: Model Selection */}
      <div className="detail-panel tools-right-panel">
        <div className="tools-right-header">
          <h3 className="tools-right-title">模型</h3>
          <span className="tools-right-model-name">
            {models.find(m => m.internalId === selectedModelId)?.name || "ChatGPT"}
          </span>
        </div>

        {/* Toggle Switches */}
        <div className="tools-toggles">
          <div className="toggle-row">
            <span className="toggle-label">Responses</span>
            <label className="switch">
              <input type="checkbox" checked={responsesEnabled} onChange={e => setResponsesEnabled(e.target.checked)} />
              <span className="slider"></span>
            </label>
            <button className="help-icon">?</button>
          </div>
          <div className="toggle-row">
            <span className="toggle-label">Web Search</span>
            <label className="switch">
              <input type="checkbox" checked={webSearchEnabled} onChange={e => setWebSearchEnabled(e.target.checked)} />
              <span className="slider"></span>
            </label>
            <button className="help-icon">?</button>
          </div>
        </div>

        {/* Model List */}
        <div className="model-selection-list">
          {models.map(model => (
            <div
              key={model.internalId}
              className={"model-selection-item" + (selectedModelId === model.internalId ? " selected" : "")}
              onClick={() => setSelectedModelId(model.internalId)}
            >
              <input
                type="radio"
                name="model"
                checked={selectedModelId === model.internalId}
                onChange={() => setSelectedModelId(model.internalId)}
                className="model-radio"
              />
              <div className="model-selection-info">
                <div className="model-selection-name">{model.name}</div>
                <div className="model-selection-url">{model.baseUrl}</div>
              </div>
              {models.indexOf(model) === 0 && (
                <span className="model-default-badge">默认</span>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
