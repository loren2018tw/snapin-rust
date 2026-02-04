## 1. 前端邏輯實作 (App.vue)

- [x] 1.1 在 `App.vue` 中擴充 `tool-changed` 監聽器，當 `payload` 為 `Mouse Pointer` 時呼叫 `currentWindow.setIgnoreCursorEvents(true)`
- [x] 1.2 在 `App.vue` 中處理切換回其他工具時呼叫 `currentWindow.setIgnoreCursorEvents(false)`
- [x] 1.3 在 `App.vue` 中確保切換白板模式時，若當前為 `Mouse Pointer` 則恢復鼠標互動性

## 2. 工具列 UI 完善 (SnapinToolbar.vue)

- [x] 2.1 檢查並確保 `SnapinToolbar.vue` 內的 `updateTool` 邏輯正確處理 `Mouse Pointer`
- [x] 2.2 驗證 `toggleWhiteboard` 函數在切換至白板模式時，正確將工具重設為 `brush1`

## 3. 游標視覺回饋 (DrawingBoard.vue)

- [x] 3.1 在 `DrawingBoard.vue` 中新增動態樣式，當 `activeTool` 為 `Mouse Pointer` 時將 `cursor` 設為 `default`

## 4. 驗證與測試

- [x] 4.1 手動測試：驗證「滑鼠指標」模式下可點擊後方視窗
- [x] 4.2 手動測試：驗證切換回「畫筆」後可恢復正常繪圖
- [x] 4.3 手動測試：驗證「白板模式」開啟後，「滑鼠指標」按鈕為停用狀態，且模式會自動重設
