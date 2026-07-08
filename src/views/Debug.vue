<script setup lang="ts">
import { ref, onActivated, onDeactivated, onMounted, nextTick, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import {
  NCard,
  NSpace,
  NTag,
  NButton,
  NEmpty,
  NDescriptions,
  NDescriptionsItem,
  NTable,
  NText,
  NAlert,
  NSwitch,
  NInputNumber,
  NTabs,
  NTabPane,
  NCheckbox,
} from 'naive-ui'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import {
  getMediaDebugInfo,
  type MediaDebugInfo,
  getToastDebugMode,
  setToastDebugMode,
  startNotificationTest,
  stopNotificationTest,
} from '../api/tauri'

const { t } = useI18n()

// ——— 媒体调试状态 ———
const data = ref<MediaDebugInfo | null>(null)
const loading = ref(false)
const errorMsg = ref<string | null>(null)
const toastDebugMode = ref(false)
const testRunning = ref(false)
const testInterval = ref(15)
let mounted = true
let timer: ReturnType<typeof setTimeout> | null = null

async function loadToastDebugMode() {
  try {
    toastDebugMode.value = await getToastDebugMode()
  } catch (e: any) {
    console.error(e)
  }
}

async function toggleToastDebugMode(value: boolean) {
  try {
    await setToastDebugMode(value)
    toastDebugMode.value = value
  } catch (e: any) {
    console.error(e)
  }
}

async function startTest() {
  try {
    await startNotificationTest(testInterval.value)
    testRunning.value = true
  } catch (e: any) {
    console.error(e)
  }
}

async function stopTest() {
  try {
    await stopNotificationTest()
    testRunning.value = false
  } catch (e: any) {
    console.error(e)
  }
}

async function refresh(manual = false) {
  if (manual) loading.value = true
  errorMsg.value = null
  try {
    data.value = await getMediaDebugInfo()
  } catch (e: any) {
    errorMsg.value = e?.message || String(e)
    console.error(e)
  } finally {
    if (manual) loading.value = false
  }
}

function startRefreshLoop() {
  refresh(false).finally(() => {
    if (mounted) {
      timer = setTimeout(startRefreshLoop, 2000)
    }
  })
}

// ——— 运行日志状态 ———
interface LogEntry {
  ts: string
  tag: string
  level: string
  message: string
}

const MAX_LOGS = 200
const logs = ref<LogEntry[]>([])
const showLevels = ref<Record<string, boolean>>({ error: true, warn: true, info: true })
let logListener: UnlistenFn | null = null
void logListener

const filteredLogs = computed(() => {
  return logs.value.filter((l) => showLevels.value[l.level])
})

function clearLogs() {
  logs.value = []
}

const logContainer = ref<HTMLElement | null>(null)
let autoScroll = true

function onLogScroll() {
  if (!logContainer.value) return
  const el = logContainer.value
  autoScroll = el.scrollTop + el.clientHeight >= el.scrollHeight - 4
}

onMounted(async () => {
  logListener = await listen<{ timestamp: string; tag: string; level: string; message: string }>(
    'catrace-log',
    (event) => {
      const { timestamp, tag, level, message } = event.payload
      const time = timestamp.length >= 19 ? timestamp.slice(11, 19) : timestamp
      const entry: LogEntry = { ts: time, tag, level, message }
      logs.value = [...logs.value.slice(-(MAX_LOGS - 1)), entry]
      if (autoScroll) {
        nextTick(() => {
          if (logContainer.value) {
            logContainer.value.scrollTop = logContainer.value.scrollHeight
          }
        })
      }
    },
  )
})

onActivated(() => {
  mounted = true
  startRefreshLoop()
  loadToastDebugMode()
})

onDeactivated(() => {
  mounted = false
  if (timer) clearTimeout(timer)
  stopTest()
})
</script>

<template>
  <div class="debug-page">
    <div class="page-header">
      <h2>{{ t('debug.title') }}</h2>
      <div class="header-actions">
        <n-space align="center" :size="8">
          <span class="debug-switch-label">Toast 调试背景</span>
          <n-switch :value="toastDebugMode" @update:value="toggleToastDebugMode" />
        </n-space>
      </div>
    </div>

    <n-tabs type="line" size="large" :default-value="'media'">
      <!-- Tab 1: 媒体调试 -->
      <n-tab-pane name="media" :tab="t('debug.title')">
        <n-space vertical :size="16">
          <n-alert v-if="errorMsg" type="error" :show-icon="true">
            {{ errorMsg }}
          </n-alert>

          <n-card :title="t('debug.notificationTest.title')" size="small">
            <n-space align="center" :size="16">
              <n-space align="center" :size="8">
                <span class="debug-switch-label">{{ t('debug.notificationTest.interval') }}</span>
                <n-input-number
                  v-model:value="testInterval"
                  :min="1"
                  :disabled="testRunning"
                  style="width: 7rem"
                >
                  <template #suffix>{{ t('debug.notificationTest.seconds') }}</template>
                </n-input-number>
              </n-space>
              <n-button
                v-if="!testRunning"
                size="small"
                type="primary"
                @click="startTest"
              >{{ t('debug.notificationTest.start') }}</n-button>
              <n-button
                v-else
                size="small"
                type="error"
                @click="stopTest"
              >{{ t('debug.notificationTest.stop') }}</n-button>
            </n-space>
          </n-card>

          <template v-if="data">
            <!-- 最终判定 -->
            <n-card :title="t('debug.finalResult')" size="small">
              <n-space align="center" :size="24">
                <div class="result-item">
                  <div class="result-label">{{ t('debug.mediaActive') }}</div>
                  <n-tag :type="data.media_active ? 'success' : 'default'" size="large">
                    {{ data.media_active ? t('debug.mediaActiveTrue') : 'false' }}
                  </n-tag>
                </div>
                <div class="result-item">
                  <div class="result-label">{{ t('debug.mkCount') }}</div>
                  <n-tag size="large">{{ data.mouse_keyboard_count }}</n-tag>
                </div>
                <div class="result-item">
                  <div class="result-label">{{ t('debug.estimatedStatus') }}</div>
                  <n-tag :type="data.mouse_keyboard_count >= 3 || data.media_active ? 'success' : 'default'" size="large">
                    {{ data.mouse_keyboard_count >= 3 || data.media_active ? t('timeline.active') : t('timeline.rest') }}
                  </n-tag>
                </div>
              </n-space>
            </n-card>

            <!-- 音频会话 -->
            <n-card :title="t('debug.audioSessions')" size="small">
              <n-space vertical :size="12">
                <n-descriptions :column="3" size="small" bordered>
                  <n-descriptions-item :label="t('debug.available')">
                    <n-tag :type="data.audio_error ? 'error' : 'success'">
                      {{ data.audio_error ? t('debug.no') : t('debug.yes') }}
                    </n-tag>
                  </n-descriptions-item>
                  <n-descriptions-item :label="t('debug.sessionCount')">{{ data.audio_sessions.length }}</n-descriptions-item>
                  <n-descriptions-item :label="t('debug.audioActive')">
                    <n-tag :type="data.audio_active ? 'success' : 'default'">
                      {{ data.audio_active ? t('debug.yes') : t('debug.no') }}
                    </n-tag>
                  </n-descriptions-item>
                </n-descriptions>

                <n-text v-if="data.audio_error" type="error">
                  {{ t('debug.errorPrefix') }}{{ data.audio_error }}
                </n-text>

                <n-table v-if="data.audio_sessions.length > 0" :single-line="false" size="small">
                  <thead>
                    <tr>
                      <th>{{ t('debug.processName') }}</th>
                      <th>PID</th>
                      <th>{{ t('debug.peak') }}</th>
                      <th>{{ t('debug.whitelisted') }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(s, i) in data.audio_sessions" :key="i">
                      <td>{{ s.process_name }}</td>
                      <td>{{ s.pid }}</td>
                      <td>{{ s.peak.toFixed(4) }}</td>
                      <td>
                        <n-tag :type="s.whitelisted ? 'success' : 'default'" size="small">
                          {{ s.whitelisted ? t('debug.yes') : t('debug.no') }}
                        </n-tag>
                      </td>
                    </tr>
                  </tbody>
                </n-table>

                <n-empty v-else-if="!data.audio_error" :description="t('debug.noAudioSessions')" size="small" />
              </n-space>
            </n-card>

            <!-- 焦点窗口 -->
            <n-card :title="t('debug.focusWindow')" size="small">
              <n-space vertical :size="12">
                <n-descriptions :column="1" size="small" bordered>
                  <n-descriptions-item :label="t('debug.windowTitle')">{{ data.focus_window_title }}</n-descriptions-item>
                  <n-descriptions-item :label="t('debug.appName')">{{ data.focus_app_name }}</n-descriptions-item>
                  <n-descriptions-item :label="t('debug.processPath')">{{ data.focus_process_path }}</n-descriptions-item>
                </n-descriptions>
              </n-space>
            </n-card>
          </template>
        </n-space>
      </n-tab-pane>

      <!-- Tab 2: 运行日志 -->
      <n-tab-pane name="log" :tab="t('debug.logTab')">
        <div class="log-panel">
          <div class="log-toolbar">
            <n-space align="center" :size="8">
              <n-checkbox
                :checked="showLevels.error"
                @update:checked="showLevels.error = $event"
                size="small"
              >
                <span class="log-filter-label log-filter-error">{{ t('debug.logFilterError') }}</span>
              </n-checkbox>
              <n-checkbox
                :checked="showLevels.warn"
                @update:checked="showLevels.warn = $event"
                size="small"
              >
                <span class="log-filter-label log-filter-warn">{{ t('debug.logFilterWarn') }}</span>
              </n-checkbox>
              <n-checkbox
                :checked="showLevels.info"
                @update:checked="showLevels.info = $event"
                size="small"
              >
                <span class="log-filter-label log-filter-info">{{ t('debug.logFilterInfo') }}</span>
              </n-checkbox>
            </n-space>
            <n-space align="center" :size="8">
              <span class="log-count">{{ filteredLogs.length }} / {{ logs.length }}</span>
              <n-button size="tiny" quaternary @click="clearLogs">{{ t('debug.logClear') }}</n-button>
            </n-space>
          </div>
          <div
            ref="logContainer"
            class="log-list"
            @scroll="onLogScroll"
          >
            <template v-if="filteredLogs.length > 0">
              <div
                v-for="(entry, i) in filteredLogs"
                :key="i"
                class="log-line"
                :class="`log-${entry.level}`"
              >
                <span class="log-ts">{{ entry.ts }}</span>
                <span class="log-tag">{{ entry.tag }}</span>
                <span class="log-msg">{{ entry.message }}</span>
              </div>
            </template>
            <div v-else class="log-empty">
              {{ t('debug.logEmpty') }}
            </div>
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>
.debug-page {
  padding: 1.25rem;
  max-width: 56.25rem;
  display: flex;
  flex-direction: column;
  height: 100%;
  box-sizing: border-box;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  flex-shrink: 0;
}

.page-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: #2e1065;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.debug-switch-label {
  font-size: 0.8125rem;
  color: #6b5b8a;
}

.result-item {
  text-align: center;
}

.result-label {
  font-size: 0.75rem;
  color: #8b7aab;
  margin-bottom: 0.375rem;
}

/* ——— Log panel ——— */
.log-panel {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 10rem);
  min-height: 18.75rem;
}

