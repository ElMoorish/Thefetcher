<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAgentStore, type SearchResult } from '../stores/agent'

const store = useAgentStore()
const query = ref('')
const modes = [
  { id: 'web', label: 'Web', icon: '🌐' },
  { id: 'vault', label: 'Vault', icon: '🧠' },
  { id: 'agent', label: 'Agent', icon: '🤖' }
]
const currentMode = ref('web')

async function submit() {
  if (!query.value.trim() || store.isRunning) return
  
  store.clearLogs()
  store.setRunning(true)
  store.lastQuery = query.value
  
  try {
    let results: SearchResult[] = []
    
    if (currentMode.value === 'web') {
      results = await invoke<SearchResult[]>('perform_search', { query: query.value })
    } else if (currentMode.value === 'vault') {
       if (!store.settings.obsidianApiKey) {
         throw new Error("Obsidian API Key required. Please set it in Settings.")
       }
       store.addLog({ step: 'discovery', status: 'running', message: 'Chatting with Vault...' })
       
       // Using 'chat_with_vault' command
       // Note: Result is FetchResult (single), we wrap it as SearchResult
       const chatRes = await invoke<any>('chat_with_vault', { 
         query: query.value,
         apiKey: store.settings.obsidianApiKey,
         model: store.settings.selectedModel
       })
       
       results = [{
         title: "Vault Answer",
         url: "local-rag",
         content: chatRes.summary // Using 'summary' as content/snippet
       }]

    } else {
       // Agent mode
       if (!store.settings.obsidianApiKey) {
           store.addLog({ step: 'discovery', status: 'pending', message: 'Warning: No API Key? Agent might fail search.' })
       }
       store.addLog({ step: 'discovery', status: 'running', message: 'Agent initializing...' })
       
       const agentRes = await invoke<any>('run_agent_loop', {
           query: query.value,
           model: store.settings.selectedModel,
           api_key: store.settings.obsidianApiKey
       })

       results = [{
           title: "Agent Answer",
           url: "autonomous-agent",
           content: agentRes.summary
       }]
    }
    
    store.setSearchResults(results)
  } catch (e) {
    store.addLog({ step: 'error', status: 'error', message: String(e) })
  } finally {
    store.setRunning(false)
  }
}
</script>

<template>
  <div class="request-container">
    <!-- Mode Switcher -->
    <div class="mode-switcher">
      <button 
        v-for="mode in modes" 
        :key="mode.id"
        class="mode-btn"
        :class="{ active: currentMode === mode.id }"
        @click="currentMode = mode.id"
      >
        <span>{{ mode.icon }}</span>
        <span>{{ mode.label }}</span>
      </button>
    </div>

    <form @submit.prevent="submit" class="input-form">
      <div class="input-wrapper">
        <span class="icon">🔍</span>
        <input
          v-model="query"
          :placeholder="currentMode === 'web' ? 'Research the web...' : (currentMode === 'vault' ? 'Ask your vault...' : 'Command the agent...')"
          :disabled="store.isRunning"
          class="glass-input"
        />
      </div>
      <button :disabled="store.isRunning || !query.trim()" class="btn-primary">
        <span v-if="store.isRunning" class="spinner"></span>
        {{ store.isRunning ? 'Running...' : 'Go' }}
      </button>
    </form>
  </div>
</template>

<style scoped>
.request-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.mode-switcher {
  display: flex;
  gap: 0.5rem;
  background: var(--bg-surface);
  padding: 0.5rem; /* Increased padding slightly */
  border-radius: 16px; /* Update radius */
  width: fit-content;
  margin: 0 auto; /* Center it */
  border: 1px solid var(--border-subtle);
}

.mode-btn {
  background: transparent;
  border: none;
  color: var(--text-muted);
  padding: 0.5rem 1rem;
  border-radius: 12px;
  cursor: pointer;
  display: flex;
  gap: 0.5rem;
  font-family: 'Outfit', sans-serif;
  font-weight: 500;
  transition: all 0.2s;
}

.mode-btn:hover {
  color: var(--text-main);
  background: rgba(255,255,255,0.05); /* Slight hover bg */
}

.mode-btn.active {
  background: var(--accent-primary);
  color: white;
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
}

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

/* ... existing styles for icon, glass-input, etc ... */
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

button.btn-primary { /* Explicit specificity */
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
  min-width: 100px; /* Reduced min-width */
}

button.btn-primary:hover:not(:disabled) {
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
