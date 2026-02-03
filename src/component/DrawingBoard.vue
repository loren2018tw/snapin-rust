<template>
  <canvas ref="canvasEl" class="drawing-board"></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

/**
 * 繪圖板組件的屬性
 */
const props = defineProps<{
  /** 當前選擇的工具類型 */
  activeTool: string;
  /** 繪圖相關設定 */
  settings: {
    /** 畫筆 1 的顏色 */
    pen1Color: string;
    /** 追蹤筆的顏色 */
    traceColor: string;
    /** 矩形和橢圓的顏色 */
    rectColor: string;
    /** 線條寬度 */
    lineWidth: number;
  };
}>();

/**
 * 繪圖板組件觸發的事件
 */
const emit = defineEmits<{
  /** 更新當前工具 */
  (e: 'update:activeTool', tool: string): void;
  /** 繪圖區域滑鼠按下事件 */
  (e: 'drawing-mousedown'): void;
}>();

/** 基礎形狀介面 */
interface BaseShape {
  /** 形狀類型 */
  type: string;
  /** 顏色 */
  color: string;
  /** 線條寬度 */
  width: number;
}

/** 畫筆形狀介面 */
interface BrushShape extends BaseShape {
  type: 'brush1';
  /** 起始 X 座標 */
  x1: number;
  /** 起始 Y 座標 */
  y1: number;
  /** 結束 X 座標 */
  x2: number;
  /** 結束 Y 座標 */
  y2: number;
}

/** 追蹤筆形狀介面 */
interface TrailShape extends BaseShape {
  type: 'Trail Pen';
  /** 起始 X 座標 */
  x1: number;
  /** 起始 Y 座標 */
  y1: number;
  /** 結束 X 座標 */
  x2: number;
  /** 結束 Y 座標 */
  y2: number;
  /** 時間戳 */
  timestamp: number;
}

/** 矩形形狀介面 */
interface RectShape extends BaseShape {
  type: 'Rectangle';
  /** 起始 X 座標 */
  x1: number;
  /** 起始 Y 座標 */
  y1: number;
  /** 結束 X 座標 */
  x2: number;
  /** 結束 Y 座標 */
  y2: number;
}

/** 橢圓形狀介面 */
interface EllipseShape extends BaseShape {
  type: 'Ellipse';
  /** 中心 X 座標 */
  x: number;
  /** 中心 Y 座標 */
  y: number;
  /** X 軸半徑 */
  radiusX: number;
  /** Y 軸半徑 */
  radiusY: number;
}

type Shape = BrushShape | TrailShape | RectShape | EllipseShape;

const canvasEl = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;
let isDrawing = false;
let lastX = 0;
let lastY = 0;
const shapes: Shape[] = [];
let tempRectangleShape: RectShape | null = null;
let tempEllipseShape: EllipseShape | null = null;
let startX = 0;
let startY = 0;
let animationFrameId: number;

// Helper functions (ported from drawing.html)
/**
 * 繪製矩形
 * @param context Canvas 繪圖環境
 * @param x1 起始 X 座標
 * @param y1 起始 Y 座標
 * @param x2 結束 X 座標
 * @param y2 結束 Y 座標
 * @param color 顏色
 * @param width 線條寬度
 */
function drawRectangle(
  context: CanvasRenderingContext2D,
  x1: number,
  y1: number,
  x2: number,
  y2: number,
  color: string,
  width: number,
) {
  context.strokeStyle = color;
  context.lineWidth = width;
  context.beginPath();
  context.rect(x1, y1, x2 - x1, y2 - y1);
  context.stroke();
}

/**
 * 繪製橢圓
 * @param context Canvas 繪圖環境
 * @param x 中心 X 座標
 * @param y 中心 Y 座標
 * @param radiusX X 軸半徑
 * @param radiusY Y 軸半徑
 * @param color 顏色
 * @param width 線條寬度
 */
function drawEllipse(
  context: CanvasRenderingContext2D,
  x: number,
  y: number,
  radiusX: number,
  radiusY: number,
  color: string,
  width: number,
) {
  context.strokeStyle = color;
  context.lineWidth = width;
  context.beginPath();
  context.ellipse(x, y, radiusX, radiusY, 0, 0, 2 * Math.PI);
  context.stroke();
}

/**
 * 處理滑鼠移動時的繪圖邏輯
 * @param e 滑鼠事件
 */
