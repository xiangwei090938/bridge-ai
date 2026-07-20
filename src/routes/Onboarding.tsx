import React, { useState } from "react";
import { setSetting } from "../services/api";

interface Props {
  onComplete: () => void;
}

const STEPS = ["欢迎", "API Key", "模型选择", "完成"];

export default function Onboarding({ onComplete }: Props) {
  const [step, setStep] = useState(0);
  const [apiKey, setApiKey] = useState("");
  const [provider, setProvider] = useState("openai");
  const [baseUrl, setBaseUrl] = useState("https://api.openai.com/v1");
  const [testResult, setTestResult] = useState("");

  async function handleTest() {
    setTestResult("测试中...");
    try {
      const { testConnection } = await import("../services/api");
      const res = await testConnection(provider, baseUrl, apiKey);
      setTestResult("连接成功! " + res);
    } catch (e: any) {
      setTestResult("连接失败: " + (e?.toString() || "未知错误"));
    }
  }

  async function handleFinish() {
    try {
      await setSetting("onboarding_completed", "true");
      if (apiKey) {
        const { addModel } = await import("../services/api");
        await addModel({
          name: provider === "openai" ? "OpenAI" : provider,
          modelId: "gpt-4o",
          baseUrl: baseUrl,
          apiKey: apiKey,
          type: "openai"
        });
      }
    } catch (e) { console.error(e); }
    onComplete();
  }

  return (
    <div className="onboarding-screen">
      {/* Steps indicator */}
      <div className="onboarding-steps">
        {STEPS.map((s, i) => (
          <span key={s}
            className={"onboarding-step " + (i === step ? "active" : "inactive")}>
            {s}
          </span>
        ))}
      </div>

      {/* Step 0: Welcome */}
      {step === 0 && (
        <div style={{ textAlign: "center", maxWidth: 400 }}>
          <div style={{
            width: 72, height: 72, borderRadius: 18, margin: "0 auto 20px",
            background: "linear-gradient(135deg, var(--accent), var(--primary))",
            display: "flex", alignItems: "center", justifyContent: "center",
            fontSize: 32, fontWeight: 800, color: "#fff",
          }}>B</div>
          <h1 style={{ fontSize: 22, fontWeight: 700, marginBottom: 8 }}>欢迎使用 Bridge AI</h1>
          <p style={{ fontSize: 14, color: "var(--text-secondary)", lineHeight: 1.6, marginBottom: 24 }}>
            统一管理所有 AI 模型的 API Key，一键同步到各个 AI 工具。
            只需几步即可开始使用。
          </p>
          <button className="btn btn-primary" onClick={() => setStep(1)}
            style={{ padding: "10px 32px", fontSize: 14 }}>
            开始配置
          </button>
          <button className="btn btn-ghost" onClick={handleFinish}
            style={{ marginLeft: 10, fontSize: 13 }}>
            跳过，直接使用
          </button>
        </div>
      )}

      {/* Step 1: API Key */}
      {step === 1 && (
        <div style={{ maxWidth: 400, width: "100%" }}>
          <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 6 }}>配置 API Key</h2>
          <p style={{ fontSize: 13, color: "var(--text-secondary)", marginBottom: 16 }}>
            选择你的模型供应商并输入 API Key
          </p>
          <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
            <select className="select" value={provider}
              onChange={e => {
                setProvider(e.target.value);
                if (e.target.value === "openai") setBaseUrl("https://api.openai.com/v1");
                else if (e.target.value === "deepseek") setBaseUrl("https://api.deepseek.com");
                else setBaseUrl("");
              }}>
              <option value="openai">OpenAI</option>
              <option value="deepseek">DeepSeek</option>
              <option value="ollama">Ollama (本地)</option>
              <option value="custom">自定义</option>
            </select>
            <input className="input" type="password" placeholder="API Key"
              value={apiKey} onChange={e => setApiKey(e.target.value)} />
            <input className="input" type="text" placeholder="Base URL"
              value={baseUrl} onChange={e => setBaseUrl(e.target.value)} />
          </div>
          <div style={{ display: "flex", gap: 8, marginTop: 16 }}>
            <button className="btn btn-primary" onClick={() => setStep(2)} style={{ flex: 1 }}>
              下一步
            </button>
            <button className="btn btn-ghost" onClick={() => setStep(0)}>返回</button>
          </div>
        </div>
      )}

      {/* Step 2: Model Selection */}
      {step === 2 && (
        <div style={{ maxWidth: 400, width: "100%" }}>
          <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 6 }}>测试连接</h2>
          <p style={{ fontSize: 13, color: "var(--text-secondary)", marginBottom: 16 }}>
            验证 API Key 是否可用
          </p>
          {testResult && (
            <div style={{
              padding: "8px 12px", borderRadius: 6, marginBottom: 12, fontSize: 13,
              background: testResult.includes("成功") ? "var(--success-bg)" : testResult.includes("失败") ? "var(--error-bg)" : "var(--bg-card)",
              color: testResult.includes("成功") ? "var(--success)" : testResult.includes("失败") ? "var(--error)" : "var(--text)",
            }}>
              {testResult}
            </div>
          )}
          <button className="btn btn-secondary" onClick={handleTest}
            style={{ width: "100%", marginBottom: 12 }}>
            测试连接
          </button>
          <div style={{ display: "flex", gap: 8 }}>
            <button className="btn btn-primary" onClick={() => setStep(3)} style={{ flex: 1 }}>
              下一步
            </button>
            <button className="btn btn-ghost" onClick={() => setStep(1)}>返回</button>
          </div>
        </div>
      )}

      {/* Step 3: Complete */}
      {step === 3 && (
        <div style={{ textAlign: "center", maxWidth: 400 }}>
          <div style={{
            width: 72, height: 72, borderRadius: "50%", margin: "0 auto 20px",
            background: "var(--success-bg)", color: "var(--success)",
            display: "flex", alignItems: "center", justifyContent: "center", fontSize: 36,
          }}>{testResult.includes("成功") || !testResult ? "\u{2713}" : "\u{26A0}"}</div>
          <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 8 }}>
            {testResult.includes("成功") || !testResult ? "配置完成！" : "配置有误"}
          </h2>
          <p style={{ fontSize: 13, color: "var(--text-secondary)", marginBottom: 24 }}>
            {testResult.includes("成功") || !testResult
              ? "现在可以开始使用 Bridge AI 了"
              : "可以在设置中随时修改配置"}
          </p>
          <button className="btn btn-primary" onClick={handleFinish}
            style={{ padding: "10px 32px", fontSize: 14 }}>
            开始使用
          </button>
        </div>
      )}
    </div>
  );
}
