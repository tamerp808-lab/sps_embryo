#![allow(dead_code)]
use crate::outbox::{EffectOutbox, EffectRequest, EffectType};
pub struct ExecutionEngine;
impl ExecutionEngine {
    pub fn execute_plan(goal: &str, outbox: &mut EffectOutbox) {
        let prompt = format!("You are a plan executor. Given the goal: \"{}\", produce a step-by-step plan in JSON array format [{{\"step\":1,\"task\":\"...\"}}] and then start executing the first task by outputting the code required. Output ONLY JSON.", goal);
        let effect = EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::LlmInference,
            payload: prompt,
        };
        outbox.enqueue(effect);
    }
}
