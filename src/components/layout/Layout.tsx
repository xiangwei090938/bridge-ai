import React from 'react'
import { NavLink } from 'react-router-dom'

const navItems = [
  { path: '/', label: '仪表盘', icon: '🏠' },
  { path: '/models', label: '模型管理', icon: '🧠' },
  { path: '/tools', label: '工具集成', icon: '🔧' },
  { path: '/skill-store', label: '技能商店', icon: '🎯' },
  { path: '/chat', label: 'AI 聊天', icon: '💬' },
  { path: '/settings', label: '设置', icon: '⚙️' },
]

const styles: Record<string, React.CSSProperties> = {
  sidebar: {
    width: 'var(--sidebar-width)',
    height: '100vh',
    background: 'var(--color-surface)',
    borderRight: '1px solid var(--color-border)',
    display: 'flex',
    flexDirection: 'column',
    flexShrink: 0,
  },
  logo: {
    padding: '20px',
    borderBottom: '1px solid var(--color-border)',
    display: 'flex',
    alignItems: 'center',
    gap: '10px',
  },
  logoIcon: {
    width: '32px',
    height: '32px',
    background: 'linear-gradient(135deg, #1D4ED8, #7C3AED)',
    borderRadius: '8px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '16px',
    fontWeight: 'bold',
    color: 'white',
  },
  logoText: {
    fontSize: '16px',
    fontWeight: 700,
    background: 'linear-gradient(135deg, #1D4ED8, #A78BFA)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
  },
  nav: { flex: 1, padding: '12px', display: 'flex', flexDirection: 'column', gap: '2px' },
  navLink: {
    display: 'flex', alignItems: 'center', gap: '10px',
    padding: '10px 12px', borderRadius: '8px',
    textDecoration: 'none', color: 'var(--color-text-secondary)',
    fontSize: '14px', transition: 'all 0.2s',
  },
  navLinkActive: {
    background: 'rgba(29,78,232,0.15)',
    color: 'var(--color-primary-light)',
  },
  main: {
    flex: 1,
    overflow: 'auto',
    padding: '32px',
  },
  trial: {
    padding: '16px 20px',
    borderTop: '1px solid var(--color-border)',
    display: 'flex', alignItems: 'center', gap: '8px',
    fontSize: '13px', color: 'var(--color-text-muted)',
  },
}

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div style={{ display: 'flex', height: '100vh' }}>
      <aside style={styles.sidebar}>
        <div style={styles.logo}>
          <div style={styles.logoIcon}>B</div>
          <span style={styles.logoText}>Bridge AI</span>
        </div>
        <nav style={styles.nav}>
          {navItems.map(item => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === '/'}
              style={({ isActive }) => ({
                ...styles.navLink,
                ...(isActive ? styles.navLinkActive : {}),
              })}
            >
              <span>{item.icon}</span>
              <span>{item.label}</span>
            </NavLink>
          ))}
        </nav>
        <div style={styles.trial}>
          <span>⏱</span>
          <span>试用剩余 <strong style={{ color: 'var(--color-warning)' }}>30天</strong></span>
        </div>
      </aside>
      <main style={styles.main}>{children}</main>
    </div>
  )
}
