import sys
sys.stdout.reconfigure(encoding='utf-8')

with open('src/pages/Models.tsx', 'r', encoding='utf-8') as f:
    content = f.read()

# Add Tauri shell import at the top
old_imports = 'import { listProviders, listModels, addModel, deleteModel, updateApiKey, testConnection } from "../services/api";'
new_imports = '''import { listProviders, listModels, addModel, deleteModel, updateApiKey, testConnection } from "../services/api";

// Open URL in system browser (works in both Tauri and browser)
async function openExternalUrl(url: string) {
  if (!url || url === "#") return;
  try {
    if (typeof window !== "undefined" && (window as any).__TAURI__) {
      const shell = await import("@tauri-apps/plugin-shell");
      await shell.open(url);
    } else {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  } catch {
    window.open(url, "_blank", "noopener,noreferrer");
  }
}'''

content = content.replace(old_imports, new_imports)

# Replace the <a> tag with a button that uses openExternalUrl
old_link = '''                    <a href={p.url || "#"} target="_blank" rel="noopener noreferrer" className="provider-list-external" onClick={(e) => e.stopPropagation()} title="打开官网">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                        <polyline points="15 3 21 3 21 9" />
                        <line x1="10" y1="14" x2="21" y2="3" />
                      </svg>
                    </a>'''

new_link = '''                    <button className="provider-list-external" onClick={(e) => { e.stopPropagation(); openExternalUrl(p.url || ""); }} title="打开官网">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                        <polyline points="15 3 21 3 21 9" />
                        <line x1="10" y1="14" x2="21" y2="3" />
                      </svg>
                    </button>'''

content = content.replace(old_link, new_link)

with open('src/pages/Models.tsx', 'w', encoding='utf-8') as f:
    f.write(content)

print('Fixed external link to use Tauri shell.open()')