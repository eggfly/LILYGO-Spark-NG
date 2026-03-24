# LILYGO Spark NG - 开发与打包指南

## 开发构建

```bash
cargo build
cargo run
```

## Release 构建

```bash
cargo build --release
```

编译产物在 `target/release/lilygo-spark-ng`。

## macOS 打包 DMG

### 方法一：cargo-bundle + create-dmg（推荐）

```bash
# 一次性安装依赖
cargo install cargo-bundle
brew install create-dmg

# 生成 .app 包（自动读取 Cargo.toml 中的 [package.metadata.bundle] 配置）
cargo bundle --release

# 生成 DMG
create-dmg \
  --volname "LILYGO Spark NG" \
  --window-pos 200 120 \
  --window-size 600 400 \
  --icon-size 100 \
  --icon "LILYGO Spark NG.app" 175 190 \
  --app-drop-link 425 190 \
  "LILYGO-Spark-NG.dmg" \
  "target/release/bundle/osx/"
```

生成的 DMG 打开后会显示 .app 图标和 Applications 快捷方式，用户拖拽即可安装。

### 方法二：手动打包（与 CI 一致）

```bash
# 1. 编译
cargo build --release

# 2. 创建 .app 目录结构
APP_NAME="lilygo-spark-ng"
APP_DISPLAY_NAME="LILYGO Spark NG"
APP_BUNDLE="${APP_DISPLAY_NAME}.app"

mkdir -p "dist/${APP_BUNDLE}/Contents/MacOS"
mkdir -p "dist/${APP_BUNDLE}/Contents/Resources"
cp "target/release/${APP_NAME}" "dist/${APP_BUNDLE}/Contents/MacOS/"

# 3. 生成图标
if [ -f resources/app-icon.png ]; then
  mkdir -p AppIcon.iconset
  sips -z 16 16     resources/app-icon.png --out AppIcon.iconset/icon_16x16.png
  sips -z 32 32     resources/app-icon.png --out AppIcon.iconset/icon_16x16@2x.png
  sips -z 32 32     resources/app-icon.png --out AppIcon.iconset/icon_32x32.png
  sips -z 64 64     resources/app-icon.png --out AppIcon.iconset/icon_32x32@2x.png
  sips -z 128 128   resources/app-icon.png --out AppIcon.iconset/icon_128x128.png
  sips -z 256 256   resources/app-icon.png --out AppIcon.iconset/icon_128x128@2x.png
  sips -z 256 256   resources/app-icon.png --out AppIcon.iconset/icon_256x256.png
  sips -z 512 512   resources/app-icon.png --out AppIcon.iconset/icon_256x256@2x.png
  sips -z 512 512   resources/app-icon.png --out AppIcon.iconset/icon_512x512.png
  sips -z 1024 1024 resources/app-icon.png --out AppIcon.iconset/icon_512x512@2x.png
  iconutil -c icns AppIcon.iconset -o "dist/${APP_BUNDLE}/Contents/Resources/AppIcon.icns"
  rm -rf AppIcon.iconset
fi

# 如果已有预编译的 .icns 可以直接用
# cp resources/AppIcon.icns "dist/${APP_BUNDLE}/Contents/Resources/"

# 4. 创建 Info.plist
cat > "dist/${APP_BUNDLE}/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key>
  <string>LILYGO Spark NG</string>
  <key>CFBundleIdentifier</key>
  <string>cc.lilygo.spark-ng</string>
  <key>CFBundleVersion</key>
  <string>0.1.0</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleExecutable</key>
  <string>lilygo-spark-ng</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>LSMinimumSystemVersion</key>
  <string>10.15.7</string>
</dict>
</plist>
PLIST

# 5. 生成 DMG
brew install create-dmg
create-dmg \
  --volname "LILYGO Spark NG" \
  --window-pos 200 120 \
  --window-size 600 400 \
  --icon-size 100 \
  --icon "LILYGO Spark NG.app" 175 190 \
  --app-drop-link 425 190 \
  "LILYGO-Spark-NG.dmg" \
  "dist/"
```

## Linux 本地构建

### 安装系统依赖（Ubuntu/Debian）

```bash
sudo apt-get update
sudo apt-get install -y \
  gcc g++ cmake clang lld \
  libasound2-dev \
  libfontconfig-dev \
  libglib2.0-dev \
  libssl-dev \
  libvulkan1 \
  libwayland-dev \
  libx11-xcb-dev \
  libxkbcommon-x11-dev \
  libzstd-dev
```

### 编译和打包

```bash
cargo build --release

mkdir -p dist/lilygo-spark-ng
cp target/release/lilygo-spark-ng dist/lilygo-spark-ng/
cp resources/app-icon.png dist/lilygo-spark-ng/
cd dist
tar czf ../lilygo-spark-ng-linux-x86_64.tar.gz lilygo-spark-ng
```

## GitHub Actions CI

CI 配置在 `.github/workflows/build.yml`，**每次 push 到 main 自动构建三端并发布到 Release**。

### 构建平台

| 平台 | Target | 产物 |
|------|--------|------|
| macOS (Apple Silicon) | `aarch64-apple-darwin` | `.dmg` |
| macOS (Intel) | `x86_64-apple-darwin` | `.dmg` |
| Windows | `x86_64-pc-windows-msvc` | `.zip` |
| Linux | `x86_64-unknown-linux-gnu` | `.tar.gz` |

### 触发方式与行为

| 触发条件 | 行为 |
|----------|------|
| push 到 `main` | 编译 + 更新 `latest` prerelease（滚动更新） |
| 创建 `v*` tag | 编译 + 创建正式 GitHub Release |
| Pull Request 到 `main` | 仅编译，不发布 |
| 手动触发 (workflow_dispatch) | 编译 + 更新 `latest` prerelease |

### 发布正式版本

```bash
git tag v0.1.0
git push origin v0.1.0
```

CI 会自动创建正式 GitHub Release 并附带所有平台的安装包。

### GitHub 设置

不需要额外设置。workflow 使用自动提供的 `GITHUB_TOKEN`，配合 `permissions: contents: write` 即可创建 Release。只需确保仓库的 Actions 是启用状态（默认就是）。
