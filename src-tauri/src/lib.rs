#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};

#[derive(Clone, Serialize)]
struct WorkflowLog {
    step: String,
    status: String,
    message: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct FetchResult {
    success: bool,
    title: String,
    summary: String,
    file_path: String,
    error: Option<String>,
}

// SearXNG response structures
#[derive(Deserialize, Debug)]
struct SearXNGResponse {
    #[serde(default)]
    results: Vec<SearXNGResult>,
}

#[derive(Deserialize, Serialize, Debug, Clone)] // Added Serialize, Clone
struct SearXNGResult {
    #[serde(default)]
    url: String,
    #[serde(default)]
    title: String,
}

// Ollama response structure
#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Matches JS options object
struct WorkflowOptions {
    use_ai: bool,
    #[allow(dead_code)]
    headless: bool,
    model_name: String,
    obsidian_api_key: String,
}

/// Step 1: Search Only
#[tauri::command]
async fn perform_search(
    window: tauri::WebviewWindow,
    query: String,
) -> Result<Vec<SearXNGResult>, String> {
    emit_log(
        &window,
        "discovery",
        "running",
        &format!("Searching: {}...", query),
    )?;

    // Create client
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let results = call_searxng_search(&client, &query).await?;

    emit_log(
        &window,
        "discovery",
        "complete",
        &format!("Found {} results", results.len()),
    )?;
    Ok(results)
}

/// Step 2-4: Process Selected Result
#[tauri::command]
async fn process_selection(
    app: tauri::AppHandle,
    query: String,
    url: String,
    title: String,
    options: WorkflowOptions,
) -> Result<FetchResult, String> {
    let window = app.get_webview_window("main").ok_or("No main window")?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Step 2: Acquisition
    emit_log(
        &window,
        "acquisition",
        "running",
        &format!("Fetching: {}...", url),
    )?;
    let content = fetch_url_content(&client, &url).await?;
    emit_log(
        &window,
        "acquisition",
        "complete",
        &format!("Retrieved {} chars", content.len()),
    )?;

    // Step 3: Synthesis
    let summary = if options.use_ai {
        emit_log(
            &window,
            "synthesis",
            "running",
            &format!("Summarizing with {}...", options.model_name),
        )?;
        let sum = call_ollama_summarize(&client, &content, &options.model_name).await?;
        emit_log(&window, "synthesis", "complete", "Summary generated")?;
        sum
    } else {
        emit_log(&window, "synthesis", "skipped", "Using raw content")?;
        content
    };

    // Step 4: Persistence
    emit_log(
        &window,
        "persistence",
        "running",
        "Saving to Obsidian vault...",
    )?;
    let file_path = save_to_obsidian(
        &client,
        &query,
        &summary,
        &url,
        &title,
        &options.obsidian_api_key,
    )
    .await?;
    emit_log(
        &window,
        "persistence",
        "complete",
        &format!("Saved: {}", file_path),
    )?;

    Ok(FetchResult {
        success: true,
        title: title, // Use the title passed from selection
        summary: summary.chars().take(200).collect(),
        file_path,
        error: None,
    })
}

fn emit_log(
    window: &tauri::WebviewWindow,
    step: &str,
    status: &str,
    message: &str,
) -> Result<(), String> {
    window
        .emit(
            "workflow_log",
            WorkflowLog {
                step: step.to_string(),
                status: status.to_string(),
                message: message.to_string(),
            },
        )
        .map_err(|e: tauri::Error| e.to_string())
}

/// Search using SearXNG (localhost:8080)
async fn call_searxng_search(
    client: &reqwest::Client,
    query: &str,
) -> Result<Vec<SearXNGResult>, String> {
    // Changed return type
    let search_url = format!(
        "http://localhost:8080/search?q={}&format=json&categories=general",
        urlencoding::encode(&format!("{} documentation", query))
    );

    let response = client
        .get(&search_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // Get response text first for better error handling
    let response_text = response.text().await.map_err(|e| e.to_string())?;

    // Parse JSON
    let searx_response: SearXNGResponse = serde_json::from_str(&response_text).map_err(|e| {
        format!(
            "JSON Parse Error: {}. Text: {}",
            e,
            &response_text[..100.min(response_text.len())]
        )
    })?;

    let results: Vec<SearXNGResult> = searx_response
        .results
        .into_iter()
        .filter(|r| !r.url.is_empty())
        .take(20) // Return top 20 results (requested > 5)
        .collect();

    Ok(results)
}

// ----------------------
// Talk to Vault (RAG)
// ----------------------

#[derive(Deserialize, Debug)]
struct ObsidianSearchResult {
    filename: String,
    #[allow(dead_code)]
    score: f64,
}

#[tauri::command]
async fn search_vault(query: String, api_key: String) -> Result<Vec<SearXNGResult>, String> {
    // Reusing SearXNGResult for UI consistency (url=path, title=filename)
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let search_url = format!(
        "https://127.0.0.1:27124/search/simple?query={}",
        urlencoding::encode(&query)
    );

    let response = client
        .post(&search_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    // Obsidian returns [ { "filename": "...", "score": ... } ]
    let results: Vec<ObsidianSearchResult> =
        serde_json::from_str(&text).map_err(|e| format!("Obsidian Parse Error: {}", e))?;

    let ui_results: Vec<SearXNGResult> = results
        .into_iter()
        .take(10)
        .map(|r| SearXNGResult {
            title: r.filename.clone(),
            url: r.filename, // Path serves as URL/ID
        })
        .collect();

    Ok(ui_results)
}

#[tauri::command]
async fn chat_with_vault(
    window: tauri::WebviewWindow,
    query: String,
    api_key: String,
    model: String,
) -> Result<FetchResult, String> {
    emit_log(
        &window,
        "discovery",
        "running",
        &format!("Searching vault for: {}...", query),
    )?;

    // 1. Search
    let results = search_vault(query.clone(), api_key.clone()).await?;
    if results.is_empty() {
        return Err("No matching notes found.".to_string());
    }
    emit_log(
        &window,
        "discovery",
        "complete",
        &format!("Found {} notes", results.len()),
    )?;

    // 2. Read Top 3
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;
    let mut context = String::new();

    emit_log(
        &window,
        "acquisition",
        "running",
        "Reading note contents...",
    )?;
    for (i, res) in results.iter().take(3).enumerate() {
        let path = &res.url;
        let url = format!(
            "https://127.0.0.1:27124/vault/{}",
            urlencoding::encode(path)
        );

        if let Ok(resp) = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
        {
            if let Ok(content) = resp.text().await {
                context.push_str(&format!("\n--- Note: {} ---\n{}\n", res.title, content));
            }
        }
        emit_log(
            &window,
            "acquisition",
            "running",
            &format!("Read {}/3 notes...", i + 1),
        )?;
    }
    emit_log(&window, "acquisition", "complete", "Context loaded")?;

    // 3. Synthesize
    emit_log(&window, "synthesis", "running", "Generating answer...")?;
    let prompt = format!(
        "You are an assistant with access to the user's notes. Answer the question based ONLY on the provided context.\n\nContext:\n{}\n\nQuestion: {}\n\nAnswer:",
        context, query
    );

    let answer = call_ollama_summarize(&client, &prompt, &model).await?; // Reusing summarize helper for generic chat
    emit_log(&window, "synthesis", "complete", "Answer ready")?;

    Ok(FetchResult {
        success: true,
        title: format!("Chat: {}", query),
        summary: answer,
        file_path: "In Memory".to_string(),
        error: None,
    })
}

// ----------------------
// Agentic Loop (Autonomous)
// ----------------------

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[tauri::command]
async fn run_agent_loop(
    window: tauri::WebviewWindow,
    query: String,
    model: String,
    api_key: String,
) -> Result<FetchResult, String> {
    emit_log(&window, "discovery", "running", "Initializing Agent...")?;

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let mut history = vec![
        Message {
            role: "system".to_string(),
            content: format!(
                "You are an autonomous research agent. Your goal is to answer the user's question by searching and fetching information.\n\n\
                TOOLS:\n\
                - [TOOL: search(\"query\")] -> Returns a list of URLs and Titles from the web.\n\
                - [TOOL: fetch(\"url\")] -> Returns the content of a web URL.\n\
                - [TOOL: vault_search(\"query\")] -> Searches your local Obsidian notes.\n\
                - [TOOL: vault_read(\"filename\")] -> Reads the full content of a local note.\n\n\
                INSTRUCTIONS:\n\
                1. Analyze the user's request.\n\
                2. DECIDE if you need to search (web/vault) or fetch/read (web/vault).\n\
                3. Check your local vault before searching the web if appropriate.\n\
                4. OUTPUT the tool call in the format [TOOL: name(\"arg\")].\n\
                5. Wait for the result.\n\
                6. Repeat until you have enough info, then provide the Final Answer.\n\n\
                User Question: {}", 
                query
            ),
        }
    ];

    let mut final_answer = String::new();
    let max_turns = 5;

    for turn in 0..max_turns {
        emit_log(
            &window,
            "synthesis",
            "running",
            &format!("Agent thinking (Turn {}/{})", turn + 1, max_turns),
        )?;

        // 1. Get LLM Response
        let response = call_ollama_chat(&client, &history, &model).await?;
        history.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        // Log the agent's thought process (cleanly)
        let thought = response.split("[TOOL:").next().unwrap_or(&response).trim();
        if !thought.is_empty() {
            emit_log(
                &window,
                "synthesis",
                "running",
                &format!("Agent: {}", thought),
            )?;
        }

        // 2. Parse Tool Call
        if let Some(tool_call) = parse_tool_call(&response) {
            emit_log(
                &window,
                "acquisition",
                "running",
                &format!("Executing: {}", tool_call),
            )?;

            let result = if tool_call.to_lowercase().contains("search")
                && !tool_call.to_lowercase().contains("vault")
            {
                let q = extract_arg(&tool_call);
                match call_searxng_search(&client, &q).await {
                    Ok(urls) => {
                        let formatted = urls
                            .iter()
                            .map(|u| format!("- {} ({})", u.title, u.url))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("Web Search Results for '{}':\n{}", q, formatted)
                    }
                    Err(e) => format!("Error searching web: {}", e),
                }
            } else if tool_call.to_lowercase().contains("fetch") {
                let url = extract_arg(&tool_call);
                match fetch_url_content(&client, &url).await {
                    Ok(content) => format!(
                        "Content of {}:\n{}",
                        url,
                        content.chars().take(2000).collect::<String>()
                    ),
                    Err(e) => format!("Error fetching web {}: {}", url, e),
                }
            } else if tool_call.to_lowercase().contains("vault_search") {
                let q = extract_arg(&tool_call);
                match search_vault(q.clone(), api_key.clone()).await {
                    Ok(notes) => {
                        let formatted = notes
                            .iter()
                            .map(|n| format!("- {} (Filename: {})", n.title, n.url))
                            .collect::<Vec<_>>()
                            .join("\n");
                        format!("Vault Search Results for '{}':\n{}", q, formatted)
                    }
                    Err(e) => format!("Error searching vault: {}", e),
                }
            } else if tool_call.to_lowercase().contains("vault_read") {
                let filename = extract_arg(&tool_call);
                match fetch_vault_file(&client, &filename, &api_key).await {
                    Ok(content) => format!(
                        "Content of note {}:\n{}",
                        filename,
                        content.chars().take(3000).collect::<String>()
                    ),
                    Err(e) => format!("Error reading note {}: {}", filename, e),
                }
            } else {
                "Unknown tool".to_string()
            };

            history.push(Message {
                role: "user".to_string(),
                content: format!("Tool Result: {}", result),
            });
        } else {
            // No tool call -> Final Answer
            final_answer = response;
            emit_log(&window, "synthesis", "complete", "Agent finished task.")?;
            break;
        }
    }

    Ok(FetchResult {
        success: true,
        title: format!("Agent: {}", query),
        summary: final_answer,
        file_path: "In Memory".to_string(),
        error: None,
    })
}

fn parse_tool_call(response: &str) -> Option<String> {
    // Look for [TOOL: name("arg")] or [TOOL: name(arg)]
    let start_pattern = "[TOOL: ";
    if let Some(start) = response.find(start_pattern) {
        if let Some(end) = response[start..].find("]") {
            return Some(
                response[start + start_pattern.len()..start + end]
                    .trim()
                    .to_string(),
            );
        }
    }
    None
}

fn extract_arg(tool_call: &str) -> String {
    // Look for content inside parentheses, and strip optional quotes
    if let Some(start) = tool_call.find("(") {
        if let Some(end) = tool_call.rfind(")") {
            let arg = &tool_call[start + 1..end];
            // Strip leading/trailing quotes if they exist
            return arg
                .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                .to_string();
        }
    }
    tool_call.trim().to_string()
}

async fn call_ollama_chat(
    client: &reqwest::Client,
    messages: &[Message],
    model: &str,
) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": false
    });

