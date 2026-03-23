# LILYGO Spark NT

基于 [GPUI](https://www.gpui.rs/) 构建的跨平台桌面应用，用于 LILYGO 设备管理。

GPUI 是 [Zed 编辑器](https://github.com/zed-industries/zed) 团队开发的 GPU 加速 UI 框架（Apache 2.0），使用 Rust 编写，支持 macOS（Metal）和 Windows（DirectX）。

## 功能

- **Dashboard** — 设备概览、统计卡片、操作按钮
- **Devices** — 设备列表与在线状态
- **Firmware** — 固件管理（开发中）
- **Settings** — 应用配置
- **Output Panel** — 实时日志输出
- **Status Bar** — 底部状态信息

## 快速开始

```bash
git clone https://github.com/eggfly/LILYGO-Spark-NT.git
cd LILYGO-Spark-NT
cargo run
```

## 环境准备

### macOS

```bash
# 1. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 安装 Xcode Command Line Tools
xcode-select --install

# 3. 安装 Metal Toolchain（GPUI 编译 Metal shader 需要）
xcodebuild -downloadComponent MetalToolchain
```

### Windows

1. **安装 Rust**：下载并运行 [rustup-init.exe](https://rustup.rs)

2. **安装 Visual Studio Build Tools**：
   - 下载 [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 安装时勾选 **"Desktop development with C++"**
   - 确保包含：MSVC C++ build tools + Windows 10/11 SDK (≥ 10.0.20348.0)

3. **安装 CMake**：下载 [CMake](https://cmake.org/download/) 并添加到 PATH

> 编译时请使用 **Developer Command Prompt** 或 **Developer PowerShell**。

## 构建

```bash
# Debug（开发，编译快）
cargo run

# Release（优化，适合发布）
cargo run --release
```

### 生成 macOS .app 和 .dmg

```bash
# 手动打 .app bundle
cargo build --release
mkdir -p "dist/LILYGO Spark NT.app/Contents/MacOS"
mkdir -p "dist/LILYGO Spark NT.app/Contents/Resources"
cp target/release/lilygo-spark-nt "dist/LILYGO Spark NT.app/Contents/MacOS/"
cp resources/AppIcon.icns "dist/LILYGO Spark NT.app/Contents/Resources/"
# 然后创建 Info.plist（见 .github/workflows/build.yml 中的模板）

# 生成 .dmg（需要 brew install create-dmg）
create-dmg \
  --volname "LILYGO Spark NT" \
  --icon "LILYGO Spark NT.app" 175 190 \
  --app-drop-link 425 190 \
  "LILYGO-Spark-NT.dmg" \
  "dist/"
```

### 应用图标

- 源文件：`resources/app-icon.png`（1024x1024 PNG）
- macOS：`resources/AppIcon.icns`（由 `sips` + `iconutil` 生成）
- CI 中会自动从 PNG 生成 .icns 并打入 .app bundle

## CI/CD

GitHub Actions 自动构建，配置在 `.github/workflows/build.yml`。

| 触发方式 | 说明 |
|---------|------|
| `git push` 到 main | 自动构建 macOS + Windows，上传 artifact |
| Pull Request | 自动构建验证 |
| 手动触发 | Actions 页面 "Run workflow" |
| 推送 `v*` tag | 自动创建 GitHub Release 并上传安装包 |

### 发布新版本

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 构建产物

| 平台 | 格式 | 说明 |
|------|------|------|
| macOS Intel | `.dmg` | 包含 .app bundle + 应用图标 |
| macOS Apple Silicon | `.dmg` | 包含 .app bundle + 应用图标 |
| Windows x64 | `.zip` | 包含 .exe |

> **关于 Release job 跳过**：`release` job 仅在推送 `v*` 格式的 tag 时运行。普通 push 到 main 只会构建并上传 artifact（在 Actions 页面可下载），不会创建 GitHub Release。

## 项目结构

```
LILYGO-Spark-NT/
├── .github/workflows/build.yml    # CI: 构建 + DMG/ZIP 打包 + Release
├── Cargo.toml                     # 项目配置 + bundle 元数据
├── resources/
│   ├── app-icon.png               # 应用图标 (1024x1024 PNG)
│   └── AppIcon.icns               # macOS 图标 (自动生成)
├── src/
│   └── main.rs                    # 应用入口 + 完整 UI
├── LICENSE                        # MIT
└── README.md
```

## 技术栈

| 组件 | 说明 |
|------|------|
| [GPUI](https://crates.io/crates/gpui) 0.2.2 | GPU 加速 UI 框架 (Zed) |
| Rust 2024 Edition | 需要 rustc 1.85+ |
| Metal (macOS) | macOS GPU 渲染后端 |
| DirectX (Windows) | Windows GPU 渲染后端 |

## 参考

- **Zed 编辑器**（GPUI 的来源）：https://github.com/zed-industries/zed
  - 开源协议：编辑器 GPL / 服务端 AGPL / GPUI 框架 Apache 2.0
  - `git clone https://github.com/zed-industries/zed.git` 可参考 GPUI 用法
- **GPUI 文档**：https://docs.rs/gpui
- **GPUI 组件库**：https://github.com/longbridge/gpui-component
