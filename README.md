# LILYGO Spark NT

基于 [GPUI](https://www.gpui.rs/) 构建的跨平台桌面应用。

GPUI 是 [Zed 编辑器](https://github.com/zed-industries/zed) 团队开发的 GPU 加速 UI 框架，使用 Rust 编写，支持 macOS（Metal）和 Windows（DirectX）。

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
   - 确保包含：
     - MSVC v143+ C++ x64/x86 build tools
     - Windows 10/11 SDK（最低版本 10.0.20348.0）

3. **安装 CMake**：下载 [CMake](https://cmake.org/download/) 并添加到 PATH

> 编译时请使用 **Developer Command Prompt** 或 **Developer PowerShell**。

## 构建

```bash
# Debug 模式（开发用，编译快）
cargo run

# Release 模式（优化，适合发布）
cargo run --release
```

首次编译会下载约 200+ 个 crate 依赖，耗时 2-5 分钟，后续增量编译很快。

## CI/CD

项目配置了 GitHub Actions，支持：

| 触发方式 | 说明 |
|---------|------|
| `git push` 到 main | 自动构建 macOS + Windows |
| Pull Request | 自动构建验证 |
| 手动触发 (workflow_dispatch) | Actions 页面手动运行 |
| 推送 `v*` tag | 自动创建 Release 并上传安装包 |

### 发布新版本

```bash
git tag v0.1.0
git push origin v0.1.0
```

构建产物：
- `LILYGO-Spark-NT-macos-x86_64-apple-darwin.zip` — macOS Intel (.app)
- `LILYGO-Spark-NT-macos-aarch64-apple-darwin.zip` — macOS Apple Silicon (.app)
- `LILYGO-Spark-NT-windows-x86_64-pc-windows-msvc.zip` — Windows x64 (.exe)

## 项目结构

```
LILYGO-Spark-NT/
├── .github/workflows/build.yml   # CI: 构建 + 打包 + 发布
├── Cargo.toml                    # 项目配置
├── src/
│   └── main.rs                   # 应用入口 + UI
├── LICENSE                       # MIT
└── README.md
```

## 技术栈

| 组件 | 说明 |
|------|------|
| [GPUI](https://crates.io/crates/gpui) 0.2.2 | GPU 加速 UI 框架 |
| Rust 2024 Edition | 需要 rustc 1.85+ |
| Metal (macOS) | macOS GPU 渲染后端 |
| DirectX (Windows) | Windows GPU 渲染后端 |

## 参考项目

- **Zed 编辑器**（GPUI 的来源）：https://github.com/zed-industries/zed
  - 开源协议：编辑器 GPL / 服务端 AGPL / GPUI 框架 Apache 2.0
  - 77,000+ Stars，490+ Contributors
  - 可以 clone 下来参考 GPUI 的用法：`git clone https://github.com/zed-industries/zed.git`
- **GPUI 文档**：https://docs.rs/gpui
- **GPUI 入门书**：https://matinaniss.github.io/gpui-book/getting-started/index.html
- **GPUI 组件库**：https://github.com/longbridge/gpui-component
