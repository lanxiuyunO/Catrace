<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick, watch } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

interface LogEntry {
  ts: string
  tag: string
  level: string
  message: string
}

const MAX_LOGS = 500

// ——— 持久化状态 ———
const STORE_KEY = 'catrace-logviewer'

function loadState() {
  try {
    const raw = localStorage.getItem(STORE_KEY)
    if (raw) return JSON.parse(raw)
  } catch {}
  return {}
}

function saveState(s: Record<string, any>) {
  try {
    localStorage.setItem(STORE_KEY, JSON.stringify(s))
  } catch {}
}

const saved = loadState()

const open = ref(saved.open ?? false)
const posX = ref(saved.posX ?? -1)
const posY = ref(saved.posY ?? -1)
const panelW = ref(saved.panelW ?? 420)
const panelH = ref(saved.panelH ?? 280)
const showLevels = ref<Record<string, boolean>>(
  saved.showLevels ?? { error: true, warn: true, info: true },
)

// 初始化位置：右下角
if (posX.value < 0 || posY.value < 0) {
  posX.value = window.innerWidth - 52
  posY.value = window.innerHeight - 52
}

function persist() {
  saveState({
    open: open.value,
    posX: posX.value,
    posY: posY.value,
    panelW: panelW.value,
    panelH: panelH.value,
    showLevels: showLevels.value,
  })
}

// ——— 日志 ———
const logs = ref<LogEntry[]>([])
let logListener: UnlistenFn | null = null

const filteredLogs = computed(() => {
  return logs.value.filter((l) => showLevels.value[l.level])
})

function clearLogs() {
  logs.value = []
}

onMounted(async () => {
  logListener = await listen<{ timestamp: string; tag: string; level: string; message: string }>(
    'catrace-log',
    (event) => {
      const { timestamp, tag, level, message } = event.payload
      const time = timestamp.length >= 19 ? timestamp.slice(11, 19) : timestamp
      const entry: LogEntry = { ts: time, tag, level, message }
      logs.value = [...logs.value.slice(-(MAX_LOGS - 1)), entry]
      if (autoScroll.value) {
        nextTick(() => {
          if (listEl.value) {
            listEl.value.scrollTop = listEl.value.scrollHeight
          }
        })
      }
    },
  )
})

onBeforeUnmount(() => {
  if (logListener) logListener()
})

// ——— 自动滚动 ———
const listEl = ref<HTMLElement | null>(null)
const autoScroll = ref(true)

function onListScroll() {
  if (!listEl.value) return
  const el = listEl.value
  autoScroll.value = el.scrollTop + el.clientHeight >= el.scrollHeight - 6
}

// ——— 拖拽按钮 ———
interface Pt { x: number; y: number }
let btnDrag = false
let btnDragStart: Pt = { x: 0, y: 0 }

function onBtnPointerDown(e: PointerEvent) {
  if ((e.target as HTMLElement).closest('.toggle-btn')) return // 展开/折叠不拖
  e.preventDefault()
  btnDrag = true
  btnDragStart = { x: e.clientX - posX.value, y: e.clientY - posY.value }
  ;(e.target as HTMLElement).setPointerCapture(e.pointerId)
}

function onBtnPointerMove(e: PointerEvent) {
  if (!btnDrag) return
  posX.value = e.clientX - btnDragStart.x
  posY.value = e.clientY - btnDragStart.y
  clampBtnPos()
}

function onBtnPointerUp() {
  if (btnDrag) {
    btnDrag = false
    persist()
  }
}

function clampBtnPos() {
  posX.value = Math.max(0, Math.min(window.innerWidth - 40, posX.value))
  posY.value = Math.max(0, Math.min(window.innerHeight - 40, posY.value))
}

// ——— 拖拽面板 ———
let panelDrag = false
let panelDragStart: Pt = { x: 0, y: 0 }

function onPanelDragStart(e: PointerEvent) {
  panelDrag = true
  panelDragStart = { x: e.clientX - posX.value, y: e.clientY - posY.value }
  ;(e.target as HTMLElement).setPointerCapture(e.pointerId)
}

function onPanelDragMove(e: PointerEvent) {
  if (!panelDrag) return
  posX.value = e.clientX - panelDragStart.x
  posY.value = e.clientY - panelDragStart.y
  clampPanelPos()
}

function onPanelDragUp() {
  if (panelDrag) {
    panelDrag = false
    persist()
  }
}

function clampPanelPos() {
  posX.value = Math.max(0, Math.min(window.innerWidth - 60, posX.value))
  posY.value = Math.max(0, Math.min(window.innerHeight - 40, posY.value))
}

// ——— resize ———
let resizing = false
let resizeStart: Pt = { x: 0, y: 0 }
let resizeInitW = 0
let resizeInitH = 0

