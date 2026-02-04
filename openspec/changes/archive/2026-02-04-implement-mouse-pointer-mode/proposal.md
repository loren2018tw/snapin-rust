## Why

目前應用程式在繪圖模式下會攔截所有鼠標事件，導致使用者無法在保留標註的情況下操作原本的桌面視窗。實作「滑鼠指標」功能可以讓使用者在保留繪製圖形的同時，正常操作繪圖視窗下方的其他程式。

## What Changes

- 在 `SnapinToolbar.vue` 中完善「滑鼠指標」工具的切換邏輯。
- 在 `App.vue` 中監聽工具切換事件，當切換至「滑鼠指標」時，呼叫 Tauri 視窗 API `setIgnoreCursorEvents(true)` 以實現點擊穿透（click-through）。
- 當切換回其他繪圖工具時，恢復視窗的鼠標互動性。
- 確保在「白板模式」下無法啟用「滑鼠指標」功能，若在「滑鼠指標」模式下啟動「白板模式」，則自動切換回預設畫筆。

## Capabilities

### New Capabilities

- `mouse-pointer-mode`: 實作視窗點擊穿透功能，並處理與白板模式的互斥邏輯。

### Modified Capabilities

<!-- 無既有 Spec -->

## Impact

- 影響檔案：`src/App.vue` (視窗監聽與邏輯控制), `src/component/SnapinToolbar.vue` (UI 觸發), `src/component/DrawingBoard.vue` (游標樣式調整)。
- 依賴：Tauri `setIgnoreCursorEvents` 視窗 API。
