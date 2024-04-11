// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn chat_with_openai(prompt: String, openai_key: String) -> Result<String, String> {
    use reqwest::Client;
    // use std::env;
    use serde_json::Value;

    let client = Client::new();
    let api_key = openai_key;
    //let api_key = env::var("OPENAI_API_KEY").map_err(|e| e.to_string())?;
    //println!("Updated api key - {}", api_key);

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

    return Ok("Invalid Api key, please check it and try again.".to_string());
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat_with_openai])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
