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
4. Phase 3: The Application Logic (The "Fetcher" Workflow)
Objective: Define the autonomous loop the agent will execute.
The agent will implement the "Fetch-Summarize-Save" pattern:
1. Step 1 (Discovery):
    ◦ Agent utilizes brave_search with arguments {"q": "Tauri v2 documentation", "goggles_id": "tech-docs"}.
    ◦ Reasoning: Filters out SEO spam, returning only high-fidelity documentation URLs.
2. Step 2 (Acquisition):
    ◦ Agent utilizes fetch_url (Scrapling) on the target URL.
    ◦ Optimization: Agent must request Markdown conversion immediately to reduce token load by 40-60% compared to raw HTML.
3. Step 3 (Synthesis):
    ◦ Agent delegates summarization to Local Ollama via the ollama-bridge or direct API call.
    ◦ Prompt: "Summarize the following documentation into a Reference Note format, preserving code blocks and key architectural concepts."
4. Step 4 (Persistence):
    ◦ Agent checks for duplicates using obsidian_search_semantic.
    ◦ Agent writes the file using obsidian_write_note to the Reference/ folder.
    ◦ Result: Smart Connections automatically indexes the new note, making it instantly available for RAG.

--------------------------------------------------------------------------------
5. Phase 4: The Executable Interface (UI/UX)
Objective: Build a standalone dashboard using Tauri v2 and Vue.js.
5.1 Runtime Selection: Tauri
We strictly choose Tauri over Electron.
• Reason: Tauri consumes ~30MB of RAM vs Electron's 200MB+. This is critical to save system RAM for the local Ollama models.
• Language: Rust (Backend) + Vue.js (Frontend).
5.2 Frontend Architecture (Vue.js)
• State Management: Use Pinia to handle the streaming state of the "Agent Activity Log".
• Components:
    ◦ RequestInput.vue: User types "Research X".
    ◦ LiveLog.vue: Visualizes the MCP tool calls (e.g., "Fetching URL...", "Summarizing with Llama 3...").
    ◦ VaultView.vue: Displays the recently added Obsidian notes via the Local REST API.
5.3 Sidecar Pattern (Packaging)
To make "TheFetcher" a "masterpiece" distributable app, we must bundle the dependencies.
• Action: Configure tauri.conf.json to bundle the MCP servers as Sidecars.
• Implementation: Use externalBin to bundle the compiled Python/Node binaries for Scrapling and the Obsidian Connector. This ensures the user does not need to manually install Python or Node.js to run TheFetcher.

--------------------------------------------------------------------------------
6. Phase 5: Security Architecture
Objective: Ensure Zero-Trust local operation.
6.1 Transport Security
• Use Stdio Transport for all MCP connections. This creates a child-process relationship, preventing the tools from being exposed on network ports.
6.2 Path Restriction
• Configure the Obsidian MCP server to strictly allow-list only the Reference/ folder. This prevents the agent from accidentally overwriting personal journals or deleting existing data.
6.3 Optional: Docker Sandboxing
• If "TheFetcher" is expected to visit high-risk websites, the Scrapling component should be moved to a Docker MCP Gateway.
• Config: The agent connects to the Docker container via SSE, ensuring malicious JS execution is trapped inside the container's ephemeral filesystem.

--------------------------------------------------------------------------------
7. Execution Checklist for the Agent
1. [ ] Initialize Environment: Verify Ollama is running and Obsidian Local API is reachable.
2. [ ] Generate Config: Write the mcp_config.json with valid API keys.
3. [ ] Scaffold Tauri App: npm create tauri-app@latest (Select Vue.js).
4. [ ] Implement Sidecars: Register Python/Node binaries in Tauri config.
5. [ ] Develop Agent Loop: Write the Rust/TypeScript logic to chain Brave -> Scrapling -> Ollama -> Obsidian.
6. [ ] Test: Run a query ("Fetch Pydantic Docs") and verify the .md file appears in Obsidian.
