import React, { useState, useEffect } from "react";
import { listAvailableSkills, installSkill, listInstalledSkills, uninstallSkill, checkPremiumStatus } from "../services/api";
import { SkillInfo } from "../types";

const CATEGORIES = ["All", "Writing", "Programming", "Translation", "Learning", "Tools", "Creative"];

export default function SkillMarket() {
  const [skills, setSkills] = useState<SkillInfo[]>([]);
  const [installed, setInstalled] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState("");
  const [category, setCategory] = useState("All");
  const [selectedSkill, setSelectedSkill] = useState<SkillInfo | null>(null);
  const [isPremium, setIsPremium] = useState(false);

  useEffect(() => { load(); }, []);

  async function load() {
    setLoading(true);
    try {
      const [s, i, p] = await Promise.all([
        listAvailableSkills(), listInstalledSkills(), checkPremiumStatus()
      ]);
      setSkills(s);
      setInstalled(i.map((x: any) => x.id || x.skill_id));
      setIsPremium(p.is_premium);
    } catch (e) { console.error(e); }
    setLoading(false);
  }

  const filtered = skills.filter(s => {
    if (category !== "All" && s.category !== category) return false;
    if (search && !s.name.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  async function handleInstall(skillId: string) {
    try {
      await installSkill(skillId);
      setInstalled(prev => [...prev, skillId]);
    } catch (e) { console.error(e); }
  }

  async function handleUninstall(skillId: string) {
    try {
      await uninstallSkill(skillId);
      setInstalled(prev => prev.filter(id => id !== skillId));
    } catch (e) { console.error(e); }
  }

  return (
    <div className="panel-row">
      <div className="panel-left">
        {/* Search + Filter */}
        <div className="search-bar">
          <input className="input search-input" type="text" placeholder="搜索小工具..."
            value={search} onChange={e => setSearch(e.target.value)} />
          <div className="cat-tabs">
            {CATEGORIES.map(c => (
              <button key={c} className={"cat-tab" + (category === c ? " active" : "")} onClick={() => setCategory(c)}>{c}</button>
            ))}
          </div>
          <button className="btn btn-ghost" onClick={load} style={{ marginLeft: "auto" }}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <polyline points="23 4 23 10 17 10" /><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
            </svg>
            刷新
          </button>
        </div>

        {/* Grid */}
        <div className="tool-grid-wrap">
          {loading ? (
            <div style={{ textAlign: "center", padding: "60px 0", color: "var(--text-secondary)", fontSize: 14 }}>
              <div className="loading-spinner" style={{ margin: "0 auto 12px" }} />
              加载中...
            </div>
          ) : filtered.length === 0 ? (
            <div style={{ textAlign: "center", padding: "60px 0", color: "var(--text-muted)", fontSize: 14 }}>
              没有找到匹配的小工具
            </div>
          ) : (
            <div className="skill-grid">
              {filtered.map(skill => {
                const isInstalled = installed.includes(skill.id);
                const needsPremium = skill.requires_premium && !isPremium;
                return (
                  <div key={skill.id} className="skill-card"
                    onClick={() => setSelectedSkill(skill)}
                    style={{ cursor: "pointer", opacity: needsPremium ? 0.6 : 1 }}>
                    <div className="skill-card-header">
                      <div className="skill-icon" style={{ background: "var(--bg-active)" }}>
                        {skill.icon || "\u{1F4E6}"}
                      </div>
                      <div>
                        <div className="skill-name">{skill.name}</div>
                        <div className="skill-version">v{skill.version}</div>
                      </div>
                    </div>
                    <div className="skill-desc">{skill.description}</div>
                    <div className="skill-tags">
                      {skill.tags.slice(0, 3).map(tag => (
                        <span key={tag} className="skill-tag">{tag}</span>
                      ))}
                      <span className="skill-tag" style={{ background: "var(--primary-light)", color: "var(--primary)" }}>
                        {skill.category}
                      </span>
                    </div>
                    <div className="skill-footer">
                      <span className="skill-downloads">{skill.downloads} 次下载</span>
                      {needsPremium ? (
                        <span className="badge badge-warning">高级版</span>
                      ) : isInstalled ? (
                        <button className="btn btn-ghost" style={{ color: "var(--error)", fontSize: 12, padding: "4px 10px" }}
                          onClick={e => { e.stopPropagation(); handleUninstall(skill.id); }}>
                          卸载
                        </button>
                      ) : (
                        <button className="btn btn-primary" style={{ fontSize: 12, padding: "4px 14px" }}
                          onClick={e => { e.stopPropagation(); handleInstall(skill.id); }}>
                          安装
                        </button>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>

      {/* Right: Detail Panel */}
      <div className="detail-panel">
        {!selectedSkill ? (
          <div className="detail-empty">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" opacity="0.3">
              <rect x="3" y="3" width="7" height="7" rx="1" />
              <rect x="14" y="3" width="7" height="7" rx="1" />
              <rect x="14" y="14" width="7" height="7" rx="1" />
              <rect x="3" y="14" width="7" height="7" rx="1" />
            </svg>
            <span>选择一个小工具查看详情</span>
          </div>
        ) : (
          <>
            <div className="detail-header">
              <h3>{selectedSkill.name}</h3>
              <div className="detail-subtitle">v{selectedSkill.version} · {selectedSkill.author}</div>
            </div>
            <div className="detail-body">
              <div className="detail-section">
                <div className="detail-label">描述</div>
                <div style={{ fontSize: 13, color: "var(--text-secondary)", lineHeight: 1.5 }}>
                  {selectedSkill.description}
                </div>
              </div>
              <div className="detail-section">
                <div className="detail-label">分类</div>
                <span className="skill-tag" style={{ background: "var(--primary-light)", color: "var(--primary)" }}>
                  {selectedSkill.category}
                </span>
              </div>
              <div className="detail-section">
                <div className="detail-label">大小</div>
                <span style={{ fontSize: 13, color: "var(--text-secondary)" }}>
                  {(selectedSkill.size_bytes / 1024).toFixed(0)} KB
                </span>
              </div>
              <div className="detail-section">
                <div className="detail-label">下载次数</div>
                <span style={{ fontSize: 13, color: "var(--text-secondary)" }}>
                  {selectedSkill.downloads.toLocaleString()}
                </span>
              </div>
              <div className="detail-section">
                <div className="detail-label">标签</div>
                <div className="skill-tags">
                  {selectedSkill.tags.map(tag => (
                    <span key={tag} className="skill-tag">{tag}</span>
                  ))}
                </div>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