    let res = client
        .post("http://localhost:11434/api/chat")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let txt = res.text().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&txt).map_err(|e| e.to_string())?;

    json["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("No content".to_string())
}

/// Simple HTML to plain text converter
fn html_to_text(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;

    for c in html.chars() {
        match c {
            '<' => {
                in_tag = true;
                let lower = html.to_lowercase();
                if lower.contains("<script") {
                    in_script = true;
                }
                if lower.contains("<style") {
                    in_style = true;
                }
            }
            '>' => {
                in_tag = false;
                if html.to_lowercase().contains("</script") {
                    in_script = false;
                }
                if html.to_lowercase().contains("</style") {
                    in_style = false;
                }
            }
            _ if !in_tag && !in_script && !in_style => {
                result.push(c);
            }
            _ => {}
        }
    }

    // Clean up whitespace
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Summarize using Ollama
async fn call_ollama_summarize(
    client: &reqwest::Client,
    content: &str,
    model: &str,
) -> Result<String, String> {
    let prompt = format!(
        r#"Summarize the following documentation into a Reference Note format.

Include:
- Executive Summary (2-3 sentences)
- Key Concepts (bullet points)
- Code Examples (if any, preserve them)

Documentation:
{}

Provide a clean, structured markdown summary:"#,
        content
    );

    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false
    });

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama request failed: {}. Is Ollama running?", e))?;

    // Get response text first for better error handling
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read Ollama response: {}", e))?;

    let ollama_response: OllamaResponse = serde_json::from_str(&response_text).map_err(|e| {
        format!(
            "Failed to parse Ollama JSON: {}. Response: {}",
            e,
            &response_text[..200.min(response_text.len())]
        )
    })?;

    Ok(ollama_response.response)
}