function draw(e: MouseEvent) {
  if (!isDrawing || !ctx || !canvasEl.value) return;

  if (props.activeTool === 'brush1') {
    ctx.strokeStyle = props.settings.pen1Color;
    ctx.lineWidth = props.settings.lineWidth;
    ctx.beginPath();
    ctx.moveTo(lastX, lastY);
    ctx.lineTo(e.clientX, e.clientY);
    ctx.stroke();
    shapes.push({
      type: 'brush1',
      x1: lastX,
      y1: lastY,
      x2: e.clientX,
      y2: e.clientY,
      color: props.settings.pen1Color,
      width: props.settings.lineWidth,
    });
  } else if (props.activeTool === 'Trail Pen') {
    ctx.strokeStyle = props.settings.traceColor;
    ctx.lineWidth = props.settings.lineWidth;
    ctx.beginPath();
    ctx.moveTo(lastX, lastY);
    ctx.lineTo(e.clientX, e.clientY);
    ctx.stroke();
    shapes.push({
      type: 'Trail Pen',
      x1: lastX,
      y1: lastY,
      x2: e.clientX,
      y2: e.clientY,
      timestamp: Date.now(),
      color: props.settings.traceColor,
      width: props.settings.lineWidth,
    });
  } else if (props.activeTool === 'Rectangle') {
    redrawCanvas();
    if (props.settings) {
      tempRectangleShape = {
        type: 'Rectangle',
        x1: startX,
        y1: startY,
        x2: e.clientX,
        y2: e.clientY,
        color: props.settings.rectColor,
        width: props.settings.lineWidth,
      };
      drawRectangle(
        ctx,
        tempRectangleShape.x1,
        tempRectangleShape.y1,
        tempRectangleShape.x2,
        tempRectangleShape.y2,
        tempRectangleShape.color,
        tempRectangleShape.width,
      );
    }
  } else if (props.activeTool === 'Ellipse') {
    redrawCanvas();
    if (props.settings) {
      const radiusX = Math.abs(e.clientX - startX) / 2;
      const radiusY = Math.abs(e.clientY - startY) / 2;
      const centerX = startX + (e.clientX >= startX ? radiusX : -radiusX);
      const centerY = startY + (e.clientY >= startY ? radiusY : -radiusY);

      tempEllipseShape = {
        type: 'Ellipse',
        x: centerX,
        y: centerY,
        radiusX: radiusX,
        radiusY: radiusY,
        color: props.settings.rectColor,
        width: props.settings.lineWidth,
      };
      drawEllipse(
        ctx,
        tempEllipseShape.x,
        tempEllipseShape.y,
        tempEllipseShape.radiusX,
        tempEllipseShape.radiusY,
        tempEllipseShape.color,
        tempEllipseShape.width,
      );
    }
  }

  lastX = e.clientX;
  lastY = e.clientY;
}

/**
 * 重新繪製整個畫布
 */
function redrawCanvas() {
  if (!ctx || !canvasEl.value) return;
  ctx.clearRect(0, 0, canvasEl.value.width, canvasEl.value.height);

  for (const shape of shapes) {
    if (shape.type === 'brush1') {
      ctx.strokeStyle = shape.color;
      ctx.lineWidth = shape.width;
      ctx.beginPath();
      ctx.moveTo(shape.x1, shape.y1);
      ctx.lineTo(shape.x2, shape.y2);
      ctx.stroke();
    } else if (shape.type === 'Trail Pen') {
      const trail = shape;
      const age = Date.now() - trail.timestamp;
      const opacity = 1 - age / 5000;
      if (opacity > 0) {
        let r = 0,
          g = 0,
          b = 0;
        if (shape.color && shape.color.startsWith('#')) {
          r = parseInt(shape.color.slice(1, 3), 16);
          g = parseInt(shape.color.slice(3, 5), 16);
          b = parseInt(shape.color.slice(5, 7), 16);
        } else {
          // Fallback if not hex or undefined (simplified)
        }
        ctx.strokeStyle = `rgba(${r}, ${g}, ${b}, ${opacity})`;
        ctx.lineWidth = shape.width;
        ctx.beginPath();
        ctx.moveTo(trail.x1, trail.y1);
        ctx.lineTo(trail.x2, trail.y2);
        ctx.stroke();
      }
    } else if (shape.type === 'Rectangle') {
      drawRectangle(ctx, shape.x1, shape.y1, shape.x2, shape.y2, shape.color, shape.width);
    } else if (shape.type === 'Ellipse') {
      drawEllipse(ctx, shape.x, shape.y, shape.radiusX, shape.radiusY, shape.color, shape.width);
    }
  }
  // Draw temporary shapes (preview)
  if (tempRectangleShape) {
    drawRectangle(
      ctx,
      tempRectangleShape.x1,
      tempRectangleShape.y1,
      tempRectangleShape.x2,
      tempRectangleShape.y2,
      tempRectangleShape.color,
      tempRectangleShape.width,
    );
  }
  if (tempEllipseShape) {
    drawEllipse(
      ctx,
      tempEllipseShape.x,
      tempEllipseShape.y,
      tempEllipseShape.radiusX,
      tempEllipseShape.radiusY,
      tempEllipseShape.color,
      tempEllipseShape.width,
    );
  }
}

