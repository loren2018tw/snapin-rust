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

      <!-- 形狀工具選單 -->
      <v-menu location="left center" offset="10">
        <template v-slot:activator="{ props: menuProps }">
          <v-btn
            v-bind="menuProps"
            :icon="shapeToolIcon"
            variant="text"
            density="comfortable"
            :color="isShapeToolActive ? 'primary' : 'grey'"
            class="tool-btn"
          />
        </template>
        <v-list density="compact" bg-color="white" elevation="4">
          <v-list-item
            @click="updateTool('Rectangle')"
            title="長方形"
          >
            <template v-slot:prepend>
              <v-icon icon="mdi-rectangle-outline" color="primary"></v-icon>
            </template>
          </v-list-item>
          <v-list-item
            @click="updateTool('Ellipse')"
            title="橢圓形"
          >
            <template v-slot:prepend>
              <v-icon icon="mdi-circle-outline" color="primary"></v-icon>
            </template>
          </v-list-item>
        </v-list>
      </v-menu>

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
  width: 32px;
  height: 32px;
  -webkit-app-region: no-drag;
}

.close-btn {
  margin-top: auto;
  margin-bottom: 20px;
  width: 32px;
  height: 32px;
  -webkit-app-region: no-drag;
}
</style>
