import React from 'react'
import { Routes, Route, NavLink } from 'react-router-dom'
import ToolsView from './routes/ToolsView'
import SkillMarket from './routes/SkillMarket'
import PremiumView from './routes/PremiumView'
import Chat from './routes/Chat'
import Models from './pages/Models'
import Settings from './pages/Settings'
import Tutorial from './pages/Tutorial'
import News from './pages/News'
import Onboarding from './routes/Onboarding'
import { getSetting } from './services/api'

export default function App() {
  const [onboarded, setOnboarded] = React.useState<boolean | null>(null)
  const [tauriError, setTauriError] = React.useState<string | null>(null)

  React.useEffect(() => {
    getSetting('onboarding_completed')
      .then(v => setOnboarded(v === 'true'))
      .catch(err => {
        console.error('[Bridge AI] Failed to load setting:', err)
        // 如果是 Tauri 连接错误，显示提示
        if (err.message && err.message.includes('Tauri')) {
          setTauriError(err.message)
        } else {
          // 其他错误，默认已完成 onboarding
          setOnboarded(true)
        }
      })
  }, [])

  // Tauri 连接错误提示
  if (tauriError) {
    return (
      <div className='error-screen'>
        <div className='error-icon'>⚠</div>
        <h2>Tauri 后端未连接</h2>
        <p>{tauriError}</p>
        <div className='error-actions'>
          <button onClick={() => window.location.reload()}>重试</button>
          <p className='error-hint'>
            请使用桌面模式运行：<br/>
            <code>npm run tauri:dev</code>
          </p>
        </div>
      </div>
    )
  }

  if (onboarded === null) {
    return (
      <div className='loading-screen'>
        <div className='loading-logo'>B</div>
        <div className='loading-spinner' />
      </div>
    )
  }

  if (!onboarded) {
    return <Onboarding onComplete={() => setOnboarded(true)} />
  }

  return (
    <div className='app-layout'>
      <aside className='sidebar'>
        <div className='sidebar-logo'>
          <div className='sidebar-logo-icon'>B</div>
        </div>

        <nav className='sidebar-nav'>
          <NavLink to='/tools' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='应用管理'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <rect x='2' y='2' width='8' height='8' rx='2'/>
              <rect x='14' y='2' width='8' height='8' rx='2'/>
              <rect x='2' y='14' width='8' height='8' rx='2'/>
              <rect x='14' y='14' width='8' height='8' rx='2'/>
            </svg>
            <span className='nav-label'>应用管理</span>
          </NavLink>

          <NavLink to='/chat' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='AI 对话'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <path d='M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z' />
            </svg>
            <span className='nav-label'>AI 对话</span>
          </NavLink>

          <NavLink to='/models' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='模型管理'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <rect x='2' y='2' width='20' height='8' rx='2' ry='2' />
              <rect x='2' y='14' width='20' height='8' rx='2' ry='2' />
              <line x1='6' y1='6' x2='6.01' y2='6' />
              <line x1='6' y1='18' x2='6.01' y2='18' />
            </svg>
            <span className='nav-label'>模型管理</span>
          </NavLink>

          <NavLink to='/skill-store' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='技能商店'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <rect x='3' y='3' width='7' height='7' rx='1' />
              <rect x='14' y='3' width='7' height='7' rx='1' />
              <rect x='14' y='14' width='7' height='7' rx='1' />
              <rect x='3' y='14' width='7' height='7' rx='1' />
            </svg>
            <span className='nav-label'>技能商店</span>
          </NavLink>

          <NavLink to='/premium' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='高级版'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <polygon points='12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2' />
            </svg>
            <span className='nav-label'>高级版</span>
          </NavLink>

          <NavLink to='/news' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='AI 资讯'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <path d='M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z' />
              <polyline points='14 2 14 8 20 8' />
              <line x1='16' y1='13' x2='8' y2='13' />
              <line x1='16' y1='17' x2='8' y2='17' />
              <polyline points='10 9 9 9 8 9' />
            </svg>
            <span className='nav-label'>AI 资讯</span>
          </NavLink>

          <NavLink to='/tutorial' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='使用教程'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <path d='M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z' />
              <path d='M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z' />
            </svg>
            <span className='nav-label'>使用教程</span>
          </NavLink>
        </nav>

        <div className='sidebar-footer'>
          <NavLink to='/settings' className={({ isActive }) => 'nav-item' + (isActive ? ' active' : '')}
            title='设置'>
            <svg className='nav-icon' viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
              <circle cx='12' cy='12' r='3' />
              <path d='M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z' />
            </svg>
            <span className='nav-label'>设置</span>
          </NavLink>
        </div>
      </aside>

      <main className='main-content'>
        <Routes>
          <Route path='/' element={<ToolsView />} />
          <Route path='/tools' element={<ToolsView />} />
          <Route path='/chat' element={<Chat />} />
          <Route path='/models' element={<Models />} />
          <Route path='/skill-store' element={<SkillMarket />} />
          <Route path='/premium' element={<PremiumView />} />
          <Route path='/news' element={<News />} />
          <Route path='/tutorial' element={<Tutorial />} />
          <Route path='/settings' element={<Settings />} />
        </Routes>
      </main>
    </div>
  )
}

