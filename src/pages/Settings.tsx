import React, { useState, useEffect } from "react";
import { getSetting, setSetting } from "../services/api";

export default function Settings() {
  const [theme, setTheme] = useState("dark");
  const [fontSize, setFontSize] = useState("14");
  const [maxContext, setMaxContext] = useState("4096");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState("");

  useEffect(() => { load(); }, []);

  async function load() {
    try {
      const t = await getSetting("theme");
      if (t) setTheme(t);
      const f = await getSetting("font_size");
      if (f) setFontSize(f);
      const m = await getSetting("max_context");
      if (m) setMaxContext(m);
    } catch (e) { console.error(e); }
    setLoading(false);
  }

  async function save(key: string, value: string) {
    try {
      await setSetting(key, value);
      setSaving(key);
      setTimeout(() => setSaving(""), 1200);
    } catch (e) { console.error(e); }
  }

  if (loading) {
    return <div style={{ textAlign: "center", padding: "60px 0", color: "var(--text-secondary)", fontSize: 14 }}>
      <div className="loading-spinner" style={{ margin: "0 auto 12px" }} />
      加载中...
    </div>;
  }

  return (
    <div style={{ padding: "20px 24px", overflow: "auto", height: "100%", maxWidth: 560 }}>
      <h2 style={{ fontSize: 16, fontWeight: 600, marginBottom: 20 }}>设置</h2>

      <div className="model-card">
        <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>外观</h3>
        <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <div>
              <div style={{ fontSize: 13, fontWeight: 500 }}>主题</div>
              <div style={{ fontSize: 11, color: "var(--text-muted)" }}>深色/浅色模式</div>
            </div>
            <div style={{ display: "flex", gap: 6 }}>
              <button className={"btn " + (theme === "dark" ? "btn-primary" : "btn-ghost")}
                onClick={() => { setTheme("dark"); save("theme", "dark"); }}
                style={{ fontSize: 12, padding: "6px 14px" }}>
                深色
              </button>
              <button className={"btn " + (theme === "light" ? "btn-primary" : "btn-ghost")}
                onClick={() => { setTheme("light"); save("theme", "light"); }}
                style={{ fontSize: 12, padding: "6px 14px" }}>
                浅色
              </button>
              <span style={{ fontSize: 11, color: "var(--success)", alignSelf: "center" }}>
                {saving === "theme" ? "\u{2713}" : ""}
              </span>
            </div>
          </div>

          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <div>
              <div style={{ fontSize: 13, fontWeight: 500 }}>字体大小</div>
              <div style={{ fontSize: 11, color: "var(--text-muted)" }}>聊天消息字体大小</div>
            </div>
            <div style={{ display: "flex", gap: 6, alignItems: "center" }}>
              <select className="select" value={fontSize}
                onChange={e => { setFontSize(e.target.value); save("font_size", e.target.value); }}
                style={{ width: 80, padding: "6px 8px", fontSize: 12 }}>
                {[12, 13, 14, 15, 16, 18].map(s => (
                  <option key={s} value={s.toString()}>{s}px</option>
                ))}
              </select>
            </div>
          </div>
        </div>
      </div>

      <div className="model-card">
        <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>对话</h3>
        <div style={{ display: "flex", flexDirection: "column", gap: 12 }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <div>
              <div style={{ fontSize: 13, fontWeight: 500 }}>最大上下文</div>
              <div style={{ fontSize: 11, color: "var(--text-muted)" }}>保留的最大 token 数</div>
            </div>
            <select className="select" value={maxContext}
              onChange={e => { setMaxContext(e.target.value); save("max_context", e.target.value); }}
              style={{ width: 120, padding: "6px 8px", fontSize: 12 }}>
              {["2048", "4096", "8192", "16384", "32768"].map(s => (
                <option key={s} value={s}>{parseInt(s).toLocaleString()}</option>
              ))}
            </select>
          </div>
        </div>
      </div>

      <div className="model-card">
        <h3 style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>关于</h3>
        <div style={{ fontSize: 13, color: "var(--text-secondary)", lineHeight: 1.8 }}>
          <div>Bridge AI v1.0.0</div>
          <div>基于 Tauri + React</div>
          <div style={{ marginTop: 4 }}>
            <a href="#" style={{ color: "var(--primary)", textDecoration: "none" }}
              onClick={e => { e.preventDefault(); /* TODO: open external */ }}>
              官方网站
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
