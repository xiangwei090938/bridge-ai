import React from 'react'

const sectionTitle: React.CSSProperties = { fontSize: '20px', fontWeight: 600, marginBottom: '4px' }
const sectionDesc: React.CSSProperties = { fontSize: '14px', color: 'var(--color-text-muted)', marginBottom: '20px' }
const card: React.CSSProperties = { background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: '12px', padding: '20px' }

export function Dashboard() {
  const stats = [
    { label: '已配置模型', value: '0', color: '#60A5FA' },
    { label: '已检测工具', value: '0', color: '#A78BFA' },
    { label: '可用技能', value: '0', color: '#22D3EE' },
    { label: '今日对话', value: '0', color: '#10b981' },
  ]

  return (
    <div>
      <h1 style={{ fontSize: '24px', fontWeight: 700, marginBottom: '4px' }}>仪表盘</h1>
      <p style={sectionDesc}>欢迎使用 Bridge AI 助手</p>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '16px', marginBottom: '24px' }}>
        {stats.map(s => (
          <div key={s.label} style={card}>
            <div style={{ fontSize: '28px', fontWeight: 700, color: s.color }}>{s.value}</div>
            <div style={{ fontSize: '14px', color: 'var(--color-text-secondary)', marginTop: '4px' }}>{s.label}</div>
          </div>
        ))}
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '2fr 1fr', gap: '16px' }}>
        <div style={card}>
          <h3 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '12px' }}>快速开始</h3>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            {['添加 API Key 开始使用', '检测本地 AI 工具', '浏览技能商店'].map((t, i) => (
              <div key={i} style={{ display: 'flex', alignItems: 'center', gap: '12px', padding: '12px', borderRadius: '8px', background: 'var(--color-surface-hover)', cursor: 'pointer' }}>
                <div style={{ width: '32px', height: '32px', borderRadius: '8px', background: 'linear-gradient(135deg, #1D4ED8, #7C3AED)', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '14px', color: 'white' }}>{i + 1}</div>
                <span style={{ fontSize: '14px', color: 'var(--color-text)' }}>{t}</span>
              </div>
            ))}
          </div>
        </div>
        <div style={card}>
          <h3 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '12px' }}>试用信息</h3>
          <div style={{ fontSize: '14px', color: 'var(--color-text-secondary)' }}>
            <p>免费试用剩余 <strong style={{ color: 'var(--color-warning)' }}>30天</strong></p>
            <p style={{ marginTop: '8px' }}>升级会员解锁全部功能</p>
          </div>
        </div>
      </div>
    </div>
  )
}
