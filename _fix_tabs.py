import sys
sys.stdout.reconfigure(encoding='utf-8')

with open('src/routes/ToolsView.tsx', 'r', encoding='utf-8') as f:
    content = f.read()

# Add back CATEGORIES with only "全部" and "桌面端"
content = content.replace(
    'const TOOL_ICONS',
    'const CATEGORIES = ["全部", "桌面端"];\n\nconst TOOL_ICONS'
)

# Add back selectedCategory state
content = content.replace(
    '  const [selectedTool, setSelectedTool]',
    '  const [selectedCategory, setSelectedCategory] = useState("全部");\n  const [selectedTool, setSelectedTool]'
)

# Add back filteredTools logic
content = content.replace(
    '  const filteredTools = tools;',
    '''  const filteredTools = selectedCategory === "全部"
    ? tools
    : tools.filter(t => t.category === selectedCategory);'''
)

# Add back category tabs JSX
import re
tabs_jsx = '''        {/* Category Tabs */}
        <div className="tools-category-tabs">
          {CATEGORIES.map(cat => (
            <button
              key={cat}
              className={"cat-tab" + (selectedCategory === cat ? " active" : "")}
              onClick={() => setSelectedCategory(cat)}
            >
              {cat}
            </button>
          ))}
        </div>

'''

# Find where to insert (after panel-left div)
content = content.replace(
    '        {/* Tool Cards Grid */}\n',
    tabs_jsx + '        {/* Tool Cards Grid */}\n'
)

with open('src/routes/ToolsView.tsx', 'w', encoding='utf-8') as f:
    f.write(content)

print('Restored CATEGORIES with only 全部 and 桌面端')

# Verify
if 'CATEGORIES = ["全部", "桌面端"]' in content:
    print('OK: CATEGORIES correct')
else:
    print('FAIL: CATEGORIES not found or incorrect')
if 'tools-category-tabs' in content:
    print('OK: tabs JSX restored')
else:
    print('FAIL: tabs JSX missing')