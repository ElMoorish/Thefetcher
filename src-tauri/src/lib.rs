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

/// Fetch URL content directly
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

    // Simple HTML to text conversion (strip tags)
    let text = html_to_text(&html);

    // Limit to ~8000 chars for Ollama context
    Ok(text.chars().take(8000).collect())
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
        .invoke_handler(tauri::generate_handler![perform_search, process_selection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
