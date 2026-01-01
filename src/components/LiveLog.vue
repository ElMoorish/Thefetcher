<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { useAgentStore, type WorkflowLog } from '../stores/agent'

const store = useAgentStore()
const logContainer = ref<HTMLElement | null>(null)
let unlisten: (() => void) | null = null

const labels: Record<string, string> = {
  discovery: '🔍 Discovery',
  acquisition: '📥 Acquisition',
  synthesis: '🧠 Synthesis',
  persistence: '💾 Persistence',
  error: '❌ Error'
}

onMounted(async () => {
  unlisten = await listen<WorkflowLog>('workflow_log', (e) => {
    store.addLog(e.payload as Omit<WorkflowLog, 'timestamp'>)
    setTimeout(() => {
      logContainer.value?.scrollTo(0, logContainer.value.scrollHeight)
    }, 50)
  })
})

onUnmounted(() => unlisten?.())
</script>

<template>
  <div class="live-log">
    <header>
      <h3>Agent Activity</h3>
      <span :class="['status-badge', store.isRunning ? 'running' : 'idle']">
        {{ store.isRunning ? '● Running' : '○ Idle' }}
      </span>
    </header>
    
    <div ref="logContainer" class="logs">
      <div v-if="store.logs.length === 0" class="empty">
        No activity yet. Enter a research query to begin.
      </div>
      
      <div
        v-for="(log, i) in store.logs"
        :key="i"
        :class="['entry', log.status]"
      >
        <span class="step">{{ labels[log.step] || log.step }}</span>
        <span class="msg">{{ log.message }}</span>
        <span class="time">{{ log.timestamp.toLocaleTimeString() }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.live-log {
  background: #1a1a2e;
  border-radius: 16px;
  padding: 1.5rem;
  height: 420px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #333;
  padding-bottom: 0.75rem;
  margin-bottom: 1rem;
}

header h3 {
  margin: 0;
  color: #fff;
  font-size: 1.1rem;
  font-weight: 600;
}

.status-badge {
  padding: 0.35rem 0.85rem;
  border-radius: 999px;
  font-size: 0.75rem;
  font-weight: 600;
}

.status-badge.running {
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: #fff;
  animation: pulse 1.5s infinite;
}

.status-badge.idle {
  background: #2a2a3e;
  color: #888;
}

.logs {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding-right: 0.5rem;
}

.logs::-webkit-scrollbar {
  width: 6px;
}

.logs::-webkit-scrollbar-track {
  background: #16213e;
  border-radius: 3px;
}

.logs::-webkit-scrollbar-thumb {
  background: #4a4a6a;
  border-radius: 3px;
}

.empty {
  color: #666;
  text-align: center;
  padding: 3rem 1rem;
  font-style: italic;
}

.entry {
  display: grid;
  grid-template-columns: 150px 1fr auto;
  gap: 1rem;
  padding: 0.85rem 1rem;
  background: #16213e;
  border-radius: 10px;
  color: #e0e0e0;
  font-size: 0.875rem;
  transition: all 0.2s;
}

.entry.complete { border-left: 3px solid #22c55e; }
.entry.running { border-left: 3px solid #6366f1; background: #1e2847; }
.entry.error { border-left: 3px solid #ef4444; }
.entry.pending { border-left: 3px solid #666; }

.step {
  font-weight: 600;
  color: #fff;
}

.msg {
  color: #a0a0b0;
}

.time {
  font-size: 0.7rem;
  color: #666;
  white-space: nowrap;
}

@keyframes pulse {
  0%, 100% { opacity: 1; box-shadow: 0 0 0 0 rgba(99, 102, 241, 0.4); }
  50% { opacity: 0.85; box-shadow: 0 0 0 8px rgba(99, 102, 241, 0); }
}
</style>
