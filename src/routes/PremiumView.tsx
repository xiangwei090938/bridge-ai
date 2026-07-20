import React, { useState, useEffect } from "react";
import { checkPremiumStatus, activatePremium, startPremiumTrial } from "../services/api";
import { LicenseInfo } from "../types";

export default function PremiumView() {
  const [status, setStatus] = useState<LicenseInfo | null>(null);
  const [licenseKey, setLicenseKey] = useState("");
  const [loading, setLoading] = useState(true);
  const [activating, setActivating] = useState(false);
  const [message, setMessage] = useState("");

  useEffect(() => { load(); }, []);

  async function load() {
    try {
      const s = await checkPremiumStatus();
      setStatus(s);
    } catch (e) { console.error(e); }
    setLoading(false);
  }

  async function handleActivate() {
    if (!licenseKey.trim()) return;
    setActivating(true);
    setMessage("");
    try {
      const result = await activatePremium(licenseKey.trim());
      setMessage("高级版已激活！");
      load();
    } catch (e: any) {
      setMessage("激活失败: " + (e?.toString() || "未知错误"));
    }
    setActivating(false);
  }

  async function handleTrial() {
    setActivating(true);
    setMessage("");
    try {
      await startPremiumTrial();
      setMessage("7天免费试用已开启！");
      load();
    } catch (e: any) {
      setMessage("试用失败: " + (e?.toString() || "未知错误"));
    }
    setActivating(false);
  }

  if (loading) {
    return <div style={{ textAlign: "center", padding: "60px 0", color: "var(--text-secondary)", fontSize: 14 }}>
      <div className="loading-spinner" style={{ margin: "0 auto 12px" }} />
      加载中...
    </div>;
  }

  const isActive = status?.is_premium;

  return (
    <div className="premium-wrap">
      <div className="premium-header">
        <div className="premium-icon">{isActive ? "\u{2B50}" : "\u{1F3C6}"}</div>
        <h2 className="premium-title">Bridge AI 高级版</h2>
        <p className="premium-subtitle">
          {isActive ? "感谢你成为高级会员！" : "解锁全部高级功能"}
        </p>
      </div>

      <div className="premium-card">
        <div className="premium-price">
          ¥29.9 <span>/ 年</span>
        </div>
        <p className="premium-desc">按年订阅，可随时取消</p>

        <div className="premium-features">
          {["技能商店全部技能", "联网搜索", "批量配置导入导出", "自定义 Prompt 模板", "会话批量导出", "全局快捷键"].map(f => (
            <div key={f} className="premium-feature">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              <span>{f}</span>
            </div>
          ))}
        </div>

        {isActive ? (
          <div style={{ textAlign: "center", padding: "12px 0", fontSize: 13, color: "var(--text-secondary)" }}>
            有效期至: {new Date(status!.expires_at!).toLocaleDateString("zh-CN")}
            <br />
            剩余 {status!.days_remaining} 天
          </div>
        ) : (
          <>
            <button className="btn btn-primary" onClick={handleTrial} disabled={activating}
              style={{ width: "100%", marginBottom: 10 }}>
              {activating ? "处理中..." : "7 天免费试用"}
            </button>
            <div style={{ position: "relative", margin: "16px 0", textAlign: "center", color: "var(--text-muted)", fontSize: 12 }}>
              <span style={{ background: "var(--bg-card)", padding: "0 10px", position: "relative", zIndex: 1 }}>或使用激活码</span>
              <div style={{ borderTop: "1px solid var(--border)", position: "absolute", top: "50%", left: 0, right: 0 }} />
            </div>
            <div style={{ display: "flex", gap: 8 }}>
              <input className="input" type="text" placeholder="输入激活码"
                value={licenseKey} onChange={e => setLicenseKey(e.target.value)} />
              <button className="btn btn-secondary" onClick={handleActivate} disabled={!licenseKey.trim() || activating}>
                激活
              </button>
            </div>
          </>
        )}

        {message && (
          <div style={{
            marginTop: 12, padding: "8px 12px", borderRadius: 6, fontSize: 13,
            background: isActive ? "var(--success-bg)" : "var(--error-bg)",
            color: isActive ? "var(--success)" : "var(--error)",
            textAlign: "center",
          }}>
            {message}
          </div>
        )}
      </div>
    </div>
  );
}

