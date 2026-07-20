# Bridge AI 跨平台构建指南

## 支持平台

| 平台 | 架构 | 状态 |
|------|------|------|
| Windows | x86_64 | ✅ 已验证 |
| macOS | Apple Silicon (arm64) | ✅ 支持 |
| macOS | Intel (x86_64) | ✅ 支持 |
| Linux | x86_64 | ✅ 支持 |
| Linux | arm64 | ✅ 支持 |

## 跨平台改动总结

### 1. 文件系统路径 (`util/fs.rs`)
- **Windows**: `%USERPROFILE%\.bridge-ai`
- **macOS**: `~/Library/Application Support/bridge-ai`
- **Linux**: `~/.config/bridge-ai`
- 使用 `dirs` crate 自动适配各平台标准路径

### 2. 工具检测 (`service/tool_detector.rs`)
每个工具定义包含三组路径：
- `win32`: Windows 安装路径
- `darwin`: macOS 安装路径
- `linux`: Linux 安装路径

检测策略：
| 平台 | 检测方式 |
|------|----------|
| Windows | 注册表、开始菜单、WindowsApps、PATH、进程 |
| macOS | /Applications、Homebrew、PATH、进程 |
| Linux | /usr/bin、/opt、snap、flatpak、PATH、进程 |

### 3. 配置同步 (`service/sync_engine.rs`)
- 支持 `$HOME`、`~`、`$XDG_CONFIG_HOME` 路径展开
- Windows 仍支持 `%LOCALAPPDATA%`、`%APPDATA%` 等

### 4. 打包配置 (`tauri.conf.json`)
- `targets: "all"` 自动根据平台生成对应安装包

## 构建命令

### Windows (当前开发机)
```powershell
# 开发模式
npm run tauri:dev

# 生产构建 (生成 .msi 安装包)
npm run tauri build
```
输出: `src-tauri/target/release/bundle/msi/Bridge AI_0.1.0_x64_en-US.msi`

### macOS
```bash
# 需要 macOS 开发机 + Xcode

# 安装依赖
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# 开发模式
npm run tauri:dev

# 生产构建 (生成 .dmg)
npm run tauri build -- --target universal-apple-darwin
```
输出: `src-tauri/target/universal-apple-darwin/release/bundle/dmg/Bridge AI_0.1.0_universal.dmg`

### Linux
```bash
# 需要 Linux 开发机

# 安装系统依赖 (Ubuntu/Debian)
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf

# 开发模式
npm run tauri:dev

# 生产构建 (生成 .deb / .AppImage)
npm run tauri build
```
输出:
- `src-tauri/target/release/bundle/deb/bridge-ai_0.1.0_amd64.deb`
- `src-tauri/target/release/bundle/appimage/bridge-ai_0.1.0_amd64.AppImage`

## CI/CD 自动构建 (GitHub Actions)

创建 `.github/workflows/release.yml`:

```yaml
name: Release Bridge AI

on:
  push:
    tags: ['v*']

jobs:
  release:
    strategy:
      matrix:
        include:
          - platform: windows-latest
            args: ''
          - platform: macos-latest
            args: '--target universal-apple-darwin'
          - platform: ubuntu-22.04
            args: ''
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npm run tauri build -- ${{ matrix.args }}
      - uses: actions/upload-artifact@v4
        with:
          name: bridge-ai-${{ matrix.platform }}
          path: src-tauri/target/*/release/bundle/
```

## 平台特定注意事项

### Windows
- 安装包格式: MSI
- 需要代码签名证书（可选，避免 SmartScreen 警告）
- 支持 Windows 10/11

### macOS
- 安装包格式: DMG
- 需要 Apple Developer ID 签名（否则 Gatekeeper 会阻止）
- 需要 notarization（公证）
- 支持 macOS 12+ (Monterey)
- Universal Binary 同时支持 Intel 和 Apple Silicon

### Linux
- 安装包格式: deb (Debian/Ubuntu) + AppImage (通用)
- 需要 libwebkit2gtk-4.1 (Tauri 2 要求)
- 建议同时提供 Flatpak 版本
- 支持 Ubuntu 20.04+、Debian 11+、Fedora 36+

## 测试清单

在发布前，每个平台需要验证：

- [ ] 应用能正常启动
- [ ] 配置目录正确创建（各平台路径不同）
- [ ] 数据库能正常读写
- [ ] 工具检测能发现已安装的应用
- [ ] 模型配置同步能写入正确的配置文件路径
- [ ] RSS 资讯采集正常工作
- [ ] 外部链接能用系统浏览器打开
- [ ] 窗口最小化/最大化/关闭正常
- [ ] 深色主题显示正常
