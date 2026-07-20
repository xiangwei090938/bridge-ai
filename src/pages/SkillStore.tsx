import React from 'react'

const skills = [
  { name: 'ChatGPT Desktop', size: '~200MB', downloads: '125K', desc: 'OpenAI 官方桌面端，支持 GPT-4o 等模型' },
  { name: 'Claude Desktop', size: '~180MB', downloads: '98K', desc: 'Anthropic 官方桌面端，支持 Claude 系列' },
  { name: 'Ollama', size: '~500MB', downloads: '856K', desc: '本地运行大语言模型的工具' },
  { name: 'Cursor', size: '~150MB', downloads: '423K', desc: 'AI 驱动的代码编辑器' },
  { name: 'Hermes Desktop', size: '~120MB', downloads: '45K', desc: '多模型 AI 助手桌面端' },
  { name: 'OpenCode Desktop', size: '~90MB', downloads: '67K', desc: '开源 AI 编程助手' },
]

export function SkillStore() {
  return (
    <div>
      <div style={{ marginBottom: '20px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 700, marginBottom: '4px' }}>技能商店 🎯</h1>
        <p style={{ fontSize: '14px', color: 'var(--color-text-muted)' }}>一键下载安装热门 AI 工具，会员专享</p>
      </div>
      <div style={{ position: 'relative' }}>
        {/* 付费墙模糊遮罩 */}
        <div style={{ position: 'absolute', inset: 0, background: 'rgba(10,8,32,0.6)', backdropFilter: 'blur(8px)', zIndex: 10, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', borderRadius: '12px' }}>
          <div style={{ fontSize: '48px', marginBottom: '12px' }}>🔒</div>
          <h2 style={{ fontSize: '20px', fontWeight: 600, marginBottom: '8px' }}>会员专享</h2>
          <p style={{ fontSize: '14px', color: 'var(--color-text-muted)', marginBottom: '20px', textAlign: 'center' }}>升级会员即可一键下载安装<br/>解决国内访问海外工具的网络问题</p>
          <button className="btn btn-accent" style={{ fontSize: '16px', padding: '12px 32px' }}>了解会员方案</button>
        </div>
        {/* 技能卡片网格 */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '12px' }}>
          {skills.map(s => (
            <div key={s.name} className="card" style={{ opacity: 0.5 }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', marginBottom: '8px' }}>
                <h3 style={{ fontSize: '15px', fontWeight: 600 }}>{s.name}</h3>
                <span className="badge badge-purple">v2.0</span>
              </div>
              <p style={{ fontSize: '13px', color: 'var(--color-text-muted)', marginBottom: '12px' }}>{s.desc}</p>
              <div style={{ display: 'flex', gap: '12px', fontSize: '12px', color: 'var(--color-text-muted)' }}>
                <span>📦 {s.size}</span>
                <span>⬇ {s.downloads}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
