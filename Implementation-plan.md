Implementation-plan.md: "TheFetcher"
1. Project Vision
TheFetcher is a local-first, autonomous research engine. It utilizes an "Agentic RAG" architecture to autonomously discover technical documentation, ingest it using stealth browser automation, summarize it via local inference, and persist it into a semantic knowledge base.
Core Stack:
• Orchestrator: Google Antigravity (Development & Agent Management)
• Runtime: Tauri v2 (Rust + Vue.js) for the executable interface
• Intelligence: Local Ollama (llama3.1) + Gemini 3 Pro (Planner)
• Connectivity: Model Context Protocol (MCP) via Stdio Transport
• Storage: Obsidian (Markdown + Vector Embeddings)

--------------------------------------------------------------------------------
2. Phase 1: Infrastructure & Prerequisites
Objective: Establish the "Brain" and "Memory" layers before building the application logic.
2.1 Inference Layer (Ollama)
The agent must verify the local inference engine is ready to handle summarization tasks to save cloud costs and preserve privacy.
• Action: Install/Verify Ollama.
• Models:
    ◦ ollama pull llama3.1 (For summarization and tool usage).
    ◦ ollama pull nomic-embed-text (For semantic vector embeddings).
• Network Config: Bind to 127.0.0.1 only to prevent LAN leakage.
2.2 Memory Layer (Obsidian)
The agent needs a structured repository for the fetched data.
• Action: Configure the target Obsidian Vault.
• Required Plugins:
    1. Local REST API: Must be installed to allow external Read/Write access.
        ▪ Config: Enable HTTPS, generate API Key.
    2. Smart Connections: Enables RAG (Retrieval-Augmented Generation).
        ▪ Config: Set Embedding Model to nomic-embed-text (Ollama).

--------------------------------------------------------------------------------
3. Phase 2: The Connectivity Layer (MCP Configuration)
Objective: configure the mcp_config.json to give the Antigravity Agent access to the necessary tools.
3.1 Tool Selection
We will use a "Composite Stack" for maximum efficacy:
1. Discovery: Brave Search (via brave-search-mcp) to find documentation URLs using "Goggles" to filter for tech docs.
2. Ingestion: Scrapling (via scrapling-fetch-mcp) for stealth extraction. It is preferred over Firecrawl for this local implementation to minimize infrastructure overhead while bypassing anti-bot protections.
3. Storage: Obsidian MCP (via @connorbritain/obsidian-mcp-server) to write files and update embeddings.
3.2 Configuration File (mcp_config.json)
The agent must create this file in the Antigravity configuration directory (%USERPROFILE%\.gemini\antigravity\ or equivalent).
{
  "mcpServers": {
    "brave-search": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-brave-search"],
      "env": {
        "BRAVE_API_KEY": "YOUR_API_KEY"
      }
    },
    "web-fetcher": {
      "command": "uvx",
      "args": ["scrapling-fetch-mcp"],
      "env": {
        "HEADLESS_MODE": "true"
      }
    },
    "obsidian-vault": {
      "command": "npx",
      "args": ["-y", "@connorbritain/obsidian-mcp-server"],
      "env": {
        "OBSIDIAN_API_KEY": "YOUR_GENERATED_KEY",
        "OBSIDIAN_PROTOCOL": "https",
        "OBSIDIAN_VERIFY_SSL": "false"
      }
    },
    "ollama-bridge": {
      "command": "uvx",
      "args": ["ollama-mcp-bridge"],
       "env": {
        "OLLAMA_BASE_URL": "http://localhost:11434"
      }
    }
  }
}
Note: OBSIDIAN_VERIFY_SSL must be set to "false" to accept the local self-signed certificate.

--------------------------------------------------------------------------------
4. Final Architecture: The "Agentic RAG" Workflow
Objective: Define the autonomous loop implemented in lib.rs.
The agent uses a ReAct (Reasoning and Acting) loop:
1. Turn 0 (Reasoning): Agent analyzes the query.
2. Turn 1 (Acting): Agent calls tools:
    - `vault_search`: Checks if the info exists locally.
    - `vault_read`: Ingests local context.
    - `search`: Queries the web if local info is insufficient.
    - `fetch`: Ingests live documentation.
3. Turn 2+ (Synthesis): Agent combines local and web data into a final refined answer.

--------------------------------------------------------------------------------
5. The Executable Interface (UI/UX)
Objective: A high-performance, aesthetically pleasing dashboard.
5.1 Runtime: Tauri v2
- Footprint: <30MB RAM overhead.
- Frontend: Vue 3 + Pinia for reactive streaming logs.
5.2 Aesthetic: Neo-Moorish Minimalism
- Uses custom Zellige CSS backgrounds and premium typography (Outfit/Inter).
- LiveLog: Provides transparency into the Agent's thought process.

--------------------------------------------------------------------------------
6. Security & Distribution
- API Keys: Secured via runtime memory; no hardcoding.
- Sanitization: All local machine references and secrets purged for GitHub.
- Packaging: `npm run tauri build` generates a production-ready `.exe`.

--------------------------------------------------------------------------------
7. Project Status: COMPLETED 🚀
1. [x] Initialize Environment: Ollama & Obsidian ready.
2. [x] Generate Config: Sanitized template.
3. [x] Scaffold Tauri App: Vue 3 + Pinia.
4. [x] Implement Agent Loop: ReAct + Vault Bridge.
5. [x] UI/UX Polish: Neo-Moorish performance targets met.
6. [x] Production: Build successful.