function onResizeStart(e: PointerEvent) {
  e.preventDefault()
  e.stopPropagation()
  resizing = true
  resizeStart = { x: e.clientX, y: e.clientY }
  resizeInitW = panelW.value
  resizeInitH = panelH.value
  ;(e.target as HTMLElement).setPointerCapture(e.pointerId)
}

function onResizeMove(e: PointerEvent) {
  if (!resizing) return
  panelW.value = Math.max(280, resizeInitW + (e.clientX - resizeStart.x))
  panelH.value = Math.max(160, resizeInitH + (e.clientY - resizeStart.y))
}

function onResizeUp() {
  if (resizing) {
    resizing = false
    persist()
  }
}

// ——— 限制按钮不跑出屏幕 ———
window.addEventListener('resize', () => {
  clampBtnPos()
  clampPanelPos()
})

// 打开面板时的位置调整
watch(open, (val) => {
  if (val) {
    // 面板打开时按钮在面板左上角外
    clampBtnPos()
    clampPanelPos()
  }
  persist()
})
</script>

<template>
  <!-- 浮动按钮 -->
  <div
    v-show="!open"
    class="lv-btn"
    :style="{ left: posX + 'px', top: posY + 'px' }"
    @pointerdown="onBtnPointerDown"
    @pointermove="onBtnPointerMove"
    @pointerup="onBtnPointerUp"
    @pointercancel="onBtnPointerUp"
    title="Log Viewer"
  >
    <span class="lv-btn-icon">LOG</span>
    <button
      class="toggle-btn"
      @pointerdown.stop
      @click.stop="open = !open"
      title="Show logs"
    >
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 15l-6-6-6 6"/></svg>
    </button>
  </div>

  <!-- 日志面板 -->
  <Teleport to="body">
    <div
      v-if="open"
      class="lv-panel"
      :style="{ left: posX + 'px', top: posY + 'px', width: panelW + 'px', height: panelH + 'px' }"
    >
      <!-- header -->
      <div
        class="lv-header"
        @pointerdown="onPanelDragStart"
        @pointermove="onPanelDragMove"
        @pointerup="onPanelDragUp"
        @pointercancel="onPanelDragUp"
      >
        <span class="lv-title">Logs</span>
        <div class="lv-header-actions">
          <span class="lv-count">{{ filteredLogs.length }} / {{ logs.length }}</span>
          <button class="lv-hdr-btn" @click="open = false" title="Close">✕</button>
        </div>
      </div>

      <!-- toolbar -->
      <div class="lv-toolbar">
        <label class="lv-filter" :class="{ active: showLevels.error }">
          <input type="checkbox" v-model="showLevels.error" @change="persist()" />
          <span class="lv-f-error">E</span>
        </label>
        <label class="lv-filter" :class="{ active: showLevels.warn }">
          <input type="checkbox" v-model="showLevels.warn" @change="persist()" />
          <span class="lv-f-warn">W</span>
        </label>
        <label class="lv-filter" :class="{ active: showLevels.info }">
          <input type="checkbox" v-model="showLevels.info" @change="persist()" />
          <span class="lv-f-info">I</span>
        </label>
        <div class="lv-toolbar-spacer" />
        <button class="lv-hdr-btn" @click="clearLogs" title="Clear">Clear</button>
      </div>

      <!-- log list -->
      <div
        ref="listEl"
        class="lv-list"
        @scroll="onListScroll"
      >
        <div
          v-for="(entry, i) in filteredLogs"
          :key="i"
          class="lv-line"
          :class="`lv-${entry.level}`"
        >
          <span class="lv-ts">{{ entry.ts }}</span>
          <span class="lv-tag">{{ entry.tag }}</span>
          <span class="lv-msg">{{ entry.message }}</span>
        </div>
        <div v-if="filteredLogs.length === 0" class="lv-empty">
          waiting for logs…
        </div>
      </div>

      <!-- resize handle -->
      <div
        class="lv-resize"
        @pointerdown="onResizeStart"
        @pointermove="onResizeMove"
        @pointerup="onResizeUp"
        @pointercancel="onResizeUp"
      />
    </div>
  </Teleport>
</template>

<style scoped>
/* ——— button ——— */
.lv-btn {
  position: fixed;
  z-index: 9999;
  width: 1.75rem;
  height: 1.75rem;
  border-radius: 50%;
  background: rgba(30, 10, 60, 0.75);
  backdrop-filter: blur(6px);
  cursor: grab;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  box-shadow: 0 0.125rem 0.5rem rgba(0,0,0,0.25);
  transition: box-shadow 0.15s;
}

