use std::env;

pub fn embed_sync(text: &str) -> Vec<f32> {
    text.bytes().map(|b| b as f32 / 255.0).collect()
}

pub struct LlmChain;

impl LlmChain {
    pub async fn chat(prompt: &str) -> String {
        chat_sync(prompt)
    }
}

pub fn chat_sync(prompt: &str) -> String {
    if let Ok(resp) = try_groq(prompt) {
        return resp;
    }

    if let Ok(resp) = try_ollama(prompt) {
        return resp;
    }

    format!("SPS fallback response: {}", prompt)
}

fn try_groq(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY")?;

    let body = serde_json::json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": 0.7
    })
    .to_string();

    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            "https://api.groq.com/openai/v1/chat/completions",
            "-H",
            &format!("Authorization: Bearer {}", api_key),
            "-H",
            "Content-Type: application/json",
            "-d",
            &body,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: serde_json::Value = serde_json::from_str(&stdout)?;

    Ok(json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

fn try_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "model": "llama3",
        "prompt": prompt,
        "stream": false
    })
    .to_string();

    let url = env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());

    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            &format!("{}/api/generate", url),
            "-H",
            "Content-Type: application/json",
            "-d",
            &body,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: serde_json::Value = serde_json::from_str(&stdout)?;

    Ok(json["response"].as_str().unwrap_or("").to_string())
}
