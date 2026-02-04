<template>
  <div class="toolbar">
    <div class="tool-wrapper">
      <!-- 拖動區域 (使用一個專門的 div 包裹圖標) -->
      <div 
        class="drag-handle" 
        data-tauri-drag-region
        @mousedown="emit('start-drag', $event)"
        title="按住拖動"
      >
        <v-icon icon="mdi-cursor-move" color="grey-darken-1" size="24" />
      </div>

      <!-- 畫筆工具 -->
      <v-btn
        icon="mdi-brush"
        variant="text"
        density="comfortable"
        :color="modelValue === 'brush1' ? 'primary' : 'grey'"
        @click="updateTool('brush1')"
        class="tool-btn"
        title="畫筆"
      />

      <!-- 軌跡筆工具 -->
      <v-btn
        icon="mdi-auto-fix"
        variant="text"
        density="comfortable"
        :color="modelValue === 'Trail Pen' ? 'primary' : 'grey'"
        @click="updateTool('Trail Pen')"
        class="tool-btn"
        title="軌跡筆"
      />

      <!-- 形狀工具選單 (Speed Dial) -->
      <v-speed-dial location="left center" transition="scale-transition">
        <template v-slot:activator="{ props: activatorProps }">
          <v-fab
            v-bind="activatorProps"
            :icon="shapeToolIcon"
            variant="text"
            density="comfortable"
            :color="isShapeToolActive ? 'primary' : 'grey-darken-1'"
            class="tool-btn"
            title="形狀工具"
          />
        </template>

        <v-btn
          key="Rectangle"
          variant="flat"
          density="comfortable"
          elevation="1"
          @click="updateTool('Rectangle')"
          class="tool-btn"
          title="長方形"
        >
          <v-icon icon="mdi-rectangle-outline" color="primary" />
        </v-btn>
        <v-btn
          key="Ellipse"
          variant="flat"
          density="comfortable"
          elevation="1"
          @click="updateTool('Ellipse')"
          class="tool-btn"
          title="橢圓形"
        >
          <v-icon icon="mdi-circle-outline" color="primary" />
        </v-btn>
      </v-speed-dial>

      <!-- 滑鼠指標工具 -->
      <v-btn
        icon="mdi-mouse"
        variant="text"
        density="comfortable"
        :color="modelValue === 'Mouse Pointer' ? 'primary' : 'grey'"
        @click="updateTool('Mouse Pointer')"
        class="tool-btn"
        :disabled="isWhiteboardMode"
        title="滑鼠指標"
      />

      <!-- 清除畫布 -->
      <v-btn
        icon="mdi-broom"
        variant="text"
        density="comfortable"
        color="orange"
        @click="handleClear"
        class="tool-btn"
        title="清除畫布"
      />

      <!-- 白板模式切換 -->
      <v-btn
        icon="mdi-layers"
        variant="text"
        density="comfortable"
        :color="isWhiteboardMode ? 'primary' : 'grey'"
        @click="toggleWhiteboard"
        class="tool-btn"
        title="白板模式"
      />

      <!-- 關閉應用 -->
      <v-btn
        icon="mdi-close"
        variant="text"
        density="comfortable"
        color="error"
        @click="emit('close')"
        class="close-btn"
        title="關閉程式"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { emit as tauriEmit } from '@tauri-apps/api/event';

/**
 * 元件屬性定義
 */
interface Props {
  /** 當前所選工具 (v-model) */
  modelValue: string;
  /** 是否開啟白板模式 */
  isWhiteboardMode?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  isWhiteboardMode: false,
});

/**
 * 元件事件定義
 */
const emit = defineEmits<{
  /** 更新工具 */
  (e: 'update:modelValue', tool: string): void;
  /** 更新白板模式 */
  (e: 'update:isWhiteboardMode', val: boolean): void;
  /** 清除畫布 */
  (e: 'clear'): void;
  /** 關閉應用 */
  (e: 'close'): void;
  /** 開始拖拽 */
  (e: 'start-drag', event: MouseEvent): void;
}>();

/**
 * 根據當前工具返回對應的形狀圖標
 */
const shapeToolIcon = computed(() => {
  if (props.modelValue === 'Rectangle') return 'mdi-rectangle-outline';
  if (props.modelValue === 'Ellipse') return 'mdi-circle-outline';
  return 'mdi-shape';
});

/**
 * 檢查當前是否選中了形狀工具（矩形或橢圓）
 */
const isShapeToolActive = computed(() => {
  return ['Rectangle', 'Ellipse'].includes(props.modelValue);
});

/**
 * 更新所選工具
 * @param tool 工具名稱
 */
async function updateTool(tool: string) {
  if (props.isWhiteboardMode && tool === 'Mouse Pointer') {
    return;
  }
  emit('update:modelValue', tool);
  
  // 透過 Tauri 事件通知其他視窗
  await tauriEmit('tool-changed', tool);
}

/**
 * 清除畫布
 */
async function handleClear() {
  emit('clear');
  // 透過 Tauri 事件通知其他視窗
  await tauriEmit('clear-canvas');
}

/**
 * 切換白板模式
 */
async function toggleWhiteboard() {
  const newVal = !props.isWhiteboardMode;
  emit('update:isWhiteboardMode', newVal);
  
  // 透過 Tauri 事件通知其他視窗
  await tauriEmit('whiteboard-mode-changed', newVal);

  // 如果切換到白板模式且當前是滑鼠指標，則切換回預設畫筆
  if (newVal && props.modelValue === 'Mouse Pointer') {
    const defaultTool = 'brush1';
    emit('update:modelValue', defaultTool);
    await tauriEmit('tool-changed', defaultTool);
  }
}
</script>

<style scoped>
.toolbar {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  height: 100vh;
  background-color: transparent; /* 整體視窗背景透明 */
  padding: 0;
  gap: 5px;
  position: relative;
  overflow: hidden;
}



/* 僅在按鈕區域顯示背景 */
.tool-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 50px; /* 縮窄背景區域 */
  padding: 10px 0;
  gap: 8px;
  background-color: rgba(255, 255, 255, 0.9); /* 半透明白色背景 */
  border-radius: 25px; /* 膠囊狀 */
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  margin-top: 20px;
}

.drag-handle {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: move;
  background-color: rgba(0, 0, 0, 0.05);
  border-radius: 50%;
  margin-bottom: 5px;
  -webkit-app-region: drag; /* 輔助 Linux 識別 */
}

.tool-btn {
  width: 36px !important;
  height: 36px !important;
  min-width: 36px !important; /* 防止被擠壓成橢圓 */
  padding: 0 !important;
  background-color: #ffffff !important; 
  border-radius: 50% !important;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0; /* 防止在 flex 佈局中縮小 */
  -webkit-app-region: no-drag;
}

/* Speed Dial 彈出內容的樣式 */
:deep(.v-speed-dial__content) {
  background-color: rgba(255, 255, 255, 0.9); 
  padding: 8px 12px;
  border-radius: 30px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: row; 
  gap: 10px;
  margin-right: 10px; 
}

.close-btn {
  margin-top: auto;
  margin-bottom: 20px;
  width: 36px !important;
  height: 36px !important;
  min-width: 36px !important;
  padding: 0 !important;
  background-color: #f0efe3 !important; 
  border-radius: 50% !important;
  flex-shrink: 0;
  -webkit-app-region: no-drag;
}
</style>
