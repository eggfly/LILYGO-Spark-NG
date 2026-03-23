# LILYGO Spark NT

基于 [GPUI](https://www.gpui.rs/) 构建的跨平台桌面应用 —— LILYGO 设备固件管理工具。

GPUI 是 [Zed 编辑器](https://github.com/zed-industries/zed)（77k+ Stars）团队开发的 GPU 加速 UI 框架，使用 Rust 编写。

## 功能

- **侧边栏导航**：Home / Devices / Firmware / Settings
- **设备管理**：查看已连接的 LILYGO 开发板
- **固件库**：浏览可用固件，一键刷写
- **状态栏**：实时显示连接状态和操作计数
- **跨平台**：macOS (Metal) + Windows (DirectX)

## 下载

前往 [Releases](https://github.com/eggfly/LILYGO-Spark-NT/releases) 下载：

| 平台 | 格式 | 说明 |
|------|------|------|
| macOS Intel | `.dmg` | 拖入 Applications 即可 |
| macOS Apple Silicon | `.dmg` | M1/M2/M3/M4 |
| Windows x64 | `.zip` | 解压即用 |

## 从源码构建

### 前置条件

**macOS：**

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Xcode Command Line Tools
xcode-select --install

# Metal Toolchain（GPUI 编译 Metal shader 需要）
xcodebuild -downloadComponent MetalToolchain
```

**Windows：**

1. 安装 [Rust](https://rustup.rs)
2. 安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 勾选 "Desktop development with C++"
   - 需要：MSVC C++ build tools + Windows 10/11 SDK (≥10.0.20348.0)
3. 安装 [CMake](https://cmake.org/download/)

### 编译运行

```bash
git clone https://github.com/eggfly/LILYGO-Spark-NT.git
cd LILYGO-Spark-NT

# 开发模式
cargo run

# 发布模式（优化后更流畅）
cargo run --release
```

### 创建 macOS .app 和 .dmg

```bash
# 编译
cargo build --release

# 手动创建 .app bundle
mkdir -p "dist/LILYGO Spark NT.app/Contents/MacOS"
mkdir -p "dist/LILYGO Spark NT.app/Contents/Resources"
cp target/release/lilygo-spark-nt "dist/LILYGO Spark NT.app/Contents/MacOS/"
cp resources/AppIcon.icns "dist/LILYGO Spark NT.app/Contents/Resources/"
# (然后添加 Info.plist，参考 .github/workflows/build.yml)

# 生成 .dmg（需要 brew install create-dmg）
create-dmg \
  --volname "LILYGO Spark NT" \
  --icon "LILYGO Spark NT.app" 175 190 \
  --app-drop-link 425 190 \
  "LILYGO-Spark-NT.dmg" \
  "dist/"
```

## CI/CD

GitHub Actions 自动化构建：

| 触发 | 产出 |
|------|------|
| Push to `main` | macOS .dmg + Windows .zip（artifact） |
| Pull Request | 构建验证 |
| 手动触发 | Actions 页面 "Run workflow" |
| Push `v*` tag | 创建 Release + 上传安装包 |

**发布新版本：**

```bash
git tag v0.1.0
git push origin v0.1.0
```

## 项目结构

```
LILYGO-Spark-NT/
├── .github/workflows/build.yml   # CI: macOS .dmg + Windows .zip + Release
├── Cargo.toml                    # 项目配置 + bundle 元数据
├── src/
│   └── main.rs                   # 应用：侧栏 + 多页面 + 状态栏
├── resources/
│   ├── app-icon.png              # 应用图标源文件 (1024x1024)
│   └── AppIcon.icns              # macOS 图标 (自动生成)
├── LICENSE                       # MIT
└── README.md
```

## 技术栈

| 组件 | 说明 |
|------|------|
| [GPUI](https://crates.io/crates/gpui) 0.2.2 | GPU 加速 UI 框架 (Apache 2.0) |
| Rust 2024 Edition | rustc 1.85+ |
| Metal | macOS 渲染后端 |
| DirectX 11 | Windows 渲染后端 |

## 参考

- **Zed 编辑器** (GPUI 来源)：https://github.com/zed-industries/zed
  - 开源协议：编辑器 GPL / GPUI 框架 Apache 2.0
  - `git clone https://github.com/zed-industries/zed.git`
- **GPUI 文档**：https://docs.rs/gpui
- **GPUI 入门书**：https://matinaniss.github.io/gpui-book/getting-started/index.html
- **GPUI 组件库**：https://github.com/longbridge/gpui-component

## License

MIT
