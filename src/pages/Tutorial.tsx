import React, { useState } from 'react';

function VideoPlayer() {
  const [error, setError] = useState(false);

  return (
    <div className='video-wrapper'>
      <video
        controls
        preload='metadata'
        poster='/videos/bridge-ai-tutorial-poster.jpg'
        onError={() => setError(true)}
        onLoadedData={() => setError(false)}
      >
        <source src='/videos/bridge-ai-tutorial.webm' type='video/webm' />
        <source src='/videos/bridge-ai-tutorial.mp4' type='video/mp4' />
        您的浏览器不支持视频播放。
      </video>
      {error && (
        <div className='video-fallback'>
          <p>当前环境无法直接播放视频，请点击下方按钮用浏览器或系统播放器观看。</p>
          <a
            href='/videos/bridge-ai-tutorial.webm'
            target='_blank'
            rel='noopener noreferrer'
            className='btn-primary'
          >
            打开视频教程
          </a>
        </div>
      )}
    </div>
  );
}

export default function Tutorial() {
  return (
    <div className='tutorial-page'>
      {/* Hero Header */}
      <div className='tutorial-hero'>
        <div className='hero-content'>
        <div className='hero-icon'>
          <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
            <path d='M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z' />
            <path d='M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z' />
          </svg>
        </div>
        <h1>Bridge AI 使用教程</h1>
        <p className='hero-subtitle'>从零开始，快速掌握 AI 工具管理</p>
        <div className='hero-badges'>
          <span className='badge'>5 分钟上手</span>
          <span className='badge'>图文并茂</span>
          <span className='badge'>新手友好</span>
        </div>
      </div>

      <div className='tutorial-body'>
        {/* 目录导航 */}
        <nav className='tutorial-toc'>
          <h3>目录</h3>
          <ul>
            <li><a href='#video-tutorial'>视频教程</a></li>
            <li><a href='#quick-start'>快速开始</a></li>
            <li><a href='#model-management'>模型管理</a></li>
            <li><a href='#app-management'>应用管理</a></li>
            <li><a href='#ai-chat'>AI 对话</a></li>
            <li><a href='#skill-store'>技能商店</a></li>
            <li><a href='#faq'>常见问题</a></li>
          </ul>
        </nav>

        <main className='tutorial-main'>
          {/* 视频教程 */}
          <section id='video-tutorial' className='tutorial-section video-section section-video'>
            <div className='section-header'>
              <div className='section-icon' style={{ background: 'linear-gradient(135deg, #00D4FF 0%, #7C3AED 100%)' }}>
                <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                  <polygon points='5 3 19 12 5 21 5 3' />
                </svg>
              </div>
              <div>
                <h2>视频教程</h2>
                <p className='section-desc'>90 秒快速了解 Bridge AI 的核心用法</p>
              </div>
            </div>
            <VideoPlayer />
          </section>

        {/* 快速开始 */}
        <section id='quick-start' className='tutorial-section section-start'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#10A37F' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <path d='M13 2L3 14h9l-1 8 10-12h-9l1-8z' />
              </svg>
            </div>
            <div>
              <h2>快速开始</h2>
              <p className='section-desc'>3 步完成初始配置，立即开始使用</p>
            </div>
          </div>

          <div className='steps-grid'>
            <div className='step-card'>
              <div className='step-badge'>01</div>
              <h3>选择供应商</h3>
              <p>在模型管理页面右侧，浏览并选择你的 AI 模型供应商</p>
              <div className='step-tags'>
                <span>OpenAI</span>
                <span>DeepSeek</span>
                <span>Anthropic</span>
              </div>
            </div>

            <div className='step-card'>
              <div className='step-badge'>02</div>
              <h3>配置 API Key</h3>
              <p>填写模型 ID、API 地址和密钥，点击添加配置</p>
              <div className='step-tags'>
                <span>模型 ID</span>
                <span>Base URL</span>
                <span>API Key</span>
              </div>
            </div>

            <div className='step-card'>
              <div className='step-badge'>03</div>
              <h3>测试连接</h3>
              <p>点击测试按钮验证配置是否正确，成功后即可使用</p>
              <div className='step-tags'>
                <span>一键测试</span>
                <span>实时反馈</span>
              </div>
            </div>
          </div>

          <div className='info-callout'>
            <div className='callout-icon'>💡</div>
            <div>
              <strong>新手提示</strong>
              <p>如果你还没有 API Key，点击供应商旁的"打开官网申请 API"按钮，前往官网注册获取。大多数供应商都提供免费额度。</p>
            </div>
          </div>
        </section>

        {/* 模型管理 */}
        <section id='model-management' className='tutorial-section section-model'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#4F46E5' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <rect x='2' y='2' width='20' height='8' rx='2' ry='2' />
                <rect x='2' y='14' width='20' height='8' rx='2' ry='2' />
                <line x1='6' y1='6' x2='6.01' y2='6' />
                <line x1='6' y1='18' x2='6.01' y2='18' />
              </svg>
            </div>
            <div>
              <h2>模型管理</h2>
              <p className='section-desc'>统一管理所有 AI 模型的 API 密钥和配置</p>
            </div>
          </div>

          <div className='feature-grid'>
            <div className='feature-card'>
              <div className='feature-icon'>🏢</div>
              <h3>14+ 供应商支持</h3>
              <p>涵盖国内外主流 AI 供应商，包括火山引擎、百度千帆、阿里百炼、腾讯混元、OpenAI、Anthropic、Google Gemini 等</p>
            </div>

            <div className='feature-card'>
              <div className='feature-icon'>🔑</div>
              <h3>安全存储</h3>
              <p>API Key 使用系统级密钥链加密存储，支持 AES-256-GCM 加密，确保密钥安全</p>
            </div>

            <div className='feature-card'>
              <div className='feature-icon'>⚡</div>
              <h3>一键同步</h3>
              <p>配置完成后，可一键同步到 ChatGPT、Claude、Cursor 等工具，无需手动修改配置文件</p>
            </div>

            <div className='feature-card'>
              <div className='feature-icon'>🔍</div>
              <h3>连接测试</h3>
              <p>实时测试 API 连接状态，快速定位配置问题，确保模型可用</p>
            </div>
          </div>

          <div className='workflow-diagram'>
            <h3>配置流程</h3>
            <div className='workflow-steps'>
              <div className='workflow-step'>
                <div className='workflow-dot'></div>
                <div className='workflow-content'>
                  <h4>选择供应商</h4>
                  <p>从右侧列表选择模型供应商</p>
                </div>
              </div>
              <div className='workflow-step'>
                <div className='workflow-dot'></div>
                <div className='workflow-content'>
                  <h4>填写配置</h4>
                  <p>输入模型 ID、API 地址和密钥</p>
                </div>
              </div>
              <div className='workflow-step'>
                <div className='workflow-dot'></div>
                <div className='workflow-content'>
                  <h4>保存配置</h4>
                  <p>点击"添加配置"按钮保存</p>
                </div>
              </div>
              <div className='workflow-step'>
                <div className='workflow-dot'></div>
                <div className='workflow-content'>
                  <h4>开始使用</h4>
                  <p>模型出现在已配置列表，可在应用中使用</p>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* 应用管理 */}
        <section id='app-management' className='tutorial-section section-app'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#D97757' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <rect x='2' y='2' width='8' height='8' rx='2'/>
                <rect x='14' y='2' width='8' height='8' rx='2'/>
                <rect x='2' y='14' width='8' height='8' rx='2'/>
                <rect x='14' y='14' width='8' height='8' rx='2'/>
              </svg>
            </div>
            <div>
              <h2>应用管理</h2>
              <p className='section-desc'>检测、配置和启动本地 AI 工具</p>
            </div>
          </div>

          <div className='app-features'>
            <div className='app-feature-row'>
              <div className='app-feature-visual'>
                <div className='app-card-mock'>
                  <div className='mock-header'>
                    <div className='mock-icon'></div>
                    <div className='mock-title'>ChatGPT</div>
                  </div>
                  <div className='mock-info'>
                    <div className='mock-row'><span>模型:</span> <strong>gpt-4o</strong></div>
                    <div className='mock-row'><span>应用:</span> <strong>C:\Program Files\...</strong></div>
                    <div className='mock-row'><span>配置:</span> <strong>~/.codex/config.toml</strong></div>
                  </div>
                </div>
              </div>
              <div className='app-feature-text'>
                <h3>自动检测已安装工具</h3>
                <p>Bridge AI 会自动扫描系统，检测已安装的 AI 工具，包括：</p>
                <ul>
                  <li>ChatGPT Desktop</li>
                  <li>Claude Desktop</li>
                  <li>Cursor IDE</li>
                  <li>OpenCode Desktop</li>
                  <li>Hermes Desktop</li>
                </ul>
                <p>已安装的工具会显示应用路径和版本信息，未安装的会显示"AI 自动安装"按钮。</p>
              </div>
            </div>

            <div className='app-feature-row reverse'>
              <div className='app-feature-text'>
                <h3>一键启动应用</h3>
                <p>选择模型后，点击"启动应用"按钮即可：</p>
                <ol>
                  <li>在右侧模型面板选择要使用的模型</li>
                  <li>勾选"修改模型配置"自动同步配置</li>
                  <li>点击"启动应用"打开工具</li>
                </ol>
                <p>操作日志会实时显示每一步的执行状态。</p>
              </div>
              <div className='app-feature-visual'>
                <div className='log-mock'>
                  <div className='log-entry info'>
                    <span className='log-time'>18:31:05</span>
                    <span className='log-msg'>准备同步模型配置: DeepSeek</span>
                  </div>
                  <div className='log-entry pending'>
                    <span className='log-time'>18:31:05</span>
                    <span className='log-msg'>正在同步配置到 OpenCode...</span>
                  </div>
                  <div className='log-entry success'>
                    <span className='log-time'>18:31:05</span>
                    <span className='log-msg'>配置同步成功</span>
                  </div>
                  <div className='log-entry success'>
                    <span className='log-time'>18:31:05</span>
                    <span className='log-msg'>启动成功: Launched OpenCode</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* AI 对话 */}
        <section id='ai-chat' className='tutorial-section section-chat'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#6C5CE7' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <path d='M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z' />
              </svg>
            </div>
            <div>
              <h2>AI 对话</h2>
              <p className='section-desc'>内置 AI 聊天面板，直接与模型对话</p>
            </div>
          </div>

          <div className='chat-features-grid'>
            <div className='chat-feature'>
              <div className='chat-feature-icon'>💬</div>
              <h3>多轮对话</h3>
              <p>自动保留上下文，支持连续对话，无需重复说明背景</p>
            </div>

            <div className='chat-feature'>
              <div className='chat-feature-icon'>⌨️</div>
              <h3>流式输出</h3>
              <p>打字机效果实时显示 AI 回复，无需等待完整响应</p>
            </div>

            <div className='chat-feature'>
              <div className='chat-feature-icon'>🎨</div>
              <h3>代码高亮</h3>
              <p>自动识别代码块并语法高亮，支持一键复制</p>
            </div>

            <div className='chat-feature'>
              <div className='chat-feature-icon'>🔄</div>
              <h3>重新生成</h3>
              <p>对回复不满意？点击重新生成获取新的答案</p>
            </div>

            <div className='chat-feature'>
              <div className='chat-feature-icon'>📋</div>
              <h3>消息复制</h3>
              <p>一键复制 AI 回复内容，方便粘贴到其他工具</p>
            </div>

            <div className='chat-feature'>
              <div className='chat-feature-icon'></div>
              <h3>新建会话</h3>
              <p>随时开始新的对话，保持对话整洁有序</p>
            </div>
          </div>
        </section>

        {/* 技能商店 */}
        <section id='skill-store' className='tutorial-section section-skill'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#8B5CF6' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <rect x='3' y='3' width='7' height='7' rx='1' />
                <rect x='14' y='3' width='7' height='7' rx='1' />
                <rect x='14' y='14' width='7' height='7' rx='1' />
                <rect x='3' y='14' width='7' height='7' rx='1' />
              </svg>
            </div>
            <div>
              <h2>技能商店</h2>
              <p className='section-desc'>扩展 AI 工具能力的技能包市场</p>
            </div>
          </div>

          <div className='skill-showcase'>
            <div className='skill-card'>
              <div className='skill-icon'>🎯</div>
              <h3>编程助手</h3>
              <p>代码补全、Bug 修复、代码审查</p>
              <span className='skill-tag'>免费</span>
            </div>

            <div className='skill-card'>
              <div className='skill-icon'>✍️</div>
              <h3>写作助手</h3>
              <p>文章润色、语法检查、风格调整</p>
              <span className='skill-tag'>免费</span>
            </div>

            <div className='skill-card premium'>
              <div className='skill-icon'>🎨</div>
              <h3>设计助手</h3>
              <p>UI 设计、配色方案、图标生成</p>
              <span className='skill-tag premium'>会员</span>
            </div>

            <div className='skill-card premium'>
              <div className='skill-icon'>📊</div>
              <h3>数据分析</h3>
              <p>数据可视化、统计分析、报告生成</p>
              <span className='skill-tag premium'>会员</span>
            </div>
          </div>

          <div className='info-callout'>
            <div className='callout-icon'>⭐</div>
            <div>
              <strong>会员特权</strong>
              <p>会员用户可以免费下载所有技能包，包括高级技能。非会员用户可使用基础免费技能。</p>
            </div>
          </div>
        </section>

        {/* 常见问题 */}
        <section id='faq' className='tutorial-section section-faq'>
          <div className='section-header'>
            <div className='section-icon' style={{ background: '#EC4899' }}>
              <svg width="24" height="24" viewBox='0 0 24 24' fill='none' stroke='currentColor' strokeWidth='2'>
                <circle cx='12' cy='12' r='10' />
                <path d='M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3' />
                <line x1='12' y1='17' x2='12.01' y2='17' />
              </svg>
            </div>
            <div>
              <h2>常见问题</h2>
              <p className='section-desc'>快速解决使用中遇到的疑问</p>
            </div>
          </div>

          <div className='faq-list'>
            <div className='faq-item'>
              <div className='faq-question'>
                <span className='faq-q'>Q</span>
                <h3>如何获取 API Key？</h3>
              </div>
              <div className='faq-answer'>
                <p>在"模型管理"页面右侧，点击供应商旁的"打开官网申请 API"按钮，前往供应商官网注册并获取 API Key。大多数供应商都提供免费额度，适合新手体验。</p>
              </div>
            </div>

            <div className='faq-item'>
              <div className='faq-question'>
                <span className='faq-q'>Q</span>
                <h3>为什么检测不到已安装的工具？</h3>
              </div>
              <div className='faq-answer'>
                <p>Bridge AI 会扫描常见的安装路径。如果工具安装在自定义位置，可能无法自动检测。建议将工具安装到默认路径，或点击"刷新"按钮重新扫描。</p>
              </div>
            </div>

            <div className='faq-item'>
              <div className='faq-question'>
                <span className='faq-q'>Q</span>
                <h3>配置同步失败怎么办？</h3>
              </div>
              <div className='faq-answer'>
                <p>首次使用某个工具时，需要先手动打开一次，让它生成默认配置文件。然后 Bridge AI 才能同步配置。如果仍有问题，请检查文件权限。</p>
              </div>
            </div>

            <div className='faq-item'>
              <div className='faq-question'>
                <span className='faq-q'>Q</span>
                <h3>如何切换模型？</h3>
              </div>
              <div className='faq-answer'>
                <p>在"应用管理"页面右侧的模型面板中，点击不同的模型即可切换。切换后点击"启动应用"，Bridge AI 会自动将新模型的配置同步到工具。</p>
              </div>
            </div>

            <div className='faq-item'>
              <div className='faq-question'>
                <span className='faq-q'>Q</span>
                <h3>会员功能有哪些？</h3>
              </div>
              <div className='faq-answer'>
                <p>会员用户可享受：技能商店全部技能免费下载、批量配置导入导出、一键本地模型部署、自定义 Prompt 模板库、C 盘缓存自动迁移、会话批量导出、快捷键全局快捷唤起等高级功能。</p>
              </div>
            </div>
          </div>
        </section>

        {/* 页脚 */}
        <div className='tutorial-footer'>
          <div className='footer-content'>
            <h3>需要更多帮助？</h3>
            <p>访问官方网站、加入用户交流群或提交问题反馈</p>
            <div className='footer-actions'>
              <button className='btn-primary'>访问官网</button>
              <button className='btn-secondary'>加入交流群</button>
            </div>
          </div>
        </div>
      </main>
      </div>
    </div>
    </div>
  );
}

