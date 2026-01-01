import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface WorkflowLog {
    step: string
    status: 'pending' | 'running' | 'complete' | 'error'
    message: string
    timestamp: Date
}

export interface FetchResult {
    success: boolean
    title: string
    summary: string
    file_path: string
    error?: string
}

export interface SearchResult {
    url: string
    title: string
}

export interface AgentSettings {
    aiSummarization: boolean
    headlessMode: boolean
    selectedModel: string
    obsidianApiKey: string
}

export const useAgentStore = defineStore('agent', () => {
    const logs = ref<WorkflowLog[]>([])
    const isRunning = ref(false)
    const lastResult = ref<FetchResult | null>(null)
    const lastQuery = ref('')
    const searchResults = ref<SearchResult[]>([])
    const settings = ref<AgentSettings>({
        aiSummarization: true,
        headlessMode: false,
        selectedModel: 'llama3.2:1b',
        obsidianApiKey: ''
    })

    const addLog = (log: Omit<WorkflowLog, 'timestamp'>) => {
        logs.value.push({ ...log, timestamp: new Date() })
    }

    const clearLogs = () => {
        logs.value = []
        lastResult.value = null
        searchResults.value = []
    }

    const setRunning = (v: boolean) => {
        isRunning.value = v
    }

    const setResult = (r: FetchResult) => {
        lastResult.value = r
    }

    const setSearchResults = (results: SearchResult[]) => {
        searchResults.value = results
    }

    const updateSettings = (s: AgentSettings) => {
        settings.value = s
    }

    return { logs, isRunning, lastResult, lastQuery, searchResults, settings, addLog, clearLogs, setRunning, setResult, setSearchResults, updateSettings }
})