.lv-btn:active {
  cursor: grabbing;
  box-shadow: 0 0.125rem 0.75rem rgba(124, 58, 237, 0.4);
}

.lv-btn-icon {
  font-size: 0.4375rem;
  font-weight: 800;
  color: #c4b5fd;
  letter-spacing: 0.05em;
  font-family: monospace;
}

.toggle-btn {
  position: absolute;
  top: -0.25rem;
  right: -0.25rem;
  width: 0.875rem;
  height: 0.875rem;
  border-radius: 50%;
  border: none;
  background: rgba(124, 58, 237, 0.85);
  color: #fff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  line-height: 0;
}

/* ——— panel ——— */
.lv-panel {
  position: fixed;
  z-index: 9998;
  background: rgba(22, 18, 32, 0.94);
  backdrop-filter: blur(12px);
  border-radius: 0.5rem;
  border: 0.0625rem solid rgba(124, 58, 237, 0.25);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 0.5rem 1.5rem rgba(0,0,0,0.4);
  user-select: none;
}

.lv-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 1.625rem;
  padding: 0 0.5rem;
  cursor: move;
  flex-shrink: 0;
  border-bottom: 0.0625rem solid rgba(255,255,255,0.06);
}

.lv-title {
  font-size: 0.6875rem;
  font-weight: 600;
  color: #a78bfa;
  letter-spacing: 0.04em;
}

.lv-header-actions {
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.lv-count {
  font-size: 0.5625rem;
  color: rgba(255,255,255,0.3);
  font-variant-numeric: tabular-nums;
  font-family: monospace;
}

.lv-hdr-btn {
  width: 1rem;
  height: 1rem;
  border: none;
  background: none;
  color: rgba(255,255,255,0.35);
  cursor: pointer;
  font-size: 0.625rem;
  padding: 0;
  line-height: 1;
  border-radius: 0.1875rem;
}

.lv-hdr-btn:hover {
  background: rgba(255,255,255,0.08);
  color: rgba(255,255,255,0.7);
}

/* ——— toolbar ——— */
.lv-toolbar {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.1875rem 0.5rem;
  flex-shrink: 0;
  border-bottom: 0.0625rem solid rgba(255,255,255,0.05);
}

.lv-toolbar-spacer {
  flex: 1;
}

.lv-filter {
  display: flex;
  align-items: center;
  gap: 0;
  cursor: pointer;
  opacity: 0.35;
  transition: opacity 0.12s;
}

.lv-filter.active {
  opacity: 1;
}

.lv-filter input {
  display: none;
}

.lv-filter span {
  font-size: 0.5rem;
  font-weight: 800;
  padding: 0.0625rem 0.25rem;
  border-radius: 0.1875rem;
  font-family: monospace;
  letter-spacing: 0.05em;
}

.lv-f-error { color: #f87171; }
.lv-f-warn { color: #fbbf24; }
.lv-f-info { color: #9ca3af; }

.lv-filter.active .lv-f-error { background: rgba(248,113,113,0.18); }
.lv-filter.active .lv-f-warn { background: rgba(251,191,36,0.18); }
.lv-filter.active .lv-f-info { background: rgba(156,163,175,0.12); }

/* ——— list ——— */
.lv-list {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', monospace;
  font-size: 0.625rem;
  line-height: 1.55;
  padding: 0.125rem 0;
}

.lv-list::-webkit-scrollbar {
  width: 0.25rem;
}

.lv-list::-webkit-scrollbar-thumb {
  background: rgba(255,255,255,0.1);
  border-radius: 0.125rem;
}

.lv-line {
  display: flex;
  gap: 0.375rem;
  padding: 0 0.5rem;
  white-space: nowrap;
}

.lv-line:hover {
  background: rgba(255,255,255,0.03);
}

.lv-ts {
  color: rgba(255,255,255,0.25);
  flex-shrink: 0;
  width: 4.5rem;
}

.lv-tag {
  flex-shrink: 0;
  width: 5rem;
  overflow: hidden;
  text-overflow: ellipsis;
  color: rgba(255,255,255,0.35);
}

.lv-msg {
  color: rgba(255,255,255,0.6);
  overflow: hidden;
  text-overflow: ellipsis;
}

/* level colors */
.lv-line.lv-error .lv-tag,
.lv-line.lv-error .lv-msg {
  color: #f87171;
}

.lv-line.lv-warn .lv-tag,
.lv-line.lv-warn .lv-msg {
  color: #fbbf24;
}

.lv-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: rgba(255,255,255,0.15);
  font-size: 0.75rem;
  font-family: inherit;
}

/* ——— resize ——— */
.lv-resize {
  position: absolute;
  right: 0;
  bottom: 0;
  width: 0.75rem;
  height: 0.75rem;
  cursor: nwse-resize;
}
</style>
