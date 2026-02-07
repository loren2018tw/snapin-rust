<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, emit as tauriEmit } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import DrawingBoard from "./component/DrawingBoard.vue";
import SnapinToolbar from "./component/SnapinToolbar.vue";
import SettingsDialog from "./component/SettingsDialog.vue";

const isToolbarWindow = ref(window.location.hash === "#toolbar");
const currentWindow = getCurrentWindow();
const drawingBoard = ref<any>(null);

const activeTool = ref("brush1");
const isWhiteboardMode = ref(false);
const settings = ref({
  pen1Color: "#ff0000",
  traceColor: "#ff8800",
  rectColor: "#0000ff",
  lineWidth: 5,
});
const isSettingsDialogVisible = ref(false);

// 定義設定類型介面（與 Rust AppSettings 結構對應）
interface AppSettings {
  pen1_color: string;
  trace_color: string;
  rect_color: string;
  line_width: number;
}

// 加載設定
async function loadSettings() {
  try {
    const savedSettings = await invoke<AppSettings>("load_settings");
    console.log("App: Loaded settings", savedSettings);
    settings.value = {
      pen1Color: savedSettings.pen1_color || "#ff0000",
      traceColor: savedSettings.trace_color || "#ff8800",
      rectColor: savedSettings.rect_color || "#0000ff",
      lineWidth: savedSettings.line_width || 5,
    };
  } catch (error) {
    console.error("App: Failed to load settings", error);
  }
}

// 保存設定
async function saveSettings(newSettings: typeof settings.value) {
  try {
    await invoke("save_settings", {
      settings: {
        pen1_color: newSettings.pen1Color,
        trace_color: newSettings.traceColor,
        rect_color: newSettings.rectColor,
        line_width: newSettings.lineWidth,
      },
    });
    console.log("App: Settings saved successfully");
  } catch (error) {
    console.error("App: Failed to save settings", error);
  }
}

let unlistenTool: (() => void) | null = null;
let unlistenClear: (() => void) | null = null;
let unlistenWhiteboard: (() => void) | null = null;

/**
 * 監聽並強制同步視窗背景
 */
watch(
  isWhiteboardMode,
  async (val) => {
    if (isToolbarWindow.value) return;

    const color = val ? "#ffffff" : "transparent";
    console.log(`Setting background to: ${color}`);

    // 使用 setProperty 強制注入樣式，避免被被組件樣式覆蓋
    document.documentElement.style.setProperty(
      "background-color",
      color,
      "important",
    );
    document.body.style.setProperty("background-color", color, "important");

    // 如果開啟了白板模式，且當前工具是滑鼠指標，則強制關閉穿透
    if (val && activeTool.value === "Mouse Pointer") {
      console.log("App: Whiteboard enabled, disabling click-through");
      await invoke("set_click_through", { ignore: false });
    }
  },
  { immediate: true },
);

const appStyle = computed(() => {
  if (isToolbarWindow.value) {
    return { backgroundColor: "transparent !important" };
  }
  return {
    "background-color": isWhiteboardMode.value
      ? "#ffffff !important"
      : "transparent !important",
    transition: "background-color 0.3s ease",
  };
});

