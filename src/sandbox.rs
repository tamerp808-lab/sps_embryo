
use crate::outbox::{EffectRequest, EffectType};

pub fn execute_effect(effect: &EffectRequest) -> String {
    match effect.effect_type {
        EffectType::Subprocess => {
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&effect.payload)
                .output();
            match output {
                Ok(out) => {
                    if out.status.success() {
                        String::from_utf8_lossy(&out.stdout).into()
                    } else {
                        format!("Error: {}", String::from_utf8_lossy(&out.stderr))
                    }
                }
                Err(e) => format!("Command failed: {}", e),
            }
        }
        EffectType::ApiCall => {
            format!("API call to {} (simulated)", effect.payload)
        }
        EffectType::LlmInference => {
            // استدعاء LLM حقيقي
            crate::llm_chain::chat_sync(&effect.payload)
        }
    }
}

pub fn process_pending_effects(outbox: &mut crate::outbox::EffectOutbox) -> Vec<String> {
    let mut results = Vec::new();
    let pending_ids = outbox.pending_ids();
    for id in pending_ids {
        if let Some(effect) = outbox.complete(id) {
            let result = execute_effect(&effect);
            results.push(result);
        }
    }
    results
}

/// استدعاءات خارجية (للتوافق مع planner.rs القديم)
pub fn call_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(crate::llm_chain::chat_sync(prompt))
}

pub fn call_groq_via_curl(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(crate::llm_chain::chat_sync(prompt))
}
