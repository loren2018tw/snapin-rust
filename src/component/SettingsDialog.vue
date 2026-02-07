<template>
  <v-dialog v-model="isVisible" max-width="500" scrollable persistent>
    <v-card>
      <v-card-title class="text-h6"> 繪圖工具設定 </v-card-title>

      <v-card-text>
        <v-form>
          <!-- 畫筆1顏色 -->
          <v-row align="center">
            <v-col cols="12" md="6">
              <label class="font-weight-bold">畫筆1顏色</label>
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="localSettings.pen1Color"
                type="color"
                hide-details
              />
            </v-col>
          </v-row>

          <!-- 追蹤筆顏色 -->
          <v-row align="center">
            <v-col cols="12" md="6">
              <label class="font-weight-bold">追蹤筆顏色</label>
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="localSettings.traceColor"
                type="color"
                hide-details
              />
            </v-col>
          </v-row>

          <!-- 矩形/橢圓顏色 -->
          <v-row align="center">
            <v-col cols="12" md="6">
              <label class="font-weight-bold">矩形/橢圓顏色</label>
            </v-col>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="localSettings.rectColor"
                type="color"
                hide-details
              />
            </v-col>
          </v-row>

          <!-- 線條寬度 -->
          <v-row align="center">
            <v-col cols="12" md="6">
              <label class="font-weight-bold">線條寬度</label>
            </v-col>
            <v-col cols="12" md="6">
              <v-slider
                v-model="localSettings.lineWidth"
                :min="1"
                :max="20"
                :step="1"
                hide-details
              />
              <div class="text-right">{{ localSettings.lineWidth }}px</div>
            </v-col>
          </v-row>
        </v-form>
      </v-card-text>

      <v-card-actions>
        <v-spacer />
        <v-btn text @click="handleCancel"> 取消 </v-btn>
        <v-btn color="primary" @click="handleSave"> 保存 </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

interface Settings {
  pen1Color: string;
  traceColor: string;
  rectColor: string;
  lineWidth: number;
}

interface Props {
  modelValue: boolean;
  settings: Settings;
}

const props = defineProps<Props>();

interface Emits {
  (e: "update:modelValue", value: boolean): void;
  (e: "update:settings", value: Settings): void;
}

const emit = defineEmits<Emits>();

const isVisible = ref(props.modelValue);
const localSettings = ref<Settings>({ ...props.settings });

watch(
  () => props.modelValue,
  (newVal) => {
    isVisible.value = newVal;
    if (newVal) {
      localSettings.value = { ...props.settings };
    }
  },
);

watch(
  () => isVisible.value,
  (newVal) => {
    emit("update:modelValue", newVal);
  },
);

const handleCancel = () => {
  isVisible.value = false;
};

const handleSave = () => {
  emit("update:settings", { ...localSettings.value });
  isVisible.value = false;
};
</script>

<style scoped></style>
