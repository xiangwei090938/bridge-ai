path = r'D:\Workspace\projects\project_002_EchoBird克隆-AI管理中枢\03-开发代码\bridge-ai\src\pages\Tutorial.tsx'
with open(path, 'r', encoding='utf-8') as f:
    c = f.read()

# wrap hero content
c = c.replace(
    "<div className='tutorial-hero'>\n        <div className='hero-icon'>",
    "<div className='tutorial-hero'>\n        <div className='hero-content'>\n        <div className='hero-icon'>"
)
c = c.replace(
    "        </div>\n\n      <div className='tutorial-body'>",
    "        </div>\n        </div>\n\n      <div className='tutorial-body'>"
)

# insert main wrapper and video section after toc
video_section = """        </nav>

        <main className='tutorial-main'>
          {/* 视频教程 */}
          <section id='video-tutorial' className='tutorial-section video-section'>
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
            <div className='video-wrapper'>
              <video controls width='100%' poster='/videos/bridge-ai-tutorial-poster.jpg'>
                <source src='/videos/bridge-ai-tutorial.mp4' type='video/mp4' />
                您的浏览器不支持视频播放。
              </video>
            </div>
          </section>

        {/* 快速开始 */}"""
c = c.replace(
    "        </nav>\n\n        {/* 快速开始 */}",
    video_section
)

# close main before tutorial-body close
c = c.replace(
    "      </div>\n    </div>\n  );",
    "      </main>\n      </div>\n    </div>\n  );",
    1
)

with open(path, 'w', encoding='utf-8') as f:
    f.write(c)
print('updated')
