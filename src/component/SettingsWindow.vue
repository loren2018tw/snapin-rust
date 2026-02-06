<template>
  <v-card class="settings-card" flat>
    <v-card-title class="text-h5 pb-4">
      偏好設定
    </v-card-title>

    <v-card-text>
      <v-container>
        <v-row>
          <v-col cols="12">
            <div class="text-subtitle-1 mb-2">畫筆顏色</div>
            <v-color-picker
              v-model="settings.pen1_color"
              hide-inputs
              show-swatches
              width="100%"
            ></v-color-picker>
          </v-col>

          <v-col cols="12">
            <div class="text-subtitle-1 mb-2">軌跡筆顏色</div>
            <v-color-picker
              v-model="settings.trace_color"
              hide-inputs
              show-swatches
              width="100%"
            ></v-color-picker>
          </v-col>

          <v-col cols="12">
            <div class="text-subtitle-1 mb-2">形狀顏色 (矩形/橢圓)</div>
            <v-color-picker
              v-model="settings.rect_color"
              hide-inputs
              show-swatches
              width="100%"
            ></v-color-picker>
          </v-col>

          <v-col cols="12">
            <div class="text-subtitle-1 mb-2">線條寬度: {{ settings.line_width }}</div>
            <v-slider
              v-model="settings.line_width"
              min="1"
              max="20"
              step="1"
              thumb-label
              color="primary"
            ></v-slider>
          </v-col>

          <v-col cols="12">
            <v-text-field
              label="顯示/隱藏快速鍵"
              v-model="settings.hotkey"
              readonly
              hint="點擊並按下按鍵來錄製快速鍵 (例如: F9, Ctrl+F9)"
              persistent-hint
              @keydown.prevent="recordHotkey"
              prepend-inner-icon="mdi-keyboard"
            ></v-text-field>
          </v-col>
        </v-row>
      </v-container>
    </v-card-text>

    <v-divider></v-divider>

    <v-card-actions class="pa-4">
      <v-btn
        variant="outlined"
        color="secondary"
        @click="resetToDefault"
      >
        預設值
      </v-btn>
      <v-spacer></v-spacer>
      <v-btn
        variant="text"
        @click="emit('close')"
      >
        取消
      </v-btn>
      <v-btn
        color="primary"
        variant="elevated"
        @click="saveSettings"
        :loading="saving"
      >
        儲存
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits(['close', 'updated']);

interface Settings {
  pen1_color: string;
  trace_color: string;
  rect_color: string;
  line_width: number;
  hotkey: string;
}

const settings = ref<Settings>({
  pen1_color: '#000000',
  trace_color: '#ff0000',
  rect_color: '#0000ff',
  line_width: 3,
  hotkey: 'F9',
});

const saving = ref(false);

onMounted(async () => {
  try {
    const savedSettings = await invoke<Settings>('get_settings');
    settings.value = savedSettings;
  } catch (err) {
    console.error('Failed to load settings:', err);
  }
});

function recordHotkey(e: KeyboardEvent) {
  // Ignore modifiers by themselves
  if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) return;

  const combo: string[] = [];
  if (e.ctrlKey) combo.push('Ctrl');
  if (e.shiftKey) combo.push('Shift');
  if (e.altKey) combo.push('Alt');
  if (e.metaKey) combo.push('Command');

  let key = e.key;
  if (key === ' ') {
    key = 'Space';
  } else if (key.length === 1) {
    key = key.toUpperCase();
  } else {
    // Handle special keys like F1-F12, etc.
    // They are already in correct format mostly
  }

  combo.push(key);
  settings.value.hotkey = combo.join('+');
}

function resetToDefault() {
  settings.value = {
    pen1_color: '#000000',
    trace_color: '#ff0000',
    rect_color: '#0000ff',
    line_width: 3,
    hotkey: 'F9',
  };
}

async function saveSettings() {
  saving.value = true;
  try {
    await invoke('update_settings', { newSettings: settings.value });
    emit('updated', settings.value);
    emit('close');
  } catch (err) {
    console.error('Failed to save settings:', err);
    alert('儲存失敗: ' + err);
  } finally {
    saving.value = false;
  }
}
</script>

<style scoped>
.settings-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.v-card-text {
  flex-grow: 1;
  overflow-y: auto;
}
</style>