/**
 * 形狀動畫處理（用於追蹤筆的淡出效果）
 */
function animateShapes() {
  redrawCanvas();
  for (let i = 0; i < shapes.length; i++) {
    const shape = shapes[i];
    if (!shape) continue;
    if (shape.type === 'Trail Pen') {
      const age = Date.now() - shape.timestamp;
      if (age / 5000 > 1) {
        shapes.splice(i, 1);
        i--;
      }
    }
  }
  animationFrameId = requestAnimationFrame(animateShapes);
}

/**
 * 處理滑鼠按下事件
 * @param e 滑鼠事件
 */
function handleMousedown(e: MouseEvent) {
  console.log('DrawingBoard: mousedown', e.clientX, e.clientY);
  emit('drawing-mousedown');
  isDrawing = true;
  lastX = e.clientX;
  lastY = e.clientY;
  startX = e.clientX;
  startY = e.clientY;
}

/**
 * 處理滑鼠放開事件
 */
function handleMouseup() {
  if (!isDrawing) return;
  isDrawing = false;

  if (props.activeTool === 'Rectangle' && tempRectangleShape) {
    shapes.push({ ...tempRectangleShape });
    tempRectangleShape = null;
  } else if (props.activeTool === 'Ellipse' && tempEllipseShape) {
    shapes.push({ ...tempEllipseShape });
    tempEllipseShape = null;
  }

  redrawCanvas();
}

/**
 * 處理滑鼠移出畫布事件
 */
function handleMouseout() {
  isDrawing = false;
}

function handleResize() {
  if (canvasEl.value && ctx) {
    canvasEl.value.width = window.innerWidth;
    canvasEl.value.height = window.innerHeight;
    ctx.lineCap = 'round'; // Reset context settings after resize
    redrawCanvas();
  }
}

// Expose clear function
/**
 * 清除畫布上的所有內容
 */
const clearCanvas = () => {
  shapes.length = 0;
  if (!ctx || !canvasEl.value) return;
  ctx.clearRect(0, 0, canvasEl.value.width, canvasEl.value.height);
};

defineExpose({
  clearCanvas,
});

onMounted(() => {
  console.log('DrawingBoard mounted');
  if (canvasEl.value) {
    try {
      ctx = canvasEl.value.getContext('2d');
      if (!ctx) {
        console.error('Failed to get 2d context');
        return;
      }
      handleResize();
      window.addEventListener('resize', handleResize);

      canvasEl.value.addEventListener('mousedown', handleMousedown);
      window.addEventListener('mousemove', draw);
      window.addEventListener('mouseup', handleMouseup);
      canvasEl.value.addEventListener('mouseout', handleMouseout);

      animateShapes();
      console.log('DrawingBoard initialized');
    } catch (err) {
      console.error('Error initializing DrawingBoard:', err);
    }
  }
});

onUnmounted(() => {
  console.log('DrawingBoard unmounting');
  window.removeEventListener('resize', handleResize);
  if (canvasEl.value) {
    canvasEl.value.removeEventListener('mousedown', handleMousedown);
    canvasEl.value.removeEventListener('mouseout', handleMouseout);
  }
  window.removeEventListener('mousemove', draw);
  window.removeEventListener('mouseup', handleMouseup);
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
  }
});
</script>

<style scoped>
.drawing-board {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: block;
  background: transparent;
  cursor: crosshair;
  z-index: 0; /* Behind toolbar */
}
</style>
