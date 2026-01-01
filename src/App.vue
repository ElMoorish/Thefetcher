<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core' // Import invoke
import RequestInput from './components/RequestInput.vue'
import LiveLog from './components/LiveLog.vue'
import SettingsModal from './components/SettingsModal.vue'
import AppLogo from './components/AppLogo.vue'
import SearchResults from './components/SearchResults.vue'
import { useAgentStore, type SearchResult, type FetchResult } from './stores/agent' // Import types

const store = useAgentStore()
const showSettings = ref(false)

async function onSelect(result: SearchResult) {
  store.setRunning(true)
  store.clearLogs() // Clear logs for the fetch phase (or keep them?) - Maybe keep search logs? Let's clear for focus.
  store.searchResults = [] // Hide results

  try {
     // Check if we already have the content (e.g. from Vault/Agent)
     if (result.content) {
         store.setResult({
             success: true,
             title: result.title,
             summary: result.content,
             file_path: 'In Memory',
             error: undefined
         })
         return
     }

     const options = {
      useAi: store.settings.aiSummarization,
      headless: store.settings.headlessMode,
      modelName: store.settings.selectedModel,
      obsidianApiKey: store.settings.obsidianApiKey
    }
    const fetchResult = await invoke<FetchResult>('process_selection', { 
        query: store.lastQuery, 
        url: result.url, 
        title: result.title,
        options 
    })
    store.setResult(fetchResult)
  } catch (e) {
    store.addLog({ step: 'error', status: 'error', message: String(e) })
  } finally {
    store.setRunning(false)
  }
}
</script>

<template>
  <div class="app">
    <header class="header">
      <button class="settings-btn" @click="showSettings = true">⚙️</button>
      <div class="logo">
        <AppLogo />
        <h1 class="moorish-text">TheFetcher</h1>
      </div>
      <p class="tagline">Autonomous Research Engine</p>
    </header>

    <SettingsModal v-model="showSettings" />

    <main class="main">
      <RequestInput />
      
      <SearchResults @select="onSelect" />
      
      <LiveLog v-if="store.isRunning || store.logs.length > 0" />
      
      <div v-if="store.lastResult" class="result-card glass-panel">
        <div class="result-header">
          <span class="result-icon">✅</span>
          <h3>{{ store.lastResult.title }}</h3>
        </div>
        <div class="result-content">
          <p class="result-summary">{{ store.lastResult.summary }}</p>
        </div>
        <div class="result-path">
          <span>📁</span>
          <code>{{ store.lastResult.file_path }}</code>
        </div>
      </div>
    </main>

    <footer class="footer">
      <p>Powered by SearXNG • Scrapling • Ollama • Obsidian</p>
      <div class="signature">
        <span>By the</span> <span class="moorish-text">Moorish</span> <span class="dot">.</span>
      </div>
    </footer>
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.header {
  padding-top: 2rem;
  padding-bottom: 1.5rem;
  text-align: center;
  position: relative;
  flex-shrink: 0;
}

.settings-btn {
  position: absolute;
  top: 1rem;
  right: 2rem;
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  color: var(--text-muted);
  font-size: 1.25rem;
  padding: 0.5rem 0.8rem;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 10;
}

.settings-btn:hover {
  background: var(--border-subtle);
  color: var(--accent-indigo);
  transform: rotate(90deg);
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1.25rem;
  margin-bottom: 0.75rem;
}

.moorish-text {
  font-family: 'Outfit', sans-serif;
  font-weight: 800;
  background: linear-gradient(135deg, var(--accent-indigo), var(--accent-emerald), var(--accent-gold));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.logo h1 {
  font-size: 3.5rem;
  letter-spacing: -2px;
  margin: 0;
}

.tagline {
  color: var(--text-muted);
  font-size: 1.1rem;
  font-weight: 300;
  letter-spacing: 1px;
  text-transform: uppercase;
}

.main {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow-y: auto;
  padding: 0.5rem 2rem 2rem 2rem;
  width: 100%;
  max-width: 900px;
  margin: 0 auto;
  scrollbar-width: thin;
  scrollbar-color: var(--border-subtle) transparent;
}

.result-card {
  padding: 2rem;
  border-radius: 20px;
  animation: slideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1);
  border: 1px solid var(--border-subtle);
}

.result-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.25rem;
}

.result-header h3 {
  font-size: 1.5rem;
  color: var(--text-main);
  margin: 0;
}

.result-summary {
  color: var(--text-muted);
  font-size: 1.05rem;
  line-height: 1.7;
  white-space: pre-wrap;
}

.result-path {
  margin-top: 1.5rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.4);
  border-radius: 12px;
  border: 1px solid var(--border-subtle);
}

.result-path code {
  color: var(--accent-gold);
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.9rem;
}

.footer {
  text-align: center;
  padding: 2rem;
  color: var(--text-muted);
  font-size: 0.85rem;
  opacity: 0.6;
  transition: opacity 0.3s;
  flex-shrink: 0;
}

.footer:hover {
  opacity: 1;
}

.signature {
  margin-top: 1rem;
  font-family: 'Outfit', sans-serif;
  font-weight: 500;
  font-size: 1rem;
}

.dot {
  color: var(--accent-gold);
  font-weight: 700;
  font-size: 1.5rem;
  line-height: 0;
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(40px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>