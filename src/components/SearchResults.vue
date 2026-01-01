<script setup lang="ts">
import { useAgentStore, type SearchResult } from '../stores/agent'

const store = useAgentStore()
const emit = defineEmits<{
  (e: 'select', result: SearchResult): void
}>()

function select(result: SearchResult) {
  emit('select', result)
}
</script>

<template>
  <div v-if="store.searchResults.length > 0" class="results-container">
    <div class="results-header">
      <h3>Select a Source</h3>
      <span class="count">{{ store.searchResults.length }} found</span>
    </div>
    
    <div class="results-grid">
      <div 
        v-for="(result, index) in store.searchResults" 
        :key="index"
        class="result-item glass-panel"
        @click="select(result)"
      >
        <div class="icon">📄</div>
        <div class="info">
          <h4>{{ result.title }}</h4>
          <p v-if="result.content" class="snippet">{{ result.content }}</p>
          <p v-else class="url">{{ result.url }}</p>
        </div>
        <div class="arrow">→</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.results-container {
  animation: fadeIn 0.4s ease;
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding: 0 0.5rem;
}

.results-header h3 {
  font-size: 1.1rem;
  color: var(--text-muted);
  font-weight: 500;
  margin: 0;
}

.count {
  font-size: 0.85rem;
  color: var(--accent-gold);
  background: rgba(251, 191, 36, 0.1);
  padding: 0.2rem 0.6rem;
  border-radius: 12px;
}

.results-grid {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
  max-height: 400px;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid var(--border-subtle);
  background: rgba(30, 41, 59, 0.4);
}

.result-item:hover {
  background: rgba(129, 140, 248, 0.1);
  border-color: var(--accent-indigo);
  transform: translateX(4px);
}

.icon {
  font-size: 1.25rem;
  opacity: 0.8;
}

.info {
  flex: 1;
  overflow: hidden;
}

.info h4 {
  margin: 0 0 0.25rem 0;
  font-size: 1rem;
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
}

.info .url {
  margin: 0;
  font-size: 0.8rem;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  opacity: 0.7;
}

.snippet {
  margin: 0;
  font-size: 0.9rem;
  color: var(--text-muted);
  line-height: 1.4;
  white-space: pre-wrap; /* Preserve formatting */
  display: -webkit-box;
  -webkit-line-clamp: 4; /* Limit lines */
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.arrow {
  color: var(--accent-emerald);
  font-weight: bold;
  opacity: 0;
  transform: translateX(-10px);
  transition: all 0.2s;
}

.result-item:hover .arrow {
  opacity: 1;
  transform: translateX(0);
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
