import React, { useState } from 'react'

interface Msg { role: string; content: string }

export function Chat() {
  const [msgs, setMsgs] = useState<Msg[]>([])
  const [input, setInput] = useState('')

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 64px)' }}>
      <div style={{ marginBottom: '20px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 700, marginBottom: '4px' }}>AI 聊天 💬</h1>
        <p style={{ fontSize: '14px', color: 'var(--color-text-muted)' }}>选择模型，开始对话</p>
      </div>

      {msgs.length === 0 ? (
        <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
          <div className="card" style={{ textAlign: 'center', padding: '40px', maxWidth: '400px' }}>
            <div style={{ fontSize: '48px', marginBottom: '16px' }}>🔒</div>
            <h3 style={{ fontSize: '18px', fontWeight: 600, marginBottom: '8px' }}>会员专属功能</h3>
            <p style={{ fontSize: '14px', color: 'var(--color-text-muted)', marginBottom: '20px' }}>
              升级会员即可使用 AI 聊天面板，支持多模型同屏对比
            </p>
            <button className="btn btn-accent" style={{ fontSize: '15px', padding: '10px 24px' }}>了解会员方案</button>
          </div>
        </div>
      ) : (
        <div style={{ flex: 1, overflow: 'auto', marginBottom: '16px' }}>
          {msgs.map((m, i) => (
            <div key={i} style={{ marginBottom: '12px', textAlign: m.role === 'user' ? 'right' : 'left' }}>
              <div className="card" style={{ display: 'inline-block', maxWidth: '70%', textAlign: 'left' }}>
                {m.content}
              </div>
            </div>
          ))}
        </div>
      )}

      <div style={{ display: 'flex', gap: '8px', padding: '12px', background: 'var(--color-surface)', borderRadius: '12px', border: '1px solid var(--color-border)' }}>
        <input
          style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'var(--color-text)', fontSize: '14px' }}
          placeholder="输入消息..."
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => { if (e.key === 'Enter' && input.trim()) { setMsgs([...msgs, { role: 'user', content: input }]); setInput('') } }}
        />
        <button className="btn btn-primary" disabled>发送</button>
      </div>
    </div>
  )
}