onMounted(async () => {
  const hash = window.location.hash;
  console.log(
    `App mounted. Window Label: ${currentWindow.label}, Hash: ${hash}`,
  );
  isToolbarWindow.value = hash === "#toolbar";

  // 載入設定
  await loadSettings();

  // 初始化視窗樣式
  document.documentElement.style.backgroundColor = "transparent";
  document.documentElement.style.overflow = "hidden";
  document.body.style.backgroundColor = "transparent";
  document.body.style.overflow = "hidden";
  unlistenTool = await listen<string>("tool-changed", async (event) => {
    console.log("App: tool-changed received", event.payload);
    const newTool = event.payload;
    if (activeTool.value !== newTool) {
      activeTool.value = newTool;
    }

    // 處理滑鼠點擊穿透 (只針對 main 視窗)
    if (!isToolbarWindow.value) {
      if (newTool === "Mouse Pointer") {
        console.log("Main: Requesting click-through via Rust");
        await invoke("set_click_through", { ignore: true });
      } else {
        console.log("Main: Disabling click-through via Rust");
        await invoke("set_click_through", { ignore: false });
      }
    }
  });

  unlistenClear = await listen("clear-canvas", () => {
    console.log("App: clear-canvas received");
    drawingBoard.value?.clearCanvas();
  });

  unlistenWhiteboard = await listen<boolean>(
    "whiteboard-mode-changed",
    (event) => {
      console.log("App: whiteboard-mode-changed received", event.payload);
      if (isWhiteboardMode.value !== event.payload) {
        isWhiteboardMode.value = event.payload;
      }
    },
  );

  // 監聽來自 Tauri 後端的設定對話框開啟事件
  await listen("open-settings", () => {
    console.log("App: open-settings received");
    isSettingsDialogVisible.value = true;
  });

  window.addEventListener("keydown", handleKeydown);

  await nextTick();
});

async function handleKeydown(e: KeyboardEvent) {
  const key = e.key.toLowerCase();

  // 如果正在輸入（雖然目前沒輸入框），則不觸發
  if (
    e.target instanceof HTMLInputElement ||
    e.target instanceof HTMLTextAreaElement
  ) {
    return;
  }

  switch (key) {
    case "b":
      updateTool("brush1");
      break;
    case "t":
      updateTool("Trail Pen");
      break;
    case "r":
      updateTool("Rectangle");
      break;
    case "e":
      updateTool("Ellipse");
      break;
    case "c":
      handleClear();
      break;
    case "w":
      toggleWhiteboard();
      break;
  }
}

async function updateTool(tool: string) {
  if (isWhiteboardMode.value && tool === "Mouse Pointer") return;

  activeTool.value = tool;
  await tauriEmit("tool-changed", tool);
}

async function toggleWhiteboard() {
  const newVal = !isWhiteboardMode.value;
  isWhiteboardMode.value = newVal;
  await tauriEmit("whiteboard-mode-changed", newVal);

  if (newVal && activeTool.value === "Mouse Pointer") {
    updateTool("brush1");
  }
}

async function handleClear() {
  drawingBoard.value?.clearCanvas();
  await tauriEmit("clear-canvas");
}

async function handleHide() {
  await invoke("hide_windows");
}

// 處理設定對話框更新設定
async function handleUpdateSettings(newSettings: typeof settings.value) {
  console.log("App: Updating settings", newSettings);
  settings.value = { ...newSettings };
  await saveSettings(newSettings);
}

onUnmounted(() => {
  if (unlistenTool) unlistenTool();
  if (unlistenClear) unlistenClear();
  if (unlistenWhiteboard) unlistenWhiteboard();
  window.removeEventListener("keydown", handleKeydown);
});

function handleClose() {
  handleHide();
}

async function handleStartDrag() {
  await currentWindow.startDragging();
}
</script>

<template>
  <v-app :style="appStyle">
    <v-main>
      <div v-if="!isToolbarWindow" class="container">
        <DrawingBoard
          ref="drawingBoard"
          :active-tool="activeTool"
          :settings="settings"
        />
        <SettingsDialog
          v-model="isSettingsDialogVisible"
          :settings="settings"
          @update:settings="handleUpdateSettings"
        />
      </div>
      <div v-else>
        <SnapinToolbar
          v-model="activeTool"
          v-model:isWhiteboardMode="isWhiteboardMode"
          @clear="handleClear"
          @close="handleClose"
          @start-drag="handleStartDrag"
        />
      </div>
    </v-main>
  </v-app>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: transparent !important; /* 預設全透明 */
  overflow: hidden !important;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

/* 強制讓 Vuetify 的基礎容器透明 */
.v-application,
.v-application__wrap,
.v-main {
  background: transparent !important;
}

/* 除非在白板模式下，否則這些屬性會被 v-app 的 :style 覆蓋 */

/* 強制隱藏所有捲軸 */
::-webkit-scrollbar {
  display: none;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: transparent; /* 預設透明 */
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
