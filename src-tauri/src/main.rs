// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn update_api_key(api_key: String) -> Result<String, String> {
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use dotenv::dotenv;
    use regex::Regex;

    println!("updating api key");
    dotenv().ok();
    let env_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()))
        .join(".env");

    let content = fs::read_to_string(&env_path).map_err(|e| e.to_string())?;

    let new_content = Regex::new(r"OPENAI_API_KEY=.*").unwrap()
    .replace(&content, format!("OPENAI_API_KEY={}", api_key))
    .to_string();

    fs::write(&env_path, new_content).map_err(|e| e.to_string())?;

    Ok("API key updated successfully.".into())
}

#[tauri::command]
async fn chat_with_openai(prompt: String) -> Result<String, String> {
    use reqwest::Client;
    use std::env;
    use serde_json::Value;

    let client = Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    let res: Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    if let Some(choices) = res["choices"].as_array() {
        if let Some(choice) = choices.first() {
            if let Some(content) = choice["message"]["content"].as_str() {
                return Ok(content.to_string());
            }
        }
    }

    Err("No content found in response".to_string())
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![update_api_key, chat_with_openai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
