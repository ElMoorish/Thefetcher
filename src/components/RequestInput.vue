<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAgentStore, type SearchResult } from '../stores/agent'

const store = useAgentStore()
const query = ref('')

async function submit() {
  if (!query.value.trim() || store.isRunning) return
  
  store.clearLogs()
  store.setRunning(true)
  store.lastQuery = query.value
  
  try {
    const results = await invoke<SearchResult[]>('perform_search', { query: query.value })
    store.setSearchResults(results)
  } catch (e) {
    store.addLog({ step: 'error', status: 'error', message: String(e) })
  } finally {
    store.setRunning(false)
  }
}
</script>

<template>
  <form @submit.prevent="submit" class="input-form">
    <div class="input-wrapper">
      <span class="icon">🔍</span>
      <input
        v-model="query"
        placeholder="Research Tauri v2 documentation..."
        :disabled="store.isRunning"
        class="glass-input"
      />
    </div>
    <button :disabled="store.isRunning || !query.trim()" class="btn-primary">
      <span v-if="store.isRunning" class="spinner"></span>
      {{ store.isRunning ? 'Searching...' : 'Search' }}
    </button>
  </form>
</template>

<style scoped>
.input-form {
  display: flex;
  gap: 1rem;
}

.input-wrapper {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.icon {
  position: absolute;
  left: 1.25rem;
  font-size: 1.1rem;
  opacity: 0.5;
  color: var(--text-muted);
}

.glass-input {
  width: 100%;
  padding: 1.1rem 1.25rem 1.1rem 3.25rem;
  border: 1px solid var(--border-subtle);
  border-radius: 14px;
  background: var(--bg-surface);
  color: var(--text-main);
  font-size: 1rem;
  transition: all 0.2s;
  font-family: 'Inter', sans-serif;
}

.glass-input::placeholder {
  color: var(--text-muted);
}

.glass-input:focus {
  outline: none;
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 4px rgba(139, 92, 246, 0.15);
  background: rgba(20, 20, 30, 0.8);
}

input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 1.1rem 2.5rem;
  background: linear-gradient(135deg, #6366f1, #8b5cf6);
  color: #fff;
  border: none;
  border-radius: 14px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 140px;
}

button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(99, 102, 241, 0.4);
}

button:active:not(:disabled) {
  transform: translateY(0);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: #fff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
