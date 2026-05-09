use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct VisionAgent;

impl VisionAgent {
    pub fn analyze_image(image_path: &str, prompt: &str, outbox: &mut EffectOutbox) {
        let b64 = match std::fs::read(image_path) {
            Ok(data) => base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data),
            Err(_) => {
                outbox.enqueue(EffectRequest {
                    id: 0,
                    tick: 0,
                    effect_type: EffectType::Subprocess,
                    payload: "echo 'Image not found'".into(),
                });
                return;
            }
        };
        let json = serde_json::json!({
            "model": "moondream:latest",
            "prompt": prompt,
            "images": [b64],
            "stream": false
        })
        .to_string();
        let cmd = format!("curl -s -X POST http://127.0.0.1:11434/api/generate -H 'Content-Type: application/json' -d '{}'", json.replace('\'', "'\\''"));
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
}