/// Save to Obsidian via Local REST API
async fn save_to_obsidian(
    client: &reqwest::Client,
    title: &str,
    content: &str,
    source_url: &str,
    source_title: &str,
    api_key: &str,
) -> Result<String, String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let safe_title = title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let file_path = format!("Reference/Docs/{}.md", safe_title);

    let note_content = format!(
        r#"---
tags: [reference, documentation, fetched]
source: {}
source_title: "{}"
fetched_date: {}
model: llama3.2
---

# {}

{}
"#,
        source_url, source_title, today, title, content
    );

    // Obsidian Local REST API endpoint
    let api_url = format!(
        "https://127.0.0.1:27124/vault/{}",
        urlencoding::encode(&file_path)
    );

    let response = client
        .put(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "text/markdown")
        .body(note_content)
        .send()
        .await
        .map_err(|e| {
            format!(
                "Obsidian API request failed: {}. Is Obsidian running with Local REST API?",
                e
            )
        })?;

    if !response.status().is_success() {
        return Err(format!(
            "Obsidian API returned error: {}",
            response.status()
        ));
    }

    Ok(file_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            perform_search,
            process_selection,
            search_vault,
            chat_with_vault,
            run_agent_loop
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn fetch_url_content(client: &reqwest::Client, url: &str) -> Result<String, String> {
    let response = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    let html = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Simple HTML to plain text conversion
    Ok(html_to_text(&html).chars().take(8000).collect())
}

async fn fetch_vault_file(
    client: &reqwest::Client,
    filename: &str,
    api_key: &str,
) -> Result<String, String> {
    let url = format!(
        "https://127.0.0.1:27124/vault/{}",
        urlencoding::encode(filename)
    );
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Vault fetch failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Vault error: {}", res.status()));
    }

    res.text()
        .await
        .map_err(|e| format!("Failed to read vault response: {}", e))
}
