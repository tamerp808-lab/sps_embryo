use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct CoderAgent;

impl CoderAgent {
    pub fn write_code(task: &str, outbox: &mut EffectOutbox) {
        let prompt = format!(
            "Write Python code for: {}. Output ONLY raw Python code, no markdown.",
            task
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::LlmInference,
            payload: prompt,
        });
    }
    pub fn review(code: &str, outbox: &mut EffectOutbox) {
        let prompt = format!("Review this Python code. Output ONLY JSON: {{\"approve\":bool,\"issues\":[]}}\nCode:\n{}", code);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::LlmInference,
            payload: prompt,
        });
    }
    pub fn debug(code: &str, error: &str, outbox: &mut EffectOutbox) {
        let prompt = format!(
            "Fix this Python code. Error: {}\nCode:\n{}\nOutput corrected code only.",
            error, code
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::LlmInference,
            payload: prompt,
        });
    }
    pub fn run_code(code: &str, outbox: &mut EffectOutbox) {
        let tmp = format!("/tmp/sps_code_{}.py", uuid::Uuid::now_v7());
        let _ = std::fs::write(&tmp, code);
        let cmd = format!("python3 {} 2>&1; rm {}", tmp, tmp);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
}
