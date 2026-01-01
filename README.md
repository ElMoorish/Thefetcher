# TheFetcher 🌟
### Autonomous Research Engine

> **"Search less, Fetch more."**
> A local-first, privacy-focused research assistant that bridges the gap between the web and your obsidian vault.

<img width="1203" height="832" alt="image" src="https://github.com/user-attachments/assets/944af2ee-88b3-4665-9e61-3d77031f123a" />


## ✨ Overview

**TheFetcher** is a desktop application built with **Tauri v2** (Rust + Vue 3) aimed at streamlining technical research. It bridges the gap between the web and your obsidian vault using autonomous agents and local RAG.

8.  **Save** structured knowledge directly into your **Obsidian** vault.

---

## 🕹️ Modes of Operation

### 🌍 Web Mode (The Discoverer)
Standard search and retrieval. Perfect for quick lookups or gathering raw list of documentation URLs.
- **Tools**: SearXNG -> Scrapling -> Ollama Summarize.

### 📚 Vault Mode (The Librarian)
Local-first Knowledge retrieval. Query your Obsidian vault using RAG (Retrieval Augmented Generation).
- **Process**: Semantic search across notes -> Context injection -> Ollama synthesis.
- **Requirement**: Obsidian Local REST API must be active.

### 🤖 Agent Mode (The Researcher)
An autonomous ReAct agent that loops through tools until the query is fully answered.
- **Toolbox**: `search`, `fetch`, `vault_search`, `vault_read`.
- **Hybrid Reasoning**: Can search the web for recent updates while simultaneously referencing your local notes for context.
- **Transparency**: Watch the "thought process" in the LiveLog as the agent decides which tool to use next.

---

---

## 🎨 Design Philosophy: Neo-Moorish Minimalism

The UI features a unique **Neo-Moorish** aesthetic, blending geometric complexity with modern glassmorphism:
*   **Zellige Background**: A mathematically generated geometric pattern (CSS-only) representing infinite knowledge.
*   **Glassmorphism**: Translucent panels that float above the complex background, focusing attention.
*   **Typography**: *Outfit* for headers (modern, geometric) and *Inter* for readability.
*   **Palette**: Deep Indigo, Emerald, and Gold accents.

---

## 🛠️ Tech Stack

*   **Frontend**: Vue 3, TypeScript, Pinia, Vanilla CSS (Variables).
*   **Backend**: Rust (Tauri), Reqwest, Serde.
*   **AI**: Ollama (Local Inference).
*   **Search**: SearXNG (Self-hosted/Local).
*   **Storage**: Obsidian (Local REST API).

---

## 🚀 Getting Started

### Prerequisites

1.  **Ollama**: Install and pull a model (e.g., `llama3.2`).
    ```bash
    ollama pull llama3.2:1b
    ```
2.  **SearXNG**: Run via Docker.
    ```bash
    docker run -d -p 8080:8080 searxng/searxng
    ```
3.  **Obsidian**:
    *   Install the **Local REST API** plugin.
    *   Enable SSL (self-signed is fine, TheFetcher handles it).
    *   Copy your **Bearer Token**.

### Installation

1.  Clone the repository.
    ```bash
    git clone https://github.com/ElMoorish/Thefetcher.git
    cd TheFetcher
    ```
2.  Install dependencies.
    ```bash
    npm install
    # Ensure Rust is installed (rustup)
    ```
3.  Run in Development Mode.
    ```bash
    npm run tauri dev
    ```

### Configuration

1.  Click the **Settings (⚙️)** icon in the app header.
2.  **AI Summarization**: Toggle ON to use Ollama, OFF for raw markdown.
3.  **Obsidian API Key**: Paste your Bearer Token here. (Required for saving).
4.  **Headless Mode**: Toggle to hide browser windows during fetching (Experimental).

---

## 🛡️ Security & Privacy

*   **Local First**: All AI processing happens on your machine (Ollama). No data is sent to cloud AI providers.
*   **Private Search**: Uses SearXNG to anonymize search queries.
*   **Secure Storage**: API Keys are stored in memory during runtime and not hardcoded in the final release.

---

## 📦 Building for Production

To create a standalone executable (`.exe`):

```bash
npm run tauri build
```
The output will be in `src-tauri/target/release/bundle/nsis/`.

---

## 🤝 Contributing

Pull requests are welcome! Please adhere to the **Neo-Moorish** design system when adding UI components.

## 📄 License

MIT

---

## ☕ Support the Project

If you find **TheFetcher** useful and would like to support its development, donations are welcome:

**BTC Address**: `17ospCx6MbNQfqETd1UjAQQSus8ENHXgnh`

