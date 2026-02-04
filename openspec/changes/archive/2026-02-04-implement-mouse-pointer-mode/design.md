## Context

目前專案使用 Tauri v2 版本。主視窗（`main`）負責繪圖，工具列視窗（`toolbar`）負責切換工具。所有的鼠標穿透邏輯需要作用於 `main` 視窗，且不影響 `toolbar` 視窗的互動性。

## Goals / Non-Goals

**Goals:**

- 實現 `main` 視窗在「滑鼠指標」模式下的點擊穿透。
- 確保「滑鼠指標」與「白板模式」互斥。
- 提供適當的游標視覺回饋。

**Non-Goals:**

- 不改變原本的繪圖邏輯。
- 不實作多個繪圖圖層。

## Decisions

### 1. 使用 Tauri 視窗 API 進行穿透 (Decision: Tauri `setIgnoreCursorEvents`)

- **方案**：在 `App.vue` 中（作為 `main` 視窗的根元件），當偵測到工具切換為 `Mouse Pointer` 時，呼叫 `currentWindow.setIgnoreCursorEvents(true)`。
- **原因**：這是 Tauri 提供的原生支援，能跨平台（Windows/Linux/macOS）實現點擊穿透。
- **替代方案**：呼叫 Rust 端 command 進行設置。但 JS API 已足夠且實作更簡單。

### 2. 管理游標外觀 (Decision: CSS Cursor Override)

- **方案**：在 `DrawingBoard.vue` 或 `App.vue` 的 `container` 加上動態 class。當為 `Mouse Pointer` 時，`cursor: default !important`。
- **原因**：即使視窗穿透，當滑鼠懸停在 `main` 視窗上方時，瀏覽器仍會渲染游標樣式（若未穿透前）。提供系統游標視覺回饋能增加易用性。

### 3. 邏輯觸發位置 (Decision: Centralized in `App.vue`)

- **方案**：由 `App.vue` 統一監聽 `tool-changed` 事件，並根據當前 `activeTool` 與 `isWhiteboardMode` 決定是否啟用穿透。
- **原因**：`App.vue` 已經有相關的監聽器和狀態，統一處理能確保視窗狀態的一致性。

## Risks / Trade-offs

- **[Risk] Linux 點擊穿透相容性** → **Mitigation**: 參考先前對 Linux 的修復經驗，確保 `main` 與 `toolbar` 視窗完全分離，避免互相干擾其 `ignore_mouse_events` 狀態。
- **[Trade-off] 效能** → 定期切換狀態不會對效能造成顯著影響，因為 API 呼叫是低頻率的。
