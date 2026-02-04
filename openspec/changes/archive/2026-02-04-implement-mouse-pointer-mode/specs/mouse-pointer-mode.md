## ADDED Requirements

### Requirement: 切換點擊穿透模式 (Toggle Click-Through)

系統應允許使用者將繪圖視窗（main 視窗）切換為點擊穿透模式。在該模式下，所有發生在繪圖視窗區域的滑鼠事件應直接傳遞給下方的視窗。

#### Scenario: 啟動滑鼠指標功能

- **WHEN** 使用者在工具列選擇「滑鼠指標」工具
- **THEN** 繪圖視窗應呼叫 `setIgnoreCursorEvents(true)`
- **AND** 使用者點擊繪圖區域時，應作用在下方的應用程式

#### Scenario: 關閉滑鼠指標功能

- **WHEN** 使用者切換至其他繪圖工具（如畫筆、矩形）
- **THEN** 繪圖視窗應呼叫 `setIgnoreCursorEvents(false)`
- **AND** 使用者可以再次在畫布上進行繪圖

### Requirement: 與白板模式互斥 (Whiteboard Mutex)

「滑鼠指標」功能與「白板模式」不得同時開啟。

#### Scenario: 白板模式下嘗試切換滑鼠指標

- **WHEN** 白板模式已開啟
- **THEN** 工具列中的「滑鼠指標」按鈕應處於停用（disabled）狀態

#### Scenario: 滑鼠指標模式下開啟白板模式

- **WHEN** 當前為「滑鼠指標」模式且使用者點擊「白板模式」切換
- **THEN** 系統應關閉「滑鼠指標」模式並切換回預設畫筆工具
- **AND** 繪圖視窗應恢復鼠標互動性（`setIgnoreCursorEvents(false)`）

### Requirement: 游標樣式變更 (Cursor Style)

在「滑鼠指標」模式下，雖然視窗已穿透，但為了提供視覺回饋，原有的繪圖游標（如 crosshair）應變更為系統預設游標。

#### Scenario: 切換至滑鼠指標模式後的游標外觀

- **WHEN** 進入「滑鼠指標」模式
- **THEN** 繪圖區域的 CSS `cursor` 屬性應設為 `default` 或 `auto`
