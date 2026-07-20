/**
 * Tool Detector — 真实路径扫描 + 环境变量检测
 * Runs as a Vite plugin to serve real detection data during development.
 * In Tauri mode, the Rust backend handles this natively.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Tool definitions with known install paths
const TOOL_DEFS = [
  { id: 'chatgptdesktop', name: 'ChatGPT', category: '桌面端', protocols: ['openai'], uris: ['shell:AppsFolder\\OpenAI.ChatGPT_2p2nqsd0c76g0!App'],
    paths: [
      '%LOCALAPPDATA%\\Programs\\ChatGPT\\ChatGPT.exe',
      '%APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\ChatGPT.lnk',
    ] },
  { id: 'claudedesktop', name: 'Claude Desktop', category: '桌面端', protocols: ['anthropic'], uris: ['shell:AppsFolder\\Anthropic.Claude_9nn07rvydr1pw!App'],
    paths: ['%LOCALAPPDATA%\\Programs\\Claude\\Claude.exe'] },
  { id: 'cursor', name: 'Cursor', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\cursor\\Cursor.exe', '%USERPROFILE%\\AppData\\Local\\Programs\\cursor\\Cursor.exe'] },
  { id: 'opencode', name: 'OpenCode', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\opencode\\OpenCode.exe'] },
  { id: 'claudecode', name: 'Claude Code', category: 'CLI 命令行', protocols: ['anthropic'],
    paths: ['%USERPROFILE%\\.local\\bin\\claude.exe'] },
  { id: 'hermes', name: 'Hermes', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\hermes\\Hermes.exe'] },
  { id: 'vscode', name: 'VS Code', category: 'IDE', protocols: [],
    paths: ['%LOCALAPPDATA%\\Programs\\Microsoft VS Code\\Code.exe', '%ProgramFiles%\\Microsoft VS Code\\Code.exe'] },
  { id: 'geminidesktop', name: 'Gemini Desktop', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\Gemini\\Gemini.exe'] },
  { id: 'aider', name: 'Aider', category: 'CLI 命令行', protocols: ['openai'],
    paths: ['%USERPROFILE%\\.local\\bin\\aider.exe'] },
  { id: 'codex', name: 'Codex CLI', category: 'CLI 命令行', protocols: ['openai'],
    paths: ['%USERPROFILE%\\.codex\\bin\\codex.exe'] },
  { id: 'kimicode', name: 'Kimi Code', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\KimiCode\\KimiCode.exe'] },
  { id: 'qwencode', name: 'Qwen Code', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\QwenCode\\QwenCode.exe'] },
  { id: 'trae', name: 'Trae', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\Trae\\Trae.exe'] },
  { id: 'traecn', name: 'Trae CN', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\TraeCN\\TraeCN.exe'] },
  { id: 'workbuddy', name: 'WorkBuddy', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\WorkBuddy\\WorkBuddy.exe'] },
  { id: 'zcode', name: 'Z Code', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\ZCode\\ZCode.exe'] },
  { id: 'mimocode', name: 'Mimo Code', category: 'IDE', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\MimoCode\\MimoCode.exe'] },
  { id: 'grok', name: 'Grok', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\Grok\\Grok.exe'] },
  { id: 'pi', name: 'Pi Assistant', category: '桌面端', protocols: [],
    paths: ['%LOCALAPPDATA%\\Programs\\Pi\\Pi.exe'] },
  { id: 'coffeecli', name: 'Coffee CLI', category: 'CLI 命令行', protocols: ['openai'],
    paths: ['%USERPROFILE%\\.local\\bin\\coffee.exe'] },
  { id: 'openclaw', name: 'OpenClaw', category: 'CLI 命令行', protocols: ['openai'],
    paths: ['%USERPROFILE%\\.local\\bin\\openclaw.exe'] },
  { id: 'claudescience', name: 'Claude Science', category: '科学', protocols: ['anthropic'],
    paths: ['%LOCALAPPDATA%\\Programs\\ClaudeScience\\ClaudeScience.exe'] },
  { id: 'opencodedesktop', name: 'OpenCode Desktop', category: '桌面端', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\OpenCodeDesktop\\OpenCodeDesktop.exe'] },
  { id: 'openscience', name: 'OpenScience', category: '科学', protocols: ['openai'],
    paths: ['%LOCALAPPDATA%\\Programs\\OpenScience\\OpenScience.exe'] },
  // Built-in tools (always "installed")
  { id: 'reversi', name: 'Reversi', category: '内置工具', protocols: [], paths: [], builtin: true },
  { id: 'translator', name: 'Translator', category: '内置工具', protocols: [], paths: [], builtin: true },
  { id: 'vibe-trading', name: 'Vibe Trading', category: '内置工具', protocols: [], paths: [], builtin: true },
];

function expandEnv(str) {
  return str.replace(/%([^%]+)%/g, (_, key) => process.env[key] || '');
}

function existsSync(p) {
  try {
    return fs.existsSync(p);
  } catch { return false; }
}

function scanRunningProcesses() {
  try {
    const out = execSync('powershell -Command "Get-Process | Select -ExpandProperty ProcessName -Unique"', { encoding: 'utf8', timeout: 5000 });
    return out.split('\n').map(s => s.trim().toLowerCase()).filter(Boolean);
  } catch { return []; }
}

function scanStartMenu() {
  const results = [];
  try {
    const ap = process.env.APPDATA || '';
    const dirs = [
      'C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs',
      path.join(ap, 'Microsoft\\Windows\\Start Menu\\Programs')
    ];
    for (const d of dirs) {
      if (!existsSync(d)) continue;
      for (const entry of fs.readdirSync(d, { withFileTypes: true })) {
        if (entry.name.endsWith('.lnk')) {
          results.push(path.basename(entry.name, '.lnk').toLowerCase());
        }
      }
    }
  } catch {}
  return results;
}

function scanCommonLocations() {
  const results = [];
  const roots = [
    expandEnv('%LOCALAPPDATA%\\Programs'),
    expandEnv('%ProgramFiles%'),
    expandEnv('%ProgramFiles(x86)%'),
    expandEnv('%USERPROFILE%\\AppData\\Local'),
    expandEnv('%USERPROFILE%\\.local\\bin'),
    expandEnv('%USERPROFILE%\\.codex\\bin'),
  ];
  for (const root of roots) {
    if (!root || !existsSync(root)) continue;
    try {
      for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
        const name = entry.name.toLowerCase();
        if (entry.isDirectory() || name.endsWith('.exe')) {
          results.push(name.replace('.exe', ''));
        }
      }
    } catch {}
  }
  return results;
}

function detectTools() {
  const running = scanRunningProcesses();
  const shortcuts = scanStartMenu();
  const common = scanCommonLocations();

  // Build a set of all detected indicators
  const detectedExes = new Set([...running, ...shortcuts, ...common]);
  // Also detect via known vendor directories
  for (const loc of common) {
    detectedExes.add(loc.toLowerCase());
  }

  return TOOL_DEFS.map(def => {
    let installPath = null;
    let isInstalled = def.builtin === true;

    // Check defined paths
    if (!isInstalled) {
      for (const p of def.paths) {
        const expanded = expandEnv(p);
        if (existsSync(expanded)) {
          installPath = expanded;
          isInstalled = true;
          break;
        }
      }
    }

    // Check by process name
    if (!isInstalled) {
      for (const p of def.paths) {
        const exeName = path.basename(p).toLowerCase().replace('%', '');
        const parts = p.split('\\');
        const exe = parts[parts.length - 1].toLowerCase().replace('.lnk', '.exe').replace(/%/g, '');
        // Check if the exe name (without path vars) is detected
        const rationalized = exe.replace(/\.exe$/, '');
        if (detectedExes.has(rationalized) || detectedExes.has(exe)) {
          isInstalled = true;
          break;
        }
      }
    }

    // Extra: check running process names that match the tool
    if (!isInstalled) {
      const idLow = def.id.toLowerCase();
      const nameLow = def.name.toLowerCase();
      for (const proc of running) {
        if (proc === idLow || proc === nameLow || proc.includes(idLow) || proc.includes(nameLow)) {
          isInstalled = true;
          break;
        }
      }
    }

    return {
      id: def.id,
      name: def.name,
      category: def.category,
      is_installed: isInstalled,
      install_path: installPath,
      config_path: null,
      api_protocols: def.protocols,
      has_config: def.protocols.length > 0,
      sync_supported: def.protocols.length > 0,
      launch_uri: def.uris ? def.uris[0] : null,
    };
  });
}

// Run detection once at startup
const detected = detectTools();

// Log results
console.log('[Tool Detector] Scanned', detected.length, 'tools');
const installed = detected.filter(t => t.is_installed);
console.log('[Tool Detector] Found', installed.length, 'installed:', installed.map(t => t.name).join(', '));

// Export both the detect function and the results
module.exports = { detectTools, detected };
