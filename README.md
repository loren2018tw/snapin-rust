# snapin-rust

這是一個基於 **Tauri v2** + **Vue 3** + **TypeScript** 開發的桌面工具應用程式。本專案提供了一個靈活的懸浮工具欄，支援繪圖板功能以及滑鼠指標穿透模式，適用於視訊教學、簡報標註等場景。

## 🚀 主要功能

- **獨立懸浮工具欄**：隨時保持在最上層，支援拖拽移動。
- **繪圖板模式 (Whiteboard)**：
  - 提供多種繪圖工具（畫筆、直線、圓形、矩形、橡皮擦）。
  - 支援顏色與筆觸粗細調整。
  - 背景可切換為全白或透明。
- **滑鼠指標模式 (Mouse Pointer)**：
  - 支援 **點擊穿透 (Click-through)**，讓您在標註的同時仍能操作底層視窗。
- **跨平台支援**：支援 Windows、Linux (AppImage) 及 macOS。

## 🛠 技術棧

- **核心**：[Tauri v2](https://tauri.app/) (Rust)
- **前端架構**：[Vue 3](https://vuejs.org/)
- **UI 元件庫**：[Vuetify 3](https://vuetifyjs.com/)
- **樣式**：Vanilla CSS
- **構建工具**：Vite

## 💻 開發指南

### 環境要求

- **Node.js**: LTS 版本 (建議 v18+)
- **Rust**: [Rustup](https://rustup.rs/) (Stable 渠道)
- **pnpm**: `npm install -g pnpm`

### 安裝與啟動

1. **安裝依賴**：

   ```bash
   pnpm install
   ```

2. **啟動開發模式**：
   ```bash
   pnpm tauri dev
   ```

### 建構與發布

#### 本地建構

- **建構 Windows 版本**：
  ```bash
  pnpm tauri build --target x86_64-pc-windows-msvc
  ```
- **建構 Linux 版本**：
  ```bash
  pnpm tauri build
  ```

#### 自動化建構 (GitHub Actions)

本專案已配置 GitHub Actions 進行自動化建構。

- **手動觸發**：在 GitHub "Actions" 標籤頁手動執行 `publish` 工作流。
- **標籤發布**：當您推送到以 `v` 開頭的 Tag 時（例如 `v0.1.0`），系統會自動開始跨平台建構並建立 Release 草稿。
  ```bash
  git tag v0.1.0
  git push origin v0.1.0
  ```

## 🛠 排除故障 (Linux)

如果您在 Linux 上遇到建構或運行問題，請確保已安裝必要的依賴：

```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## 📄 推薦 IDE 設定

- [VS Code](https://code.visualstudio.com/)
- 必裝擴充功能：
  - [Vue - Official (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