.log-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
  flex-shrink: 0;
  border-bottom: 0.0625rem solid #ebe6f2;
  margin-bottom: 0.25rem;
}

.log-filter-label {
  font-size: 0.75rem;
}

.log-filter-error {
  color: #dc2626;
}

.log-filter-warn {
  color: #d97706;
}

.log-filter-info {
  color: #6b7280;
}

.log-count {
  font-size: 0.6875rem;
  color: #9ca3af;
  font-variant-numeric: tabular-nums;
}

.log-list {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', monospace;
  font-size: 0.6875rem;
  line-height: 1.6;
}

.log-line {
  display: flex;
  gap: 0.5rem;
  padding: 0.09375rem 0;
  white-space: nowrap;
}

.log-line:hover {
  background: rgba(0, 0, 0, 0.02);
}

.log-ts {
  color: #9ca3af;
  flex-shrink: 0;
  width: 5rem;
}

.log-tag {
  flex-shrink: 0;
  min-width: 5.5rem;
  max-width: 7rem;
  overflow: hidden;
  text-overflow: ellipsis;
  color: #6b7280;
}

.log-msg {
  color: #374151;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* level colors */
.log-line.log-error .log-tag,
.log-line.log-error .log-msg {
  color: #dc2626;
}

.log-line.log-warn .log-tag,
.log-line.log-warn .log-msg {
  color: #d97706;
}

.log-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #9ca3af;
  font-size: 0.8125rem;
}
</style>
