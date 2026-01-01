<script setup lang="ts">
import { ref } from 'vue'
import { useAgentStore } from '../stores/agent'

const props = defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const store = useAgentStore()
// Local state for modal
const aiSummarization = ref(true)
const headlessMode = ref(false)
const selectedModel = ref('llama3.2:1b')
const obsidianApiKey = ref('')

// Initialize when opening
// Note: We should ideally watch for modelValue becoming true, but for now we init on mount or rely on watcher
import { watch } from 'vue'
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    aiSummarization.value = store.settings.aiSummarization
    headlessMode.value = store.settings.headlessMode
    selectedModel.value = store.settings.selectedModel
    obsidianApiKey.value = store.settings.obsidianApiKey
  }
})

function close() {
  emit('update:modelValue', false)
}

function save() {
  store.updateSettings({
    aiSummarization: aiSummarization.value,
    headlessMode: headlessMode.value,
    selectedModel: selectedModel.value,
    obsidianApiKey: obsidianApiKey.value
  })
  close()
}
</script>

<template>
  <div v-if="modelValue" class="modal-overlay" @click.self="close">
    <div class="modal glass-panel">
      <header class="modal-header">
        <h3>Agent Settings</h3>
        <button class="close-btn" @click="close">×</button>
      </header>

      <div class="modal-body">
        <!-- API Key Setting -->
        <div class="setting-item">
          <div class="setting-info">
            <label>Obsidian API Key</label>
            <p>Bearer Token (Local REST API)</p>
          </div>
          <input 
            v-model="obsidianApiKey" 
            type="password" 
            placeholder="Key..." 
            class="glass-input"
            style="width: 200px;" 
          />
        </div>

        <!-- AI Setting -->
        <div class="setting-item">
          <div class="setting-info">
            <label>AI Summarization</label>
            <p>On: Synthesize • Off: Save Raw Markdown</p>
          </div>
          <label class="switch">
            <input type="checkbox" v-model="aiSummarization">
            <span class="slider round"></span>
          </label>
        </div>

        <!-- Model Setting -->
        <div class="setting-item">
          <div class="setting-info">
            <label>Ollama Model</label>
            <p>Select inference engine</p>
          </div>
          <select v-model="selectedModel" class="glass-input">
            <option value="llama3.2:1b">Llama 3.2 (1B)</option>
            <option value="llama3.2:3b">Llama 3.2 (3B)</option>
            <option value="deepseek-r1">DeepSeek R1</option>
          </select>
        </div>

        <!-- Headless Setting -->
        <div class="setting-item">
          <div class="setting-info">
            <label>Headless Mode</label>
            <p>Hide browser during retrieval</p>
          </div>
          <label class="switch">
            <input type="checkbox" v-model="headlessMode">
            <span class="slider round"></span>
          </label>
        </div>
      </div>

      <footer class="modal-footer">
        <button class="btn-text" @click="close">Cancel</button>
        <button class="btn-primary" @click="save">Save Changes</button>
      </footer>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0; left: 0; width: 100%; height: 100%;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease;
}

.modal {
  width: 90%;
  max-width: 450px;
  border-radius: 20px;
  overflow: hidden;
  animation: scaleIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.modal-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-subtle);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h3 {
  margin: 0;
  color: var(--text-main);
  font-size: 1.25rem;
}

.close-btn {
  background: none; border: none; color: var(--text-muted);
  font-size: 1.5rem; cursor: pointer;
}

.modal-body {
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.setting-info label {
  display: block;
  font-weight: 600;
  color: var(--text-main);
  margin-bottom: 0.25rem;
}

.setting-info p {
  margin: 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.glass-input {
  background: rgba(0,0,0,0.3);
  border: 1px solid var(--border-subtle);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  font-family: inherit;
  outline: none;
}

/* Switch Toggle */
.switch { position: relative; display: inline-block; width: 44px; height: 24px; }
.switch input { opacity: 0; width: 0; height: 0; }
.slider {
  position: absolute; cursor: pointer;
  top: 0; left: 0; right: 0; bottom: 0;
  background-color: #334155;
  transition: .4s;
  border-radius: 34px;
}
.slider:before {
  position: absolute; content: "";
  height: 18px; width: 18px;
  left: 3px; bottom: 3px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}
input:checked + .slider { background-color: var(--accent-primary); }
input:checked + .slider:before { transform: translateX(20px); }

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid var(--border-subtle);
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  background: rgba(0,0,0,0.2);
}

.btn-text {
  background: none; border: none; color: var(--text-muted);
  cursor: pointer; font-weight: 500;
}
.btn-text:hover { color: white; }

.btn-primary {
  padding: 0.75rem 1.5rem;
  border-radius: 10px;
  cursor: pointer;
}

@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
@keyframes scaleIn { from { transform: scale(0.95); opacity: 0; } to { transform: scale(1); opacity: 1; } }
</style>
